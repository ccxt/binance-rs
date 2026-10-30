use error_chain::bail;

use crate::util::{build_signed_request, is_start_time_valid, uuid_spot};
use crate::model::{
    AccountCommission, AccountInformation, Allocation, AmendKeepPriorityResponse, Balance,
    CancelReplaceResponse, Empty, MyFilters, Order, OrderCanceled, OrderList, OrderRateLimit,
    PreventedMatch, SpotOrderAmendment, TradeHistory, Transaction,
};
use crate::client::Client;
use crate::errors::Result;
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

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum OrderType {
    Limit,
    Market,
    StopLossLimit,
    StopLoss,
    TakeProfit,
    TakeProfitLimit,
    LimitMaker,
}

impl OrderType {
    pub fn from_int(value: i32) -> Option<Self> {
        match value {
            1 => Some(OrderType::Limit),
            2 => Some(OrderType::Market),
            3 => Some(OrderType::StopLossLimit),
            4 => Some(OrderType::StopLoss),
            5 => Some(OrderType::TakeProfit),
            6 => Some(OrderType::TakeProfitLimit),
            7 => Some(OrderType::LimitMaker),
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
            Self::StopLoss => write!(f, "STOP_LOSS"),
            Self::TakeProfit => write!(f, "TAKE_PROFIT"),
            Self::TakeProfitLimit => write!(f, "TAKE_PROFIT_LIMIT"),
            Self::LimitMaker => write!(f, "LIMIT_MAKER"),
        }
    }
}

/// Self-trade prevention modes. The allowed set depends on the symbol
/// (see `allowed_self_trade_prevention_modes` in exchange info).
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SelfTradePreventionMode {
    None,
    ExpireTaker,
    ExpireMaker,
    ExpireBoth,
    Decrement,
    Transfer,
}

impl Display for SelfTradePreventionMode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::None => write!(f, "NONE"),
            Self::ExpireTaker => write!(f, "EXPIRE_TAKER"),
            Self::ExpireMaker => write!(f, "EXPIRE_MAKER"),
            Self::ExpireBoth => write!(f, "EXPIRE_BOTH"),
            Self::Decrement => write!(f, "DECREMENT"),
            Self::Transfer => write!(f, "TRANSFER"),
        }
    }
}

/// How `POST /api/v3/order/cancelReplace` behaves when the cancel fails.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CancelReplaceMode {
    /// If the cancel request fails, the new order placement will not be attempted.
    StopOnFailure,
    /// The new order placement will be attempted even if the cancel request fails.
    AllowFailure,
}

impl Display for CancelReplaceMode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::StopOnFailure => write!(f, "STOP_ON_FAILURE"),
            Self::AllowFailure => write!(f, "ALLOW_FAILURE"),
        }
    }
}

/// Parameters of one leg of an order list (OCO above/below, OTO working/pending, ...).
///
/// Only the fields relevant to the leg's `order_type` are sent; everything is optional
/// except `order_type`.
#[derive(Debug, Clone, Default)]
pub struct OrderListLeg {
    pub order_type: Option<OrderType>,
    pub side: Option<OrderSide>,
    pub quantity: Option<f64>,
    pub price: Option<f64>,
    pub stop_price: Option<f64>,
    pub trailing_delta: Option<u64>,
    pub time_in_force: Option<TimeInForce>,
    pub client_order_id: Option<String>,
    pub iceberg_qty: Option<f64>,
    pub strategy_id: Option<u64>,
    pub strategy_type: Option<u64>,
}

impl OrderListLeg {
    /// Insert this leg's parameters with the given prefix (`above`, `below`, `working`,
    /// `pending`, `pendingAbove`, `pendingBelow`) into `params`.
    fn insert_into(&self, prefix: &str, params: &mut BTreeMap<String, String>) {
        if let Some(t) = &self.order_type {
            params.insert(format!("{}Type", prefix), t.to_string());
        }
        if let Some(side) = &self.side {
            params.insert(format!("{}Side", prefix), side.to_string());
        }
        if let Some(q) = self.quantity {
            params.insert(format!("{}Quantity", prefix), q.to_string());
        }
        if let Some(p) = self.price {
            params.insert(format!("{}Price", prefix), p.to_string());
        }
        if let Some(sp) = self.stop_price {
            params.insert(format!("{}StopPrice", prefix), sp.to_string());
        }
        if let Some(td) = self.trailing_delta {
            params.insert(format!("{}TrailingDelta", prefix), td.to_string());
        }
        if let Some(tif) = &self.time_in_force {
            params.insert(format!("{}TimeInForce", prefix), tif.to_string());
        }
        if let Some(id) = &self.client_order_id {
            params.insert(format!("{}ClientOrderId", prefix), id.clone());
        }
        if let Some(iq) = self.iceberg_qty {
            params.insert(format!("{}IcebergQty", prefix), iq.to_string());
        }
        if let Some(sid) = self.strategy_id {
            params.insert(format!("{}StrategyId", prefix), sid.to_string());
        }
        if let Some(st) = self.strategy_type {
            params.insert(format!("{}StrategyType", prefix), st.to_string());
        }
    }
}

/// `POST /api/v3/orderList/oco` request.
///
/// One leg must be `LIMIT_MAKER`/`TAKE_PROFIT`/`TAKE_PROFIT_LIMIT` and the other
/// `STOP_LOSS`/`STOP_LOSS_LIMIT`. `quantity` applies to both legs.
#[derive(Debug, Clone)]
pub struct OcoOrderListRequest {
    pub symbol: String,
    pub side: OrderSide,
    pub quantity: f64,
    pub above: OrderListLeg,
    pub below: OrderListLeg,
    pub list_client_order_id: Option<String>,
    pub new_order_resp_type: Option<String>,
    pub self_trade_prevention_mode: Option<SelfTradePreventionMode>,
}

/// `POST /api/v3/orderList/oto` request: the `pending` order is placed once the
/// `working` order (`LIMIT` or `LIMIT_MAKER`) is fully filled.
#[derive(Debug, Clone)]
pub struct OtoOrderListRequest {
    pub symbol: String,
    pub working: OrderListLeg,
    pub pending: OrderListLeg,
    pub list_client_order_id: Option<String>,
    pub new_order_resp_type: Option<String>,
    pub self_trade_prevention_mode: Option<SelfTradePreventionMode>,
}

/// `POST /api/v3/orderList/otoco` request: once the `working` order fills, an OCO made of
/// `pending_above` and `pending_below` is placed. `pending_side` and `pending_quantity`
/// apply to both pending legs.
#[derive(Debug, Clone)]
pub struct OtocoOrderListRequest {
    pub symbol: String,
    pub working: OrderListLeg,
    pub pending_side: OrderSide,
    pub pending_quantity: f64,
    pub pending_above: OrderListLeg,
    pub pending_below: OrderListLeg,
    pub list_client_order_id: Option<String>,
    pub new_order_resp_type: Option<String>,
    pub self_trade_prevention_mode: Option<SelfTradePreventionMode>,
}

/// `POST /api/v3/order/cancelReplace` request.
///
/// Either `cancel_order_id` or `cancel_orig_client_order_id` must be set. The new order
/// is described by the remaining fields, which mirror `POST /api/v3/order`.
#[derive(Debug, Clone)]
pub struct CancelReplaceRequest {
    pub symbol: String,
    pub side: OrderSide,
    pub order_type: OrderType,
    pub cancel_replace_mode: CancelReplaceMode,
    pub cancel_order_id: Option<u64>,
    pub cancel_orig_client_order_id: Option<String>,
    pub cancel_new_client_order_id: Option<String>,
    pub time_in_force: Option<TimeInForce>,
    pub quantity: Option<f64>,
    pub quote_order_qty: Option<f64>,
    pub price: Option<f64>,
    pub new_client_order_id: Option<String>,
    pub stop_price: Option<f64>,
    pub trailing_delta: Option<u64>,
    pub iceberg_qty: Option<f64>,
    pub strategy_id: Option<u64>,
    pub strategy_type: Option<u64>,
    pub new_order_resp_type: Option<String>,
    pub self_trade_prevention_mode: Option<SelfTradePreventionMode>,
    /// `ONLY_NEW` or `ONLY_PARTIALLY_FILLED`
    pub cancel_restrictions: Option<String>,
    /// `DO_NOTHING` (default) or `CANCEL_ONLY`
    pub order_rate_limit_exceeded_mode: Option<String>,
}

#[derive(Debug, Clone)]
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
#[derive(Debug, Clone, PartialEq, Eq)]
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

    /// Account information, optionally omitting zero balances (`omitZeroBalances=true`).
    pub fn get_account_with_params(&self, omit_zero_balances: bool) -> Result<AccountInformation> {
        let mut parameters: BTreeMap<String, String> = BTreeMap::new();
        if omit_zero_balances {
            parameters.insert("omitZeroBalances".into(), "true".into());
        }
        let request = build_signed_request(parameters, self.recv_window)?;
        self.client
            .get_signed(API::Spot(Spot::Account), Some(request))
    }

    /// Current commission rates for a symbol (`GET /api/v3/account/commission`).
    pub fn get_account_commission<S>(&self, symbol: S) -> Result<AccountCommission>
    where
        S: Into<String>,
    {
        let mut parameters: BTreeMap<String, String> = BTreeMap::new();
        parameters.insert("symbol".into(), symbol.into());
        let request = build_signed_request(parameters, self.recv_window)?;
        self.client
            .get_signed(API::Spot(Spot::AccountCommission), Some(request))
    }

    /// Filters relevant to this account on a symbol (`GET /api/v3/myFilters`).
    ///
    /// This is the only endpoint that shows account-level `MAX_ASSET` filters.
    pub fn get_my_filters<S>(&self, symbol: S) -> Result<MyFilters>
    where
        S: Into<String>,
    {
        let mut parameters: BTreeMap<String, String> = BTreeMap::new();
        parameters.insert("symbol".into(), symbol.into());
        let request = build_signed_request(parameters, self.recv_window)?;
        self.client
            .get_signed(API::Spot(Spot::MyFilters), Some(request))
    }

    /// Unfilled order count for all intervals (`GET /api/v3/rateLimit/order`).
    pub fn get_order_rate_limits(&self) -> Result<Vec<OrderRateLimit>> {
        let request = build_signed_request(BTreeMap::new(), self.recv_window)?;
        self.client
            .get_signed(API::Spot(Spot::RateLimitOrder), Some(request))
    }

    /// Orders that expired because of self-trade prevention (`GET /api/v3/myPreventedMatches`).
    ///
    /// Supported combinations: `symbol` + `prevented_match_id`, `symbol` + `order_id`,
    /// `symbol` + `order_id` + `from_prevented_match_id`.
    pub fn get_prevented_matches<S, P, O, F, L>(
        &self, symbol: S, prevented_match_id: P, order_id: O, from_prevented_match_id: F, limit: L,
    ) -> Result<Vec<PreventedMatch>>
    where
        S: Into<String>,
        P: Into<Option<u64>>,
        O: Into<Option<u64>>,
        F: Into<Option<u64>>,
        L: Into<Option<u16>>,
    {
        let mut parameters: BTreeMap<String, String> = BTreeMap::new();
        parameters.insert("symbol".into(), symbol.into());
        if let Some(id) = prevented_match_id.into() {
            parameters.insert("preventedMatchId".into(), id.to_string());
        }
        if let Some(id) = order_id.into() {
            parameters.insert("orderId".into(), id.to_string());
        }
        if let Some(id) = from_prevented_match_id.into() {
            parameters.insert("fromPreventedMatchId".into(), id.to_string());
        }
        if let Some(lt) = limit.into() {
            parameters.insert("limit".into(), lt.to_string());
        }
        let request = build_signed_request(parameters, self.recv_window)?;
        self.client
            .get_signed(API::Spot(Spot::MyPreventedMatches), Some(request))
    }

    /// Allocations resulting from SOR order placement (`GET /api/v3/myAllocations`).
    #[allow(clippy::too_many_arguments)]
    pub fn get_allocations<S, T1, T2, F, L, O>(
        &self, symbol: S, start_time: T1, end_time: T2, from_allocation_id: F, limit: L,
        order_id: O,
    ) -> Result<Vec<Allocation>>
    where
        S: Into<String>,
        T1: Into<Option<u64>>,
        T2: Into<Option<u64>>,
        F: Into<Option<u64>>,
        L: Into<Option<u16>>,
        O: Into<Option<u64>>,
    {
        let mut parameters: BTreeMap<String, String> = BTreeMap::new();
        parameters.insert("symbol".into(), symbol.into());
        if let Some(st) = start_time.into() {
            parameters.insert("startTime".into(), st.to_string());
        }
        if let Some(et) = end_time.into() {
            parameters.insert("endTime".into(), et.to_string());
        }
        if let Some(id) = from_allocation_id.into() {
            parameters.insert("fromAllocationId".into(), id.to_string());
        }
        if let Some(lt) = limit.into() {
            parameters.insert("limit".into(), lt.to_string());
        }
        if let Some(id) = order_id.into() {
            parameters.insert("orderId".into(), id.to_string());
        }
        let request = build_signed_request(parameters, self.recv_window)?;
        self.client
            .get_signed(API::Spot(Spot::MyAllocations), Some(request))
    }

    /// All orders (active, canceled, filled) for a symbol (`GET /api/v3/allOrders`).
    ///
    /// If `order_id` is set, orders >= that id are returned; otherwise the most recent ones.
    pub fn get_all_orders<S, O, T1, T2, L>(
        &self, symbol: S, order_id: O, start_time: T1, end_time: T2, limit: L,
    ) -> Result<Vec<Order>>
    where
        S: Into<String>,
        O: Into<Option<u64>>,
        T1: Into<Option<u64>>,
        T2: Into<Option<u64>>,
        L: Into<Option<u16>>,
    {
        let mut parameters: BTreeMap<String, String> = BTreeMap::new();
        parameters.insert("symbol".into(), symbol.into());
        if let Some(id) = order_id.into() {
            parameters.insert("orderId".into(), id.to_string());
        }
        if let Some(st) = start_time.into() {
            parameters.insert("startTime".into(), st.to_string());
        }
        if let Some(et) = end_time.into() {
            parameters.insert("endTime".into(), et.to_string());
        }
        if let Some(lt) = limit.into() {
            parameters.insert("limit".into(), lt.to_string());
        }
        let request = build_signed_request(parameters, self.recv_window)?;
        self.client
            .get_signed(API::Spot(Spot::AllOrders), Some(request))
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
                bail!("Asset not found");
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

    /// Check an order's status by its client order id.
    pub fn order_status_with_client_id<S>(
        &self, symbol: S, orig_client_order_id: String,
    ) -> Result<Order>
    where
        S: Into<String>,
    {
        let mut parameters: BTreeMap<String, String> = BTreeMap::new();
        parameters.insert("symbol".into(), symbol.into());
        parameters.insert("origClientOrderId".into(), orig_client_order_id);

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
        let order = self.build_order(buy, None);
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
        let order = self.build_order(buy, None);
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
        let order = self.build_order(sell, None);
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
        let order = self.build_order(sell, None);
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
        let order = self.build_order(buy, None);
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
        let order = self.build_order(buy, None);
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
        let order = self.build_order(sell, None);
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
        let order = self.build_order(sell, None);
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
        let order = self.build_order(sell, None);
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
        let order = self.build_order(sell, None);
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
        let order = self.build_order(sell, None);
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
        let order = self.build_order(sell, None);
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
        self.custom_order_with_params(
            symbol,
            qty,
            price,
            stop_price,
            order_side,
            order_type,
            time_in_force,
            new_client_order_id,
            BTreeMap::new(),
        )
    }

    /// Place a custom order
    #[allow(clippy::too_many_arguments)]
    pub fn custom_order_with_params<S, F>(
        &self, symbol: S, qty: F, price: f64, stop_price: Option<f64>, order_side: OrderSide,
        order_type: OrderType, time_in_force: TimeInForce, new_client_order_id: Option<String>,
        request_params: BTreeMap<String, String>,
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
        let order = self.build_order(sell, Some(request_params));
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
        let order = self.build_order(sell, None);
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

    /// Cancel an existing order and immediately place a new one
    /// (`POST /api/v3/order/cancelReplace`).
    ///
    /// Note that Binance answers with HTTP 400 when the cancel fails in
    /// `STOP_ON_FAILURE` mode, which surfaces here as a `BinanceError`.
    pub fn cancel_replace_order(&self, req: CancelReplaceRequest) -> Result<CancelReplaceResponse> {
        self.cancel_replace_order_with_params(req, BTreeMap::new())
    }

    /// Same as [`Account::cancel_replace_order`] with extra raw request parameters.
    pub fn cancel_replace_order_with_params(
        &self, req: CancelReplaceRequest, request_params: BTreeMap<String, String>,
    ) -> Result<CancelReplaceResponse> {
        let mut p: BTreeMap<String, String> = BTreeMap::new();
        p.insert("symbol".into(), req.symbol);
        p.insert("side".into(), req.side.to_string());
        p.insert("type".into(), req.order_type.to_string());
        p.insert(
            "cancelReplaceMode".into(),
            req.cancel_replace_mode.to_string(),
        );
        if let Some(id) = req.cancel_order_id {
            p.insert("cancelOrderId".into(), id.to_string());
        }
        if let Some(id) = req.cancel_orig_client_order_id {
            p.insert("cancelOrigClientOrderId".into(), id);
        }
        if let Some(id) = req.cancel_new_client_order_id {
            p.insert("cancelNewClientOrderId".into(), id);
        }
        if let Some(tif) = req.time_in_force {
            p.insert("timeInForce".into(), tif.to_string());
        }
        if let Some(q) = req.quantity {
            p.insert("quantity".into(), q.to_string());
        }
        if let Some(q) = req.quote_order_qty {
            p.insert("quoteOrderQty".into(), q.to_string());
        }
        if let Some(price) = req.price {
            p.insert("price".into(), price.to_string());
        }
        p.insert(
            "newClientOrderId".into(),
            req.new_client_order_id.unwrap_or_else(uuid_spot),
        );
        if let Some(sp) = req.stop_price {
            p.insert("stopPrice".into(), sp.to_string());
        }
        if let Some(td) = req.trailing_delta {
            p.insert("trailingDelta".into(), td.to_string());
        }
        if let Some(iq) = req.iceberg_qty {
            p.insert("icebergQty".into(), iq.to_string());
        }
        if let Some(sid) = req.strategy_id {
            p.insert("strategyId".into(), sid.to_string());
        }
        if let Some(st) = req.strategy_type {
            p.insert("strategyType".into(), st.to_string());
        }
        if let Some(rt) = req.new_order_resp_type {
            p.insert("newOrderRespType".into(), rt);
        }
        if let Some(stp) = req.self_trade_prevention_mode {
            p.insert("selfTradePreventionMode".into(), stp.to_string());
        }
        if let Some(cr) = req.cancel_restrictions {
            p.insert("cancelRestrictions".into(), cr);
        }
        if let Some(m) = req.order_rate_limit_exceeded_mode {
            p.insert("orderRateLimitExceededMode".into(), m);
        }
        for (key, value) in request_params {
            p.insert(key, value);
        }
        let request = build_signed_request(p, self.recv_window)?;
        self.client
            .post_signed(API::Spot(Spot::OrderCancelReplace), request)
    }

    /// Reduce the quantity of an open order while keeping its queue priority
    /// (`PUT /api/v3/order/amend/keepPriority`).
    ///
    /// Exactly one of `order_id` / `orig_client_order_id` must be provided; `new_qty`
    /// must be greater than 0 and lower than the order's current quantity.
    pub fn amend_order_keep_priority<S, O, C, N>(
        &self, symbol: S, order_id: O, orig_client_order_id: C, new_qty: f64,
        new_client_order_id: N,
    ) -> Result<AmendKeepPriorityResponse>
    where
        S: Into<String>,
        O: Into<Option<u64>>,
        C: Into<Option<String>>,
        N: Into<Option<String>>,
    {
        let mut parameters: BTreeMap<String, String> = BTreeMap::new();
        parameters.insert("symbol".into(), symbol.into());
        parameters.insert("newQty".into(), new_qty.to_string());
        if let Some(id) = order_id.into() {
            parameters.insert("orderId".into(), id.to_string());
        }
        if let Some(id) = orig_client_order_id.into() {
            parameters.insert("origClientOrderId".into(), id);
        }
        if let Some(id) = new_client_order_id.into() {
            parameters.insert("newClientOrderId".into(), id);
        }
        let request = build_signed_request(parameters, self.recv_window)?;
        self.client
            .put_signed(API::Spot(Spot::OrderAmendKeepPriority), request)
    }

    /// Amendments made to an order (`GET /api/v3/order/amendments`).
    pub fn get_order_amendments<S, F, L>(
        &self, symbol: S, order_id: u64, from_execution_id: F, limit: L,
    ) -> Result<Vec<SpotOrderAmendment>>
    where
        S: Into<String>,
        F: Into<Option<u64>>,
        L: Into<Option<u16>>,
    {
        let mut parameters: BTreeMap<String, String> = BTreeMap::new();
        parameters.insert("symbol".into(), symbol.into());
        parameters.insert("orderId".into(), order_id.to_string());
        if let Some(id) = from_execution_id.into() {
            parameters.insert("fromExecutionId".into(), id.to_string());
        }
        if let Some(lt) = limit.into() {
            parameters.insert("limit".into(), lt.to_string());
        }
        let request = build_signed_request(parameters, self.recv_window)?;
        self.client
            .get_signed(API::Spot(Spot::OrderAmendments), Some(request))
    }

    /// Place an OCO order list (`POST /api/v3/orderList/oco`).
    ///
    /// This replaces the deprecated `POST /api/v3/order/oco` endpoint.
    pub fn new_oco_order_list(&self, req: OcoOrderListRequest) -> Result<OrderList> {
        let mut p: BTreeMap<String, String> = BTreeMap::new();
        p.insert("symbol".into(), req.symbol);
        p.insert("side".into(), req.side.to_string());
        p.insert("quantity".into(), req.quantity.to_string());
        req.above.insert_into("above", &mut p);
        req.below.insert_into("below", &mut p);
        Self::insert_list_common(
            &mut p,
            req.list_client_order_id,
            req.new_order_resp_type,
            req.self_trade_prevention_mode,
        );
        let request = build_signed_request(p, self.recv_window)?;
        self.client
            .post_signed(API::Spot(Spot::OrderListOco), request)
    }

    /// Place an OTO (one-triggers-the-other) order list (`POST /api/v3/orderList/oto`).
    pub fn new_oto_order_list(&self, req: OtoOrderListRequest) -> Result<OrderList> {
        let mut p: BTreeMap<String, String> = BTreeMap::new();
        p.insert("symbol".into(), req.symbol);
        req.working.insert_into("working", &mut p);
        req.pending.insert_into("pending", &mut p);
        Self::insert_list_common(
            &mut p,
            req.list_client_order_id,
            req.new_order_resp_type,
            req.self_trade_prevention_mode,
        );
        let request = build_signed_request(p, self.recv_window)?;
        self.client
            .post_signed(API::Spot(Spot::OrderListOto), request)
    }

    /// Place an OTOCO (one-triggers-one-cancels-the-other) order list
    /// (`POST /api/v3/orderList/otoco`).
    pub fn new_otoco_order_list(&self, req: OtocoOrderListRequest) -> Result<OrderList> {
        let mut p: BTreeMap<String, String> = BTreeMap::new();
        p.insert("symbol".into(), req.symbol);
        req.working.insert_into("working", &mut p);
        p.insert("pendingSide".into(), req.pending_side.to_string());
        p.insert("pendingQuantity".into(), req.pending_quantity.to_string());
        req.pending_above.insert_into("pendingAbove", &mut p);
        req.pending_below.insert_into("pendingBelow", &mut p);
        Self::insert_list_common(
            &mut p,
            req.list_client_order_id,
            req.new_order_resp_type,
            req.self_trade_prevention_mode,
        );
        let request = build_signed_request(p, self.recv_window)?;
        self.client
            .post_signed(API::Spot(Spot::OrderListOtoco), request)
    }

    fn insert_list_common(
        p: &mut BTreeMap<String, String>, list_client_order_id: Option<String>,
        new_order_resp_type: Option<String>, stp: Option<SelfTradePreventionMode>,
    ) {
        if let Some(id) = list_client_order_id {
            p.insert("listClientOrderId".into(), id);
        }
        if let Some(rt) = new_order_resp_type {
            p.insert("newOrderRespType".into(), rt);
        }
        if let Some(stp) = stp {
            p.insert("selfTradePreventionMode".into(), stp.to_string());
        }
    }

    /// Query an order list by `orderListId` (`GET /api/v3/orderList`).
    pub fn get_order_list(&self, order_list_id: u64) -> Result<OrderList> {
        let mut parameters: BTreeMap<String, String> = BTreeMap::new();
        parameters.insert("orderListId".into(), order_list_id.to_string());
        let request = build_signed_request(parameters, self.recv_window)?;
        self.client
            .get_signed(API::Spot(Spot::OrderList), Some(request))
    }

    /// Query an order list by its `listClientOrderId` (`GET /api/v3/orderList`).
    pub fn get_order_list_with_client_id<S>(&self, list_client_order_id: S) -> Result<OrderList>
    where
        S: Into<String>,
    {
        let mut parameters: BTreeMap<String, String> = BTreeMap::new();
        parameters.insert("origClientOrderId".into(), list_client_order_id.into());
        let request = build_signed_request(parameters, self.recv_window)?;
        self.client
            .get_signed(API::Spot(Spot::OrderList), Some(request))
    }

    /// All order lists (`GET /api/v3/allOrderList`).
    pub fn get_all_order_lists<F, T1, T2, L>(
        &self, from_id: F, start_time: T1, end_time: T2, limit: L,
    ) -> Result<Vec<OrderList>>
    where
        F: Into<Option<u64>>,
        T1: Into<Option<u64>>,
        T2: Into<Option<u64>>,
        L: Into<Option<u16>>,
    {
        let mut parameters: BTreeMap<String, String> = BTreeMap::new();
        if let Some(id) = from_id.into() {
            parameters.insert("fromId".into(), id.to_string());
        }
        if let Some(st) = start_time.into() {
            parameters.insert("startTime".into(), st.to_string());
        }
        if let Some(et) = end_time.into() {
            parameters.insert("endTime".into(), et.to_string());
        }
        if let Some(lt) = limit.into() {
            parameters.insert("limit".into(), lt.to_string());
        }
        let request = build_signed_request(parameters, self.recv_window)?;
        self.client
            .get_signed(API::Spot(Spot::AllOrderList), Some(request))
    }

    /// Currently open order lists (`GET /api/v3/openOrderList`).
    pub fn get_open_order_lists(&self) -> Result<Vec<OrderList>> {
        let request = build_signed_request(BTreeMap::new(), self.recv_window)?;
        self.client
            .get_signed(API::Spot(Spot::OpenOrderList), Some(request))
    }

    /// Cancel an entire order list (`DELETE /api/v3/orderList`).
    ///
    /// Either `order_list_id` or `list_client_order_id` must be provided.
    pub fn cancel_order_list<S, O, C>(
        &self, symbol: S, order_list_id: O, list_client_order_id: C,
    ) -> Result<OrderList>
    where
        S: Into<String>,
        O: Into<Option<u64>>,
        C: Into<Option<String>>,
    {
        let mut parameters: BTreeMap<String, String> = BTreeMap::new();
        parameters.insert("symbol".into(), symbol.into());
        if let Some(id) = order_list_id.into() {
            parameters.insert("orderListId".into(), id.to_string());
        }
        if let Some(id) = list_client_order_id.into() {
            parameters.insert("listClientOrderId".into(), id);
        }
        let request = build_signed_request(parameters, self.recv_window)?;
        self.client
            .delete_signed(API::Spot(Spot::OrderList), Some(request))
    }

    /// Account trade list with all supported filters (`GET /api/v3/myTrades`).
    ///
    /// `order_id` can only be combined with `symbol`; `from_id` cannot be combined with
    /// `start_time`/`end_time`.
    pub fn trade_history_with_params<S, O, T1, T2, F, L>(
        &self, symbol: S, order_id: O, start_time: T1, end_time: T2, from_id: F, limit: L,
    ) -> Result<Vec<TradeHistory>>
    where
        S: Into<String>,
        O: Into<Option<u64>>,
        T1: Into<Option<u64>>,
        T2: Into<Option<u64>>,
        F: Into<Option<u64>>,
        L: Into<Option<u16>>,
    {
        let mut parameters: BTreeMap<String, String> = BTreeMap::new();
        parameters.insert("symbol".into(), symbol.into());
        if let Some(id) = order_id.into() {
            parameters.insert("orderId".into(), id.to_string());
        }
        if let Some(st) = start_time.into() {
            parameters.insert("startTime".into(), st.to_string());
        }
        if let Some(et) = end_time.into() {
            parameters.insert("endTime".into(), et.to_string());
        }
        if let Some(id) = from_id.into() {
            parameters.insert("fromId".into(), id.to_string());
        }
        if let Some(lt) = limit.into() {
            parameters.insert("limit".into(), lt.to_string());
        }
        let request = build_signed_request(parameters, self.recv_window)?;
        self.client
            .get_signed(API::Spot(Spot::MyTrades), Some(request))
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
            bail!("Start time should be less than the current time");
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
            bail!("End time should be greater than start time");
        }
        if !is_start_time_valid(&start_time) {
            bail!("Start time should be less than the current time");
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

    fn build_order(
        &self, order: OrderRequest, request_params: Option<BTreeMap<String, String>>,
    ) -> BTreeMap<String, String> {
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
        } else {
            let uuid = uuid_spot();
            order_parameters.insert("newClientOrderId".into(), uuid);
        }

        if let Some(params) = request_params {
            for (key, value) in params {
                order_parameters.insert(key, value);
            }
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
        } else {
            let uuid = uuid_spot();
            order_parameters.insert("newClientOrderId".into(), uuid);
        }
        order_parameters
    }
}
