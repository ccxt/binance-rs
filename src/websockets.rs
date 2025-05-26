use crate::errors::Result;
use crate::config::Config;
use crate::model::{
    AccountUpdateEvent, AggrTradesEvent, BalanceUpdateEvent, BookTickerEvent, DayTickerEvent,
    WindowTickerEvent, DepthOrderBookEvent, KlineEvent, OrderBook, OrderTradeEvent, TradeEvent,
};
use url::Url;
use serde::{Deserialize, Serialize};

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
    OrderTrade(OrderTradeEvent),
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

pub struct WebSockets<'a> {
    pub socket: Option<(WebSocket<MaybeTlsStream<TcpStream>>, Response)>,
    handler: Box<dyn FnMut(WebsocketEvent) -> Result<()> + 'a>,
}

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

    fn connect_wss(&mut self, wss: &str) -> Result<()> {
        let url = Url::parse(wss)?;
        match connect(url) {
            Ok(answer) => {
                self.socket = Some(answer);
                Ok(())
            }
            // Err(e) => bail!(format!("Error during handshake {}", e)),
            Err(e) => Err(format!("Error during handshake {}", e).into()),
        }
    }

    pub fn disconnect(&mut self) -> Result<()> {
        if let Some(ref mut socket) = self.socket {
            socket.0.close(None)?;
            return Ok(());
        }
        // bail!("Not able to close the connection");
        Err("Not able to close the connection".to_string().into())
    }

    pub fn test_handle_msg(&mut self, msg: &str) -> Result<()> {
        self.handle_msg(msg)
    }

    pub fn handle_msg(&mut self, msg: &str) -> Result<()> {
        let value: serde_json::Value = serde_json::from_str(msg)?;

        if let Some(data) = value.get("data") {
            self.handle_msg(&data.to_string())?;
            return Ok(());
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
                let message = socket.0.read()?;
                match message {
                    Message::Text(msg) => {
                        if let Err(e) = self.handle_msg(&msg) {
                            // bail!(format!("Error on handling stream message: {}", e));
                            return Err(format!("Error on handling stream message: {}", e).into());
                        }
                    }
                    Message::Ping(payload) => {
                        socket.0.send(Message::Pong(payload)).unwrap();
                    }
                    Message::Pong(_) | Message::Binary(_) | Message::Frame(_) => (),
                    Message::Close(e) => {
                        return Err(format!("Disconnected from server: {:?}", e).into());
                    }
                }
            }
        }
        Ok(())
    }
}
