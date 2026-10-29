use crate::errors::Result;
use crate::config::Config;
use crate::model::{
    AccountUpdateEvent, AggrTradesEvent, BookTickerEvent, ContinuousKlineEvent, DayTickerEvent,
    DepthOrderBookEvent, IndexKlineEvent, IndexPriceEvent, KlineEvent, LiquidationEvent,
    MarkPriceEvent, MiniTickerEvent, OrderBook, TradeEvent, UserDataStreamExpiredEvent,
};
use crate::futures::model;
use crate::futures::model::{
    AccountConfigUpdateEvent, AlgoUpdateEvent, MarginCallEvent, TradeLiteEvent,
};
use error_chain::bail;
use url::Url;
use serde::{Deserialize, Serialize};
use std::sync::atomic::{AtomicBool, Ordering};
use std::net::TcpStream;
use tungstenite::{connect, Message};
use tungstenite::protocol::WebSocket;
use tungstenite::stream::MaybeTlsStream;
use tungstenite::handshake::client::Response;

#[allow(clippy::all)]
enum FuturesWebsocketAPI {
    Default,
    MultiStream,
    Custom(String),
}

/// Routed WebSocket entry point. Since 2026 Binance only pushes a stream on the path it
/// belongs to: `/public` for book ticker and depth streams, `/market` for aggTrade, mark
/// price, klines, tickers, liquidations and composite index, `/private` for user data.
/// Un-routed connections only receive `/public` streams.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FuturesStreamRoute {
    /// Legacy un-routed base URL; only `/public` streams are delivered
    Root,
    Public,
    Market,
    Private,
}

impl FuturesStreamRoute {
    fn path(self) -> &'static str {
        match self {
            FuturesStreamRoute::Root => "",
            FuturesStreamRoute::Public => "/public",
            FuturesStreamRoute::Market => "/market",
            FuturesStreamRoute::Private => "/private",
        }
    }

    /// Pick the route a stream name belongs to.
    pub fn for_stream(stream: &str) -> FuturesStreamRoute {
        let name = stream.split('@').nth(1).unwrap_or("");
        if stream == "!bookTicker" || name.starts_with("bookTicker") || name.starts_with("depth") {
            FuturesStreamRoute::Public
        } else if stream.contains('@') || stream.starts_with('!') {
            FuturesStreamRoute::Market
        } else {
            // A bare listen key
            FuturesStreamRoute::Private
        }
    }
}

pub enum FuturesMarket {
    USDM,
    COINM,
    Vanilla,
    USDMTestnet,
    COINMTestnet,
    VanillaTestnet,
}

impl FuturesWebsocketAPI {
    fn params(
        self, market: &FuturesMarket, route: FuturesStreamRoute, subscription: &str,
    ) -> String {
        let baseurl = match market {
            FuturesMarket::USDM => "wss://fstream.binance.com",
            FuturesMarket::COINM => "wss://dstream.binance.com",
            FuturesMarket::Vanilla => "wss://vstream.binance.com",
            FuturesMarket::USDMTestnet => "wss://fstream.binancefuture.com",
            FuturesMarket::COINMTestnet => "wss://dstream.binancefuture.com",
            FuturesMarket::VanillaTestnet => "wss://vstream.binancefuture.com",
        };

        match self {
            FuturesWebsocketAPI::Default => {
                format!("{}{}/ws/{}", baseurl, route.path(), subscription)
            }
            FuturesWebsocketAPI::MultiStream => {
                format!(
                    "{}{}/stream?streams={}",
                    baseurl,
                    route.path(),
                    subscription
                )
            }
            FuturesWebsocketAPI::Custom(url) => url,
        }
    }
}

/// Build the stream URL for a market, route and subscription.
pub fn futures_stream_url(
    market: &FuturesMarket, route: FuturesStreamRoute, subscription: &str,
) -> String {
    FuturesWebsocketAPI::Default.params(market, route, subscription)
}

#[allow(clippy::large_enum_variant)]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum FuturesWebsocketEvent {
    AccountUpdate(AccountUpdateEvent),
    OrderTrade(model::OrderTradeEvent),
    /// `TRADE_LITE`: lightweight trade execution event
    TradeLite(TradeLiteEvent),
    /// `ALGO_UPDATE`: conditional order status change
    AlgoUpdate(AlgoUpdateEvent),
    /// `MARGIN_CALL`
    MarginCall(MarginCallEvent),
    /// `ACCOUNT_CONFIG_UPDATE`: leverage or multi-assets mode change
    AccountConfigUpdate(AccountConfigUpdateEvent),
    AggrTrades(AggrTradesEvent),
    Trade(TradeEvent),
    OrderBook(OrderBook),
    DayTicker(DayTickerEvent),
    MiniTicker(MiniTickerEvent),
    MiniTickerAll(Vec<MiniTickerEvent>),
    IndexPrice(IndexPriceEvent),
    MarkPrice(MarkPriceEvent),
    MarkPriceAll(Vec<MarkPriceEvent>),
    DayTickerAll(Vec<DayTickerEvent>),
    Kline(KlineEvent),
    ContinuousKline(ContinuousKlineEvent),
    IndexKline(IndexKlineEvent),
    Liquidation(LiquidationEvent),
    DepthOrderBook(DepthOrderBookEvent),
    BookTicker(BookTickerEvent),
    UserDataStreamExpiredEvent(UserDataStreamExpiredEvent),
}

pub struct FuturesWebSockets<'a> {
    pub socket: Option<(WebSocket<MaybeTlsStream<TcpStream>>, Response)>,
    handler: Box<dyn FnMut(FuturesWebsocketEvent) -> Result<()> + 'a>,
}

#[allow(clippy::large_enum_variant)]
#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
enum FuturesEvents {
    Vec(Vec<DayTickerEvent>),
    DayTickerEvent(DayTickerEvent),
    BookTickerEvent(BookTickerEvent),
    MiniTickerEvent(MiniTickerEvent),
    VecMiniTickerEvent(Vec<MiniTickerEvent>),
    AccountUpdateEvent(AccountUpdateEvent),
    OrderTradeEvent(model::OrderTradeEvent),
    AggrTradesEvent(AggrTradesEvent),
    IndexPriceEvent(IndexPriceEvent),
    MarkPriceEvent(MarkPriceEvent),
    VecMarkPriceEvent(Vec<MarkPriceEvent>),
    TradeEvent(TradeEvent),
    KlineEvent(KlineEvent),
    ContinuousKlineEvent(ContinuousKlineEvent),
    IndexKlineEvent(IndexKlineEvent),
    LiquidationEvent(LiquidationEvent),
    OrderBook(OrderBook),
    DepthOrderBookEvent(DepthOrderBookEvent),
    UserDataStreamExpiredEvent(UserDataStreamExpiredEvent),
}

impl<'a> FuturesWebSockets<'a> {
    pub fn new<Callback>(handler: Callback) -> FuturesWebSockets<'a>
    where
        Callback: FnMut(FuturesWebsocketEvent) -> Result<()> + 'a,
    {
        FuturesWebSockets {
            socket: None,
            handler: Box::new(handler),
        }
    }

    /// Connect to a single stream (or a listen key), picking the routed entry point
    /// (`/public`, `/market` or `/private`) from the stream name.
    pub fn connect(&mut self, market: &FuturesMarket, subscription: &'a str) -> Result<()> {
        let route = Self::default_route(market, subscription);
        self.connect_with_route(market, route, subscription)
    }

    /// Routed paths only exist on USDⓈ-M futures; other markets keep the legacy base URL.
    fn default_route(market: &FuturesMarket, subscription: &str) -> FuturesStreamRoute {
        match market {
            FuturesMarket::USDM | FuturesMarket::USDMTestnet => {
                FuturesStreamRoute::for_stream(subscription)
            }
            _ => FuturesStreamRoute::Root,
        }
    }

    /// Connect to a single stream on an explicit route.
    pub fn connect_with_route(
        &mut self, market: &FuturesMarket, route: FuturesStreamRoute, subscription: &str,
    ) -> Result<()> {
        self.connect_wss(&FuturesWebsocketAPI::Default.params(market, route, subscription))
    }

    /// Connect to `config.futures_ws_endpoint`; the endpoint is used verbatim, so include the
    /// route and `/ws/<stream>` part in it.
    pub fn connect_with_config(
        &mut self, market: &FuturesMarket, subscription: &'a str, config: &'a Config,
    ) -> Result<()> {
        self.connect_wss(
            &FuturesWebsocketAPI::Custom(config.futures_ws_endpoint.clone()).params(
                market,
                FuturesStreamRoute::Root,
                subscription,
            ),
        )
    }

    /// Connect to several streams at once. All streams must belong to the same route; the
    /// route is picked from the first stream.
    pub fn connect_multiple_streams(
        &mut self, market: &FuturesMarket, endpoints: &[String],
    ) -> Result<()> {
        let route = endpoints
            .first()
            .map(|s| Self::default_route(market, s))
            .unwrap_or(FuturesStreamRoute::Root);
        self.connect_multiple_streams_with_route(market, route, endpoints)
    }

    /// Connect to several streams at once on an explicit route.
    pub fn connect_multiple_streams_with_route(
        &mut self, market: &FuturesMarket, route: FuturesStreamRoute, endpoints: &[String],
    ) -> Result<()> {
        self.connect_wss(&FuturesWebsocketAPI::MultiStream.params(
            market,
            route,
            &endpoints.join("/"),
        ))
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

        if let Some(data) = value.get("data") {
            self.handle_msg(&data.to_string())?;
            return Ok(());
        }

        // User data events sharing a shape are dispatched by name first.
        if let Some(event_type) = value.get("e").and_then(|e| e.as_str()) {
            let action = match event_type {
                "ORDER_TRADE_UPDATE" => Some(FuturesWebsocketEvent::OrderTrade(
                    serde_json::from_value(value.clone())?,
                )),
                "ACCOUNT_UPDATE" => Some(FuturesWebsocketEvent::AccountUpdate(
                    serde_json::from_value(value.clone())?,
                )),
                "TRADE_LITE" => Some(FuturesWebsocketEvent::TradeLite(serde_json::from_value(
                    value.clone(),
                )?)),
                "ALGO_UPDATE" => Some(FuturesWebsocketEvent::AlgoUpdate(serde_json::from_value(
                    value.clone(),
                )?)),
                "MARGIN_CALL" => Some(FuturesWebsocketEvent::MarginCall(serde_json::from_value(
                    value.clone(),
                )?)),
                "ACCOUNT_CONFIG_UPDATE" => Some(FuturesWebsocketEvent::AccountConfigUpdate(
                    serde_json::from_value(value.clone())?,
                )),
                "listenKeyExpired" => Some(FuturesWebsocketEvent::UserDataStreamExpiredEvent(
                    serde_json::from_value(value.clone())?,
                )),
                // markPriceUpdate carries the index price too, so an untagged match would
                // classify it as an IndexPriceEvent.
                "markPriceUpdate" => Some(FuturesWebsocketEvent::MarkPrice(
                    serde_json::from_value(value.clone())?,
                )),
                "indexPriceUpdate" => Some(FuturesWebsocketEvent::IndexPrice(
                    serde_json::from_value(value.clone())?,
                )),
                _ => None,
            };
            if let Some(action) = action {
                (self.handler)(action)?;
                return Ok(());
            }
        }

        if let Ok(events) = serde_json::from_value::<FuturesEvents>(value) {
            let action = match events {
                FuturesEvents::Vec(v) => FuturesWebsocketEvent::DayTickerAll(v),
                FuturesEvents::DayTickerEvent(v) => FuturesWebsocketEvent::DayTicker(v),
                FuturesEvents::BookTickerEvent(v) => FuturesWebsocketEvent::BookTicker(v),
                FuturesEvents::MiniTickerEvent(v) => FuturesWebsocketEvent::MiniTicker(v),
                FuturesEvents::VecMiniTickerEvent(v) => FuturesWebsocketEvent::MiniTickerAll(v),
                FuturesEvents::AccountUpdateEvent(v) => FuturesWebsocketEvent::AccountUpdate(v),
                FuturesEvents::OrderTradeEvent(v) => FuturesWebsocketEvent::OrderTrade(v),
                FuturesEvents::IndexPriceEvent(v) => FuturesWebsocketEvent::IndexPrice(v),
                FuturesEvents::MarkPriceEvent(v) => FuturesWebsocketEvent::MarkPrice(v),
                FuturesEvents::VecMarkPriceEvent(v) => FuturesWebsocketEvent::MarkPriceAll(v),
                FuturesEvents::TradeEvent(v) => FuturesWebsocketEvent::Trade(v),
                FuturesEvents::ContinuousKlineEvent(v) => FuturesWebsocketEvent::ContinuousKline(v),
                FuturesEvents::IndexKlineEvent(v) => FuturesWebsocketEvent::IndexKline(v),
                FuturesEvents::LiquidationEvent(v) => FuturesWebsocketEvent::Liquidation(v),
                FuturesEvents::KlineEvent(v) => FuturesWebsocketEvent::Kline(v),
                FuturesEvents::OrderBook(v) => FuturesWebsocketEvent::OrderBook(v),
                FuturesEvents::DepthOrderBookEvent(v) => FuturesWebsocketEvent::DepthOrderBook(v),
                FuturesEvents::AggrTradesEvent(v) => FuturesWebsocketEvent::AggrTrades(v),
                FuturesEvents::UserDataStreamExpiredEvent(v) => {
                    FuturesWebsocketEvent::UserDataStreamExpiredEvent(v)
                }
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
        bail!("running loop closed");
    }
}
