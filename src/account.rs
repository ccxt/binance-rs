use crate::util::{build_signed_request, is_start_time_valid};
use crate::model::{
    AccountInformation, Balance, Empty, Order, OrderCanceled, TradeHistory, Transaction,
};
use crate::client::Client;
use crate::errors::{Result, SdkError};
use std::collections::BTreeMap;
use std::fmt::Display;
use crate::api::API;
use crate::api::Spot;

#[derive(Clone)]
pub struct Account {
    pub client: Client,
    pub recv_window: u64,
}

struct OrderRequest {
    pub symbol: String,
    pub qty: f64,
    pub price: f64,
    pub stop_price: Option<f64>,
    pub order_side: OrderSide,
    pub order_type: OrderType,
    pub time_in_force: TimeInForce,
    pub new_client_order_id: Option<String>,
}

struct OrderQuoteQuantityRequest {
    pub symbol: String,
    pub quote_order_qty: f64,
    pub price: f64,
    pub order_side: OrderSide,
    pub order_type: OrderType,
    pub time_in_force: TimeInForce,
    pub new_client_order_id: Option<String>,
}

pub enum OrderType {
    Limit,
    Market,
    StopLossLimit,
}

impl OrderType {
    pub fn from_int(value: i32) -> Option<Self> {
        match value {
            1 => Some(OrderType::Limit),
            2 => Some(OrderType::Market),
            3 => Some(OrderType::StopLossLimit),
            _ => None,
        }
    }
}

impl Display for OrderType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Limit => write!(f, "LIMIT"),
            Self::Market => write!(f, "MARKET"),
            Self::StopLossLimit => write!(f, "STOP_LOSS_LIMIT"),
        }
    }
}

pub enum OrderSide {
    Buy,
    Sell,
}

impl OrderSide {
    pub fn from_int(value: i32) -> Option<Self> {
        match value {
            1 => Some(OrderSide::Buy),
            2 => Some(OrderSide::Sell),
            _ => None,
        }
    }
}

impl Display for OrderSide {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Buy => write!(f, "BUY"),
            Self::Sell => write!(f, "SELL"),
        }
    }
}

#[allow(clippy::all)]
pub enum TimeInForce {
    GTC,
    IOC,
    FOK,
}

impl TimeInForce {
    pub fn from_int(value: i32) -> Option<Self> {
        match value {
            1 => Some(TimeInForce::GTC),
            2 => Some(TimeInForce::IOC),
            3 => Some(TimeInForce::FOK),
            _ => None,
        }
    }
}

impl Display for TimeInForce {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::GTC => write!(f, "GTC"),
            Self::IOC => write!(f, "IOC"),
            Self::FOK => write!(f, "FOK"),
        }
    }
}

impl Account {
    // Account Information
    pub fn get_account(&self) -> Result<AccountInformation> {
        let request = build_signed_request(BTreeMap::new(), self.recv_window)?;
        self.client
            .get_signed(API::Spot(Spot::Account), Some(request))
    }

    // Balance for a single Asset
    pub fn get_balance<S>(&self, asset: S) -> Result<Balance>
    where
        S: Into<String>,
    {
        match self.get_account() {
            Ok(account) => {
                let cmp_asset = asset.into();
                for balance in account.balances {
                    if balance.asset == cmp_asset {
                        return Ok(balance);
                    }
                }
                Err("Asset not found".to_string().into())
            }
            Err(e) => Err(e),
        }
    }

    // Current open orders for ONE symbol
    pub fn get_open_orders<S>(&self, symbol: S) -> Result<Vec<Order>>
    where
        S: Into<String>,
    {
        let mut parameters: BTreeMap<String, String> = BTreeMap::new();
        parameters.insert("symbol".into(), symbol.into());

        let request = build_signed_request(parameters, self.recv_window)?;
        self.client
            .get_signed(API::Spot(Spot::OpenOrders), Some(request))
    }

    // All current open orders
    pub fn get_all_open_orders(&self) -> Result<Vec<Order>> {
        let parameters: BTreeMap<String, String> = BTreeMap::new();

        let request = build_signed_request(parameters, self.recv_window)?;
        self.client
            .get_signed(API::Spot(Spot::OpenOrders), Some(request))
    }

    // Cancel all open orders for a single symbol
    pub fn cancel_all_open_orders<S>(&self, symbol: S) -> Result<Vec<OrderCanceled>>
    where
        S: Into<String>,
    {
        let mut parameters: BTreeMap<String, String> = BTreeMap::new();
        parameters.insert("symbol".into(), symbol.into());
        let request = build_signed_request(parameters, self.recv_window)?;
        self.client
            .delete_signed(API::Spot(Spot::OpenOrders), Some(request))
    }

    // Check an order's status
    pub fn order_status<S>(&self, symbol: S, order_id: u64) -> Result<Order>
    where
        S: Into<String>,
    {
        let mut parameters: BTreeMap<String, String> = BTreeMap::new();
        parameters.insert("symbol".into(), symbol.into());
        parameters.insert("orderId".into(), order_id.to_string());

        let request = build_signed_request(parameters, self.recv_window)?;
        self.client
            .get_signed(API::Spot(Spot::Order), Some(request))
    }

    /// Place a test status order
    ///
    /// This order is sandboxed: it is validated, but not sent to the matching engine.
    pub fn test_order_status<S>(&self, symbol: S, order_id: u64) -> Result<()>
    where
        S: Into<String>,
    {
        let mut parameters: BTreeMap<String, String> = BTreeMap::new();
        parameters.insert("symbol".into(), symbol.into());
        parameters.insert("orderId".into(), order_id.to_string());

        let request = build_signed_request(parameters, self.recv_window)?;
        self.client
            .get_signed::<Empty>(API::Spot(Spot::OrderTest), Some(request))
            .map(|_| ())
    }

    // Place a LIMIT order - BUY
    pub fn limit_buy<S, F>(&self, symbol: S, qty: F, price: f64) -> Result<Transaction>
    where
        S: Into<String>,
        F: Into<f64>,
    {
        let buy = OrderRequest {
            symbol: symbol.into(),
            qty: qty.into(),
            price,
            stop_price: None,
            order_side: OrderSide::Buy,
            order_type: OrderType::Limit,
            time_in_force: TimeInForce::GTC,
            new_client_order_id: None,
        };
        let order = self.build_order(buy);
        let request = build_signed_request(order, self.recv_window)?;
        self.client.post_signed(API::Spot(Spot::Order), request)
    }

    /// Place a test limit order - BUY
    ///
    /// This order is sandboxed: it is validated, but not sent to the matching engine.
    pub fn test_limit_buy<S, F>(&self, symbol: S, qty: F, price: f64) -> Result<()>
    where
        S: Into<String>,
        F: Into<f64>,
    {
        let buy = OrderRequest {
            symbol: symbol.into(),
            qty: qty.into(),
            price,
            stop_price: None,
            order_side: OrderSide::Buy,
            order_type: OrderType::Limit,
            time_in_force: TimeInForce::GTC,
            new_client_order_id: None,
        };
        let order = self.build_order(buy);
        let request = build_signed_request(order, self.recv_window)?;
        self.client
            .post_signed::<Empty>(API::Spot(Spot::OrderTest), request)
            .map(|_| ())
    }

    // Place a LIMIT order - SELL
    pub fn limit_sell<S, F>(&self, symbol: S, qty: F, price: f64) -> Result<Transaction>
    where
        S: Into<String>,
        F: Into<f64>,
    {
        let sell = OrderRequest {
            symbol: symbol.into(),
            qty: qty.into(),
            price,
            stop_price: None,
            order_side: OrderSide::Sell,
            order_type: OrderType::Limit,
            time_in_force: TimeInForce::GTC,
            new_client_order_id: None,
        };
        let order = self.build_order(sell);
        let request = build_signed_request(order, self.recv_window)?;
        self.client.post_signed(API::Spot(Spot::Order), request)
    }

    /// Place a test LIMIT order - SELL
    ///
    /// This order is sandboxed: it is validated, but not sent to the matching engine.
    pub fn test_limit_sell<S, F>(&self, symbol: S, qty: F, price: f64) -> Result<()>
    where
        S: Into<String>,
        F: Into<f64>,
    {
        let sell = OrderRequest {
            symbol: symbol.into(),
            qty: qty.into(),
            price,
            stop_price: None,
            order_side: OrderSide::Sell,
            order_type: OrderType::Limit,
            time_in_force: TimeInForce::GTC,
            new_client_order_id: None,
        };
        let order = self.build_order(sell);
        let request = build_signed_request(order, self.recv_window)?;
        self.client
            .post_signed::<Empty>(API::Spot(Spot::OrderTest), request)
            .map(|_| ())
    }

    // Place a MARKET order - BUY
    pub fn market_buy<S, F>(&self, symbol: S, qty: F) -> Result<Transaction>
    where
        S: Into<String>,
        F: Into<f64>,
    {
        let buy = OrderRequest {
            symbol: symbol.into(),
            qty: qty.into(),
            price: 0.0,
            stop_price: None,
            order_side: OrderSide::Buy,
            order_type: OrderType::Market,
            time_in_force: TimeInForce::GTC,
            new_client_order_id: None,
        };
        let order = self.build_order(buy);
        let request = build_signed_request(order, self.recv_window)?;
        self.client.post_signed(API::Spot(Spot::Order), request)
    }

    /// Place a test MARKET order - BUY
    ///
    /// This order is sandboxed: it is validated, but not sent to the matching engine.
    pub fn test_market_buy<S, F>(&self, symbol: S, qty: F) -> Result<()>
    where
        S: Into<String>,
        F: Into<f64>,
    {
        let buy = OrderRequest {
            symbol: symbol.into(),
            qty: qty.into(),
            price: 0.0,
            stop_price: None,
            order_side: OrderSide::Buy,
            order_type: OrderType::Market,
            time_in_force: TimeInForce::GTC,
            new_client_order_id: None,
        };
        let order = self.build_order(buy);
        let request = build_signed_request(order, self.recv_window)?;
        self.client
            .post_signed::<Empty>(API::Spot(Spot::OrderTest), request)
            .map(|_| ())
    }

    // Place a MARKET order with quote quantity - BUY
    pub fn market_buy_using_quote_quantity<S, F>(
        &self, symbol: S, quote_order_qty: F,
    ) -> Result<Transaction>
    where
        S: Into<String>,
        F: Into<f64>,
    {
        let buy = OrderQuoteQuantityRequest {
            symbol: symbol.into(),
            quote_order_qty: quote_order_qty.into(),
            price: 0.0,
            order_side: OrderSide::Buy,
            order_type: OrderType::Market,
            time_in_force: TimeInForce::GTC,
            new_client_order_id: None,
        };
        let order = self.build_quote_quantity_order(buy);
        let request = build_signed_request(order, self.recv_window)?;
        self.client.post_signed(API::Spot(Spot::Order), request)
    }

    /// Place a test MARKET order with quote quantity - BUY
    ///
    /// This order is sandboxed: it is validated, but not sent to the matching engine.
    pub fn test_market_buy_using_quote_quantity<S, F>(
        &self, symbol: S, quote_order_qty: F,
    ) -> Result<()>
    where
        S: Into<String>,
        F: Into<f64>,
    {
        let buy = OrderQuoteQuantityRequest {
            symbol: symbol.into(),
            quote_order_qty: quote_order_qty.into(),
            price: 0.0,
            order_side: OrderSide::Buy,
            order_type: OrderType::Market,
            time_in_force: TimeInForce::GTC,
            new_client_order_id: None,
        };
        let order = self.build_quote_quantity_order(buy);
        let request = build_signed_request(order, self.recv_window)?;
        self.client
            .post_signed::<Empty>(API::Spot(Spot::OrderTest), request)
            .map(|_| ())
    }

    // Place a MARKET order - SELL
    pub fn market_sell<S, F>(&self, symbol: S, qty: F) -> Result<Transaction>
    where
        S: Into<String>,
        F: Into<f64>,
    {
        let sell = OrderRequest {
            symbol: symbol.into(),
            qty: qty.into(),
            price: 0.0,
            stop_price: None,
            order_side: OrderSide::Sell,
            order_type: OrderType::Market,
            time_in_force: TimeInForce::GTC,
            new_client_order_id: None,
        };
        let order = self.build_order(sell);
        let request = build_signed_request(order, self.recv_window)?;
        self.client.post_signed(API::Spot(Spot::Order), request)
    }

    /// Place a test MARKET order - SELL
    ///
    /// This order is sandboxed: it is validated, but not sent to the matching engine.
    pub fn test_market_sell<S, F>(&self, symbol: S, qty: F) -> Result<()>
    where
        S: Into<String>,
        F: Into<f64>,
    {
        let sell = OrderRequest {
            symbol: symbol.into(),
            qty: qty.into(),
            price: 0.0,
            stop_price: None,
            order_side: OrderSide::Sell,
            order_type: OrderType::Market,
            time_in_force: TimeInForce::GTC,
            new_client_order_id: None,
        };
        let order = self.build_order(sell);
        let request = build_signed_request(order, self.recv_window)?;
        self.client
            .post_signed::<Empty>(API::Spot(Spot::OrderTest), request)
            .map(|_| ())
    }

    // Place a MARKET order with quote quantity - SELL
    pub fn market_sell_using_quote_quantity<S, F>(
        &self, symbol: S, quote_order_qty: F,
    ) -> Result<Transaction>
    where
        S: Into<String>,
        F: Into<f64>,
    {
        let sell = OrderQuoteQuantityRequest {
            symbol: symbol.into(),
            quote_order_qty: quote_order_qty.into(),
            price: 0.0,
            order_side: OrderSide::Sell,
            order_type: OrderType::Market,
            time_in_force: TimeInForce::GTC,
            new_client_order_id: None,
        };
        let order = self.build_quote_quantity_order(sell);
        let request = build_signed_request(order, self.recv_window)?;
        self.client.post_signed(API::Spot(Spot::Order), request)
    }

    /// Place a test MARKET order with quote quantity - SELL
    ///
    /// This order is sandboxed: it is validated, but not sent to the matching engine.
    pub fn test_market_sell_using_quote_quantity<S, F>(
        &self, symbol: S, quote_order_qty: F,
    ) -> Result<()>
    where
        S: Into<String>,
        F: Into<f64>,
    {
        let sell = OrderQuoteQuantityRequest {
            symbol: symbol.into(),
            quote_order_qty: quote_order_qty.into(),
            price: 0.0,
            order_side: OrderSide::Sell,
            order_type: OrderType::Market,
            time_in_force: TimeInForce::GTC,
            new_client_order_id: None,
        };
        let order = self.build_quote_quantity_order(sell);
        let request = build_signed_request(order, self.recv_window)?;
        self.client
            .post_signed::<Empty>(API::Spot(Spot::OrderTest), request)
            .map(|_| ())
    }

    /// Create a stop limit buy order for the given symbol, price and stop price.
    /// Returning a `Transaction` value with the same parameters sent on the order.
    ///
    ///```no_run
    /// use binance::api::Binance;
    /// use binance::account::*;
    ///
    /// fn main() {
    ///     let api_key = Some("api_key".into());
    ///     let secret_key = Some("secret_key".into());
    ///     let account: Account = Binance::new(api_key, secret_key);
    ///     let result = account.stop_limit_buy_order("LTCBTC", 1, 0.1, 0.09, TimeInForce::GTC);
    /// }
    /// ```
    pub fn stop_limit_buy_order<S, F>(
        &self, symbol: S, qty: F, price: f64, stop_price: f64, time_in_force: TimeInForce,
    ) -> Result<Transaction>
    where
        S: Into<String>,
        F: Into<f64>,
    {
        let sell = OrderRequest {
            symbol: symbol.into(),
            qty: qty.into(),
            price,
            stop_price: Some(stop_price),
            order_side: OrderSide::Buy,
            order_type: OrderType::StopLossLimit,
            time_in_force,
            new_client_order_id: None,
        };
        let order = self.build_order(sell);
        let request = build_signed_request(order, self.recv_window)?;
        self.client.post_signed(API::Spot(Spot::Order), request)
    }

    /// Create a stop limit buy test order for the given symbol, price and stop price.
    /// Returning a `Transaction` value with the same parameters sent on the order.
    ///
    /// This order is sandboxed: it is validated, but not sent to the matching engine.
    ///
    ///```no_run
    /// use binance::api::Binance;
    /// use binance::account::*;
    ///
    /// fn main() {
    ///     let api_key = Some("api_key".into());
    ///     let secret_key = Some("secret_key".into());
    ///     let account: Account = Binance::new(api_key, secret_key);
    ///     let result = account.test_stop_limit_buy_order("LTCBTC", 1, 0.1, 0.09, TimeInForce::GTC);
    /// }
    /// ```
    pub fn test_stop_limit_buy_order<S, F>(
        &self, symbol: S, qty: F, price: f64, stop_price: f64, time_in_force: TimeInForce,
    ) -> Result<()>
    where
        S: Into<String>,
        F: Into<f64>,
    {
        let sell = OrderRequest {
            symbol: symbol.into(),
            qty: qty.into(),
            price,
            stop_price: Some(stop_price),
            order_side: OrderSide::Buy,
            order_type: OrderType::StopLossLimit,
            time_in_force,
            new_client_order_id: None,
        };
        let order = self.build_order(sell);
        let request = build_signed_request(order, self.recv_window)?;
        self.client
            .post_signed::<Empty>(API::Spot(Spot::OrderTest), request)
            .map(|_| ())
    }

    /// Create a stop limit sell order for the given symbol, price and stop price.
    /// Returning a `Transaction` value with the same parameters sent on the order.
    ///
    ///```no_run
    /// use binance::api::Binance;
    /// use binance::account::*;
    ///
    /// fn main() {
    ///     let api_key = Some("api_key".into());
    ///     let secret_key = Some("secret_key".into());
    ///     let account: Account = Binance::new(api_key, secret_key);
    ///     let result = account.stop_limit_sell_order("LTCBTC", 1, 0.1, 0.09, TimeInForce::GTC);
    /// }
    /// ```
    pub fn stop_limit_sell_order<S, F>(
        &self, symbol: S, qty: F, price: f64, stop_price: f64, time_in_force: TimeInForce,
    ) -> Result<Transaction>
    where
        S: Into<String>,
        F: Into<f64>,
    {
        let sell = OrderRequest {
            symbol: symbol.into(),
            qty: qty.into(),
            price,
            stop_price: Some(stop_price),
            order_side: OrderSide::Sell,
            order_type: OrderType::StopLossLimit,
            time_in_force,
            new_client_order_id: None,
        };
        let order = self.build_order(sell);
        let request = build_signed_request(order, self.recv_window)?;
        self.client.post_signed(API::Spot(Spot::Order), request)
    }

    /// Create a stop limit sell order for the given symbol, price and stop price.
    /// Returning a `Transaction` value with the same parameters sent on the order.
    ///
    /// This order is sandboxed: it is validated, but not sent to the matching engine.
    ///
    ///```no_run
    /// use binance::api::Binance;
    /// use binance::account::*;
    ///
    /// fn main() {
    ///     let api_key = Some("api_key".into());
    ///     let secret_key = Some("secret_key".into());
    ///     let account: Account = Binance::new(api_key, secret_key);
    ///     let result = account.test_stop_limit_sell_order("LTCBTC", 1, 0.1, 0.09, TimeInForce::GTC);
    /// }
    /// ```
    pub fn test_stop_limit_sell_order<S, F>(
        &self, symbol: S, qty: F, price: f64, stop_price: f64, time_in_force: TimeInForce,
    ) -> Result<()>
    where
        S: Into<String>,
        F: Into<f64>,
    {
        let sell = OrderRequest {
            symbol: symbol.into(),
            qty: qty.into(),
            price,
            stop_price: Some(stop_price),
            order_side: OrderSide::Sell,
            order_type: OrderType::StopLossLimit,
            time_in_force,
            new_client_order_id: None,
        };
        let order = self.build_order(sell);
        let request = build_signed_request(order, self.recv_window)?;
        self.client
            .post_signed::<Empty>(API::Spot(Spot::OrderTest), request)
            .map(|_| ())
    }

    /// Place a custom order
    #[allow(clippy::too_many_arguments)]
    pub fn custom_order<S, F>(
        &self, symbol: S, qty: F, price: f64, stop_price: Option<f64>, order_side: OrderSide,
        order_type: OrderType, time_in_force: TimeInForce, new_client_order_id: Option<String>,
    ) -> Result<Transaction>
    where
        S: Into<String>,
        F: Into<f64>,
    {
        let sell = OrderRequest {
            symbol: symbol.into(),
            qty: qty.into(),
            price,
            stop_price,
            order_side,
            order_type,
            time_in_force,
            new_client_order_id,
        };
        let order = self.build_order(sell);
        let request = build_signed_request(order, self.recv_window)?;
        self.client.post_signed(API::Spot(Spot::Order), request)
    }

    /// Place a test custom order
    ///
    /// This order is sandboxed: it is validated, but not sent to the matching engine.
    #[allow(clippy::too_many_arguments)]
    pub fn test_custom_order<S, F>(
        &self, symbol: S, qty: F, price: f64, stop_price: Option<f64>, order_side: OrderSide,
        order_type: OrderType, time_in_force: TimeInForce, new_client_order_id: Option<String>,
    ) -> Result<()>
    where
        S: Into<String>,
        F: Into<f64>,
    {
        let sell = OrderRequest {
            symbol: symbol.into(),
            qty: qty.into(),
            price,
            stop_price,
            order_side,
            order_type,
            time_in_force,
            new_client_order_id,
        };
        let order = self.build_order(sell);
        let request = build_signed_request(order, self.recv_window)?;
        self.client
            .post_signed::<Empty>(API::Spot(Spot::OrderTest), request)
            .map(|_| ())
    }

    // Check an order's status
    pub fn cancel_order<S>(&self, symbol: S, order_id: u64) -> Result<OrderCanceled>
    where
        S: Into<String>,
    {
        let mut parameters: BTreeMap<String, String> = BTreeMap::new();
        parameters.insert("symbol".into(), symbol.into());
        parameters.insert("orderId".into(), order_id.to_string());

        let request = build_signed_request(parameters, self.recv_window)?;
        self.client
            .delete_signed(API::Spot(Spot::Order), Some(request))
    }

    pub fn cancel_order_with_client_id<S>(
        &self, symbol: S, orig_client_order_id: String,
    ) -> Result<OrderCanceled>
    where
        S: Into<String>,
    {
        let mut parameters: BTreeMap<String, String> = BTreeMap::new();
        parameters.insert("symbol".into(), symbol.into());
        parameters.insert("origClientOrderId".into(), orig_client_order_id);

        let request = build_signed_request(parameters, self.recv_window)?;
        self.client
            .delete_signed(API::Spot(Spot::Order), Some(request))
    }

    pub fn cancel_order_with_client_id_rs<S>() {}
    /// Place a test cancel order
    ///
    /// This order is sandboxed: it is validated, but not sent to the matching engine.
    pub fn test_cancel_order<S>(&self, symbol: S, order_id: u64) -> Result<()>
    where
        S: Into<String>,
    {
        let mut parameters: BTreeMap<String, String> = BTreeMap::new();
        parameters.insert("symbol".into(), symbol.into());
        parameters.insert("orderId".into(), order_id.to_string());
        let request = build_signed_request(parameters, self.recv_window)?;
        self.client
            .delete_signed::<Empty>(API::Spot(Spot::OrderTest), Some(request))
            .map(|_| ())
    }

    // Trade history
    pub fn trade_history<S>(&self, symbol: S) -> Result<Vec<TradeHistory>>
    where
        S: Into<String>,
    {
        let mut parameters: BTreeMap<String, String> = BTreeMap::new();
        parameters.insert("symbol".into(), symbol.into());

        let request = build_signed_request(parameters, self.recv_window)?;
        self.client
            .get_signed(API::Spot(Spot::MyTrades), Some(request))
    }

    // Trade history starting from selected date
    pub fn trade_history_from<S>(&self, symbol: S, start_time: u64) -> Result<Vec<TradeHistory>>
    where
        S: Into<String>,
    {
        if !is_start_time_valid(&start_time) {
            // return bail!("Start time should be less than the current time");
            return Err(SdkError::Other(
                "Start time should be less than the current time".into(),
            ));
        }

        let mut parameters: BTreeMap<String, String> = BTreeMap::new();
        parameters.insert("symbol".into(), symbol.into());
        parameters.insert("startTime".into(), start_time.to_string());
        let request = build_signed_request(parameters, self.recv_window)?;
        self.client
            .get_signed(API::Spot(Spot::MyTrades), Some(request))
    }

    // Trade history starting from selected time to some time
    pub fn trade_history_from_to<S>(
        &self, symbol: S, start_time: u64, end_time: u64,
    ) -> Result<Vec<TradeHistory>>
    where
        S: Into<String>,
    {
        if end_time <= start_time {
            // return bail!("End time should be greater than start time");
            return Err(SdkError::Other(
                "End time should be greater than start time".into(),
            ));
        }
        if !is_start_time_valid(&start_time) {
            // return bail!("Start time should be less than the current time");
            return Err(SdkError::Other(
                "Start time should be less than the current time".into(),
            ));
        }
        self.get_trades(symbol, start_time, end_time)
    }

    fn get_trades<S>(&self, symbol: S, start_time: u64, end_time: u64) -> Result<Vec<TradeHistory>>
    where
        S: Into<String>,
    {
        let mut trades = self.trade_history_from(symbol, start_time)?;
        trades.retain(|trade| trade.time <= end_time);
        Ok(trades)
    }

    fn build_order(&self, order: OrderRequest) -> BTreeMap<String, String> {
        let mut order_parameters: BTreeMap<String, String> = BTreeMap::new();

        order_parameters.insert("symbol".into(), order.symbol);
        order_parameters.insert("side".into(), order.order_side.to_string());
        order_parameters.insert("type".into(), order.order_type.to_string());
        order_parameters.insert("quantity".into(), order.qty.to_string());

        if let Some(stop_price) = order.stop_price {
            order_parameters.insert("stopPrice".into(), stop_price.to_string());
        }

        if order.price != 0.0 {
            order_parameters.insert("price".into(), order.price.to_string());
            order_parameters.insert("timeInForce".into(), order.time_in_force.to_string());
        }

        if let Some(client_order_id) = order.new_client_order_id {
            order_parameters.insert("newClientOrderId".into(), client_order_id);
        }

        order_parameters
    }

    fn build_quote_quantity_order(
        &self, order: OrderQuoteQuantityRequest,
    ) -> BTreeMap<String, String> {
        let mut order_parameters: BTreeMap<String, String> = BTreeMap::new();

        order_parameters.insert("symbol".into(), order.symbol);
        order_parameters.insert("side".into(), order.order_side.to_string());
        order_parameters.insert("type".into(), order.order_type.to_string());
        order_parameters.insert("quoteOrderQty".into(), order.quote_order_qty.to_string());

        if order.price != 0.0 {
            order_parameters.insert("price".into(), order.price.to_string());
            order_parameters.insert("timeInForce".into(), order.time_in_force.to_string());
        }

        if let Some(client_order_id) = order.new_client_order_id {
            order_parameters.insert("newClientOrderId".into(), client_order_id);
        }

        order_parameters
    }
}
