use crate::errors::Result;
use crate::config::Config;
use crate::model::{
    AccountUpdateEvent, AggrTradesEvent, BalanceUpdateEvent, BookTickerEvent, DayTickerEvent,
    WindowTickerEvent, DepthOrderBookEvent, ExternalLockUpdateEvent, GenericEvent, KlineEvent,
    ListStatusEvent, OrderBook, OrderTradeEvent, OutboundAccountPositionEvent,
    SpotBalanceUpdateEvent, TradeEvent,
};
use crate::util::{get_timestamp_now, sign_hmac};
use error_chain::bail;
use url::Url;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

use std::sync::atomic::{AtomicBool, Ordering};
use std::net::TcpStream;
use tungstenite::{connect, Message};
use tungstenite::protocol::WebSocket;
use tungstenite::stream::MaybeTlsStream;
use tungstenite::handshake::client::Response;

#[allow(clippy::all)]
enum WebsocketAPI {
    Default,
    MultiStream,
    Custom(String),
}

impl WebsocketAPI {
    fn params(self, subscription: &str) -> String {
        match self {
            WebsocketAPI::Default => format!("wss://stream.binance.com/ws/{}", subscription),
            WebsocketAPI::MultiStream => {
                format!("wss://stream.binance.com/stream?streams={}", subscription)
            }
            WebsocketAPI::Custom(url) => format!("{}/{}", url, subscription),
        }
    }
}

#[allow(clippy::large_enum_variant)]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum WebsocketEvent {
    AccountUpdate(AccountUpdateEvent),
    BalanceUpdate(BalanceUpdateEvent),
    /// `executionReport`: order placed, canceled, filled, expired, amended
    OrderTrade(OrderTradeEvent),
    /// `outboundAccountPosition`: balances changed
    OutboundAccountPosition(OutboundAccountPositionEvent),
    /// `balanceUpdate`: deposit, withdrawal or transfer
    SpotBalanceUpdate(SpotBalanceUpdateEvent),
    /// `listStatus`: order list (OCO/OTO/...) status change
    ListStatus(ListStatusEvent),
    /// `externalLockUpdate`: part of the balance locked/unlocked by an external system
    ExternalLockUpdate(ExternalLockUpdateEvent),
    /// `eventStreamTerminated`: the user data stream subscription was closed
    EventStreamTerminated(GenericEvent),
    /// `listenKeyExpired`: legacy listen key based stream expired
    ListenKeyExpired(GenericEvent),
    /// `serverShutdown`: the server will close this connection in about 10 minutes
    ServerShutdown(GenericEvent),
    AggrTrades(AggrTradesEvent),
    Trade(TradeEvent),
    OrderBook(OrderBook),
    DayTicker(DayTickerEvent),
    DayTickerAll(Vec<DayTickerEvent>),
    WindowTicker(WindowTickerEvent),
    WindowTickerAll(Vec<WindowTickerEvent>),
    Kline(KlineEvent),
    DepthOrderBook(DepthOrderBookEvent),
    BookTicker(BookTickerEvent),
}

/// Build the `userDataStream.subscribe.signature` request of the Spot WebSocket API.
///
/// The signature payload is the alphabetically sorted `key=value&...` string of all
/// params (`apiKey`, optional `recvWindow`, `timestamp`), signed with HMAC-SHA256.
pub fn user_data_stream_subscribe_request(
    api_key: &str, secret_key: &str, recv_window: u64, timestamp: u64, id: &str,
) -> String {
    let mut params: BTreeMap<String, String> = BTreeMap::new();
    params.insert("apiKey".into(), api_key.to_string());
    if recv_window > 0 {
        params.insert("recvWindow".into(), recv_window.to_string());
    }
    params.insert("timestamp".into(), timestamp.to_string());
    let payload = params
        .iter()
        .map(|(k, v)| format!("{}={}", k, v))
        .collect::<Vec<_>>()
        .join("&");
    let signature = sign_hmac(secret_key, &payload);

    let mut json_params = serde_json::Map::new();
    json_params.insert(
        "apiKey".into(),
        serde_json::Value::String(api_key.to_string()),
    );
    if recv_window > 0 {
        json_params.insert("recvWindow".into(), serde_json::Value::from(recv_window));
    }
    json_params.insert("timestamp".into(), serde_json::Value::from(timestamp));
    json_params.insert("signature".into(), serde_json::Value::String(signature));

    serde_json::json!({
        "id": id,
        "method": "userDataStream.subscribe.signature",
        "params": json_params,
    })
    .to_string()
}

pub struct WebSockets<'a> {
    pub socket: Option<(WebSocket<MaybeTlsStream<TcpStream>>, Response)>,
    handler: Box<dyn FnMut(WebsocketEvent) -> Result<()> + 'a>,
}

#[allow(clippy::large_enum_variant)]
#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
enum Events {
    DayTickerEventAll(Vec<DayTickerEvent>),
    WindowTickerEventAll(Vec<WindowTickerEvent>),
    BalanceUpdateEvent(BalanceUpdateEvent),
    DayTickerEvent(DayTickerEvent),
    WindowTickerEvent(WindowTickerEvent),
    BookTickerEvent(BookTickerEvent),
    AccountUpdateEvent(AccountUpdateEvent),
    OrderTradeEvent(OrderTradeEvent),
    AggrTradesEvent(AggrTradesEvent),
    TradeEvent(TradeEvent),
    KlineEvent(KlineEvent),
    OrderBook(OrderBook),
    DepthOrderBookEvent(DepthOrderBookEvent),
}

impl<'a> WebSockets<'a> {
    pub fn new<Callback>(handler: Callback) -> WebSockets<'a>
    where
        Callback: FnMut(WebsocketEvent) -> Result<()> + 'a,
    {
        WebSockets {
            socket: None,
            handler: Box::new(handler),
        }
    }

    pub fn connect(&mut self, subscription: &str) -> Result<()> {
        self.connect_wss(&WebsocketAPI::Default.params(subscription))
    }

    pub fn connect_with_config(&mut self, subscription: &str, config: &Config) -> Result<()> {
        self.connect_wss(&WebsocketAPI::Custom(config.ws_endpoint.clone()).params(subscription))
    }

    pub fn connect_multiple_streams(&mut self, endpoints: &[String]) -> Result<()> {
        self.connect_wss(&WebsocketAPI::MultiStream.params(&endpoints.join("/")))
    }

    /// Subscribe to the account's user data stream through the Spot WebSocket API
    /// (`userDataStream.subscribe.signature`).
    ///
    /// This replaces the `listenKey` based `wss://stream.binance.com/ws/<listenKey>` flow,
    /// which Binance retired on 2026-02-20. The connection goes to `config.ws_api_endpoint`
    /// and works with HMAC API keys; no prior `session.logon` is required.
    ///
    /// Events arrive wrapped as `{"subscriptionId": n, "event": {...}}` and are unwrapped
    /// by [`WebSockets::handle_msg`]. Run [`WebSockets::event_loop`] as usual afterwards.
    pub fn connect_user_data_stream(
        &mut self, api_key: &str, secret_key: &str, config: &Config,
    ) -> Result<()> {
        self.connect_wss(&config.ws_api_endpoint)?;
        let timestamp = get_timestamp_now()?;
        let id = uuid::Uuid::new_v4().to_string();
        let request = user_data_stream_subscribe_request(
            api_key,
            secret_key,
            config.recv_window,
            timestamp,
            &id,
        );
        if let Some(ref mut socket) = self.socket {
            socket.0.write_message(Message::Text(request))?;
            return Ok(());
        }
        bail!("Not connected");
    }

    fn connect_wss(&mut self, wss: &str) -> Result<()> {
        let url = Url::parse(wss)?;
        match connect(url) {
            Ok(answer) => {
                self.socket = Some(answer);
                Ok(())
            }
            Err(e) => bail!(format!("Error during handshake {}", e)),
        }
    }

    pub fn disconnect(&mut self) -> Result<()> {
        if let Some(ref mut socket) = self.socket {
            socket.0.close(None)?;
            return Ok(());
        }
        bail!("Not able to close the connection");
    }

    pub fn test_handle_msg(&mut self, msg: &str) -> Result<()> {
        self.handle_msg(msg)
    }

    pub fn handle_msg(&mut self, msg: &str) -> Result<()> {
        let value: serde_json::Value = serde_json::from_str(msg)?;

        // Combined stream payload: {"stream": "...", "data": {...}}
        if let Some(data) = value.get("data") {
            self.handle_msg(&data.to_string())?;
            return Ok(());
        }

        // WebSocket API user data stream payload: {"subscriptionId": 0, "event": {...}}
        if let Some(event) = value.get("event") {
            self.handle_msg(&event.to_string())?;
            return Ok(());
        }

        // WebSocket API response to one of our requests: {"id": ..., "status": 200, ...}
        if let (Some(id), Some(status)) = (value.get("id"), value.get("status")) {
            if status.as_u64() != Some(200) {
                bail!(format!(
                    "WebSocket API request {} failed: {}",
                    id,
                    value.get("error").cloned().unwrap_or_default()
                ));
            }
            return Ok(());
        }

        // User data stream events are dispatched by name: several of them share the same
        // shape, which an untagged enum cannot tell apart.
        if let Some(event_type) = value.get("e").and_then(|e| e.as_str()) {
            let action = match event_type {
                "executionReport" => Some(WebsocketEvent::OrderTrade(serde_json::from_value(
                    value.clone(),
                )?)),
                "outboundAccountPosition" => Some(WebsocketEvent::OutboundAccountPosition(
                    serde_json::from_value(value.clone())?,
                )),
                "balanceUpdate" => Some(WebsocketEvent::SpotBalanceUpdate(serde_json::from_value(
                    value.clone(),
                )?)),
                "listStatus" => Some(WebsocketEvent::ListStatus(serde_json::from_value(
                    value.clone(),
                )?)),
                "externalLockUpdate" => Some(WebsocketEvent::ExternalLockUpdate(
                    serde_json::from_value(value.clone())?,
                )),
                "eventStreamTerminated" => Some(WebsocketEvent::EventStreamTerminated(
                    serde_json::from_value(value.clone())?,
                )),
                "listenKeyExpired" => Some(WebsocketEvent::ListenKeyExpired(
                    serde_json::from_value(value.clone())?,
                )),
                "serverShutdown" => Some(WebsocketEvent::ServerShutdown(serde_json::from_value(
                    value.clone(),
                )?)),
                _ => None,
            };
            if let Some(action) = action {
                (self.handler)(action)?;
                return Ok(());
            }
        }

        if let Ok(events) = serde_json::from_value::<Events>(value) {
            let action = match events {
                Events::DayTickerEventAll(v) => WebsocketEvent::DayTickerAll(v),
                Events::WindowTickerEventAll(v) => WebsocketEvent::WindowTickerAll(v),
                Events::BookTickerEvent(v) => WebsocketEvent::BookTicker(v),
                Events::BalanceUpdateEvent(v) => WebsocketEvent::BalanceUpdate(v),
                Events::AccountUpdateEvent(v) => WebsocketEvent::AccountUpdate(v),
                Events::OrderTradeEvent(v) => WebsocketEvent::OrderTrade(v),
                Events::AggrTradesEvent(v) => WebsocketEvent::AggrTrades(v),
                Events::TradeEvent(v) => WebsocketEvent::Trade(v),
                Events::DayTickerEvent(v) => WebsocketEvent::DayTicker(v),
                Events::WindowTickerEvent(v) => WebsocketEvent::WindowTicker(v),
                Events::KlineEvent(v) => WebsocketEvent::Kline(v),
                Events::OrderBook(v) => WebsocketEvent::OrderBook(v),
                Events::DepthOrderBookEvent(v) => WebsocketEvent::DepthOrderBook(v),
            };
            (self.handler)(action)?;
        }
        Ok(())
    }

    pub fn event_loop(&mut self, running: &AtomicBool) -> Result<()> {
        while running.load(Ordering::Relaxed) {
            if let Some(ref mut socket) = self.socket {
                let message = socket.0.read_message()?;
                match message {
                    Message::Text(msg) => {
                        if let Err(e) = self.handle_msg(&msg) {
                            bail!(format!("Error on handling stream message: {}", e));
                        }
                    }
                    Message::Ping(payload) => {
                        socket.0.write_message(Message::Pong(payload)).unwrap();
                    }
                    Message::Pong(_) | Message::Binary(_) | Message::Frame(_) => (),
                    Message::Close(e) => bail!(format!("Disconnected {:?}", e)),
                }
            }
        }
        Ok(())
    }
}
