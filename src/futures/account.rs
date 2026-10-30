use std::collections::BTreeMap;
use std::fmt::Display;

use error_chain::bail;

use crate::util::{build_signed_request, json_string_array, uuid_futures};
use crate::errors::Result;
use crate::client::Client;
use crate::api::{API, Futures};
use crate::model::Empty;
use crate::account::OrderSide;
use crate::futures::model::{Order, TradeHistory};

use super::model::{
    AccountBalance, AccountConfig, AccountInformation, AccountInformationV3, AdlQuantile,
    AlgoOrder, ApiTradingStatus, BatchCancelResponse, BatchOrderResponse, CanceledAlgoOrder,
    CanceledOrder, ChangeLeverageResponse, CodeMsg, CommissionRate, CountdownCancelAll, FeeBurn,
    ForceOrder, LeverageBracket, ModifiedOrder, MultiAssetsMargin, OrderAmendment, OrderRateLimit,
    PositionMarginChange, PositionMode, PositionRisk, PositionRiskV3, SymbolConfig, Transaction,
};

#[derive(Clone)]
pub struct FuturesAccount {
    pub client: Client,
    pub recv_window: u64,
}

pub enum ContractType {
    Perpetual,
    CurrentMonth,
    NextMonth,
    CurrentQuarter,
    NextQuarter,
    /// Traditional finance perpetual (stocks, FX, commodities)
    TradfiPerpetual,
}

impl From<ContractType> for String {
    fn from(item: ContractType) -> Self {
        match item {
            ContractType::Perpetual => String::from("PERPETUAL"),
            ContractType::CurrentMonth => String::from("CURRENT_MONTH"),
            ContractType::NextMonth => String::from("NEXT_MONTH"),
            ContractType::CurrentQuarter => String::from("CURRENT_QUARTER"),
            ContractType::NextQuarter => String::from("NEXT_QUARTER"),
            ContractType::TradfiPerpetual => String::from("TRADIFI_PERPETUAL"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PositionSide {
    Both,
    Long,
    Short,
}

/// Algo order type. Conditional orders are the only algo type currently exposed by the
/// USDⓈ-M futures Algo service.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AlgoType {
    Conditional,
}

impl Display for AlgoType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Conditional => write!(f, "CONDITIONAL"),
        }
    }
}

impl Display for PositionSide {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Both => write!(f, "BOTH"),
            Self::Long => write!(f, "LONG"),
            Self::Short => write!(f, "SHORT"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum OrderType {
    Limit,
    Market,
    Stop,
    StopMarket,
    TakeProfit,
    TakeProfitMarket,
    TrailingStopMarket,
}

impl OrderType {
    /// Conditional order types are handled by the Algo service (`/fapi/v1/algoOrder`)
    /// since 2025-12-09; `POST /fapi/v1/order` rejects them with error `-4120`.
    pub fn is_conditional(&self) -> bool {
        matches!(
            self,
            Self::Stop |
                Self::StopMarket |
                Self::TakeProfit |
                Self::TakeProfitMarket |
                Self::TrailingStopMarket
        )
    }
}

impl Display for OrderType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Limit => write!(f, "LIMIT"),
            Self::Market => write!(f, "MARKET"),
            Self::Stop => write!(f, "STOP"),
            Self::StopMarket => write!(f, "STOP_MARKET"),
            Self::TakeProfit => write!(f, "TAKE_PROFIT"),
            Self::TakeProfitMarket => write!(f, "TAKE_PROFIT_MARKET"),
            Self::TrailingStopMarket => write!(f, "TRAILING_STOP_MARKET"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WorkingType {
    MarkPrice,
    ContractPrice,
}

impl Display for WorkingType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::MarkPrice => write!(f, "MARK_PRICE"),
            Self::ContractPrice => write!(f, "CONTRACT_PRICE"),
        }
    }
}

#[allow(clippy::all)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TimeInForce {
    /// Good Till Cancel
    GTC,
    /// Immediate or Cancel
    IOC,
    ///  Fill or Kill
    FOK,
    /// Good Till Crossing (post only)
    GTX,
    /// Good Till Date
    GTD,
    /// Retail Price Improvement: post-only order that only matches against retail flow
    RPI,
}

impl Display for TimeInForce {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::GTC => write!(f, "GTC"),
            Self::IOC => write!(f, "IOC"),
            Self::FOK => write!(f, "FOK"),
            Self::GTX => write!(f, "GTX"),
            Self::GTD => write!(f, "GTD"),
            Self::RPI => write!(f, "RPI"),
        }
    }
}

/// Price match mode: let the engine pick the price relative to the best opposite
/// (`OPPONENT*`) or same-side (`QUEUE*`) order. Cannot be combined with `price`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PriceMatch {
    Opponent,
    Opponent5,
    Opponent10,
    Opponent20,
    Queue,
    Queue5,
    Queue10,
    Queue20,
}

impl Display for PriceMatch {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Opponent => write!(f, "OPPONENT"),
            Self::Opponent5 => write!(f, "OPPONENT_5"),
            Self::Opponent10 => write!(f, "OPPONENT_10"),
            Self::Opponent20 => write!(f, "OPPONENT_20"),
            Self::Queue => write!(f, "QUEUE"),
            Self::Queue5 => write!(f, "QUEUE_5"),
            Self::Queue10 => write!(f, "QUEUE_10"),
            Self::Queue20 => write!(f, "QUEUE_20"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SelfTradePreventionMode {
    None,
    ExpireTaker,
    ExpireMaker,
    ExpireBoth,
}

impl Display for SelfTradePreventionMode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::None => write!(f, "NONE"),
            Self::ExpireTaker => write!(f, "EXPIRE_TAKER"),
            Self::ExpireMaker => write!(f, "EXPIRE_MAKER"),
            Self::ExpireBoth => write!(f, "EXPIRE_BOTH"),
        }
    }
}

/// Full order description used by [`FuturesAccount::custom_order`],
/// [`FuturesAccount::custom_algo_order`] and [`FuturesAccount::custom_batch_orders`].
///
/// Use [`CustomOrderRequest::new`] and set the optional fields you need.
#[derive(Debug, Clone)]
pub struct CustomOrderRequest {
    pub symbol: String,
    pub side: OrderSide,
    /// Default `BOTH` for One-way Mode ; `LONG` or `SHORT` for Hedge Mode. \
    /// It must be sent in Hedge Mode.
    pub position_side: Option<PositionSide>,
    pub order_type: OrderType,
    pub time_in_force: Option<TimeInForce>,
    /// Cannot be sent with `closePosition`=true(Close-All)
    pub qty: Option<f64>,
    pub reduce_only: Option<bool>,
    pub price: Option<f64>,
    /// Trigger price, used with `STOP`/`STOP_MARKET` or `TAKE_PROFIT`/`TAKE_PROFIT_MARKET`
    /// orders (sent as `triggerPrice` to the Algo service).
    pub stop_price: Option<f64>,
    /// Close-All，used with `STOP_MARKET` or `TAKE_PROFIT_MARKET`.
    pub close_position: Option<bool>,
    /// Used with `TRAILING_STOP_MARKET` orders, default as the latest price(supporting different `workingType`)
    pub activation_price: Option<f64>,
    /// Used with `TRAILING_STOP_MARKET` orders, min `0.1`, max `20` where `1` for `1%`
    pub callback_rate: Option<f64>,
    /// `stopPrice` triggered by: `MARK_PRICE`, `CONTRACT_PRICE`. Default `CONTRACT_PRICE`
    pub working_type: Option<WorkingType>,
    /// Price protection for conditional orders
    pub price_protect: Option<bool>,
    /// A unique id among open orders. \
    /// Automatically generated if not sent. \
    /// Can only be string following the rule: `^[\.A-Z\:/a-z0-9_-]{1,36}$`
    pub new_client_order_id: Option<String>,
    /// Client id of a conditional order. Falls back to `new_client_order_id` (or an auto
    /// generated id) when sent to the Algo service.
    pub client_algo_id: Option<String>,
    /// order cancel time for timeInForce **GTD**, mandatory when [TimeInForce] set to **GTD** \
    /// order the timestamp only retains second-level precision, **ms part will be ignored** \
    /// The goodTillDate timestamp must be greater than the current time **plus 600 seconds** and smaller than **253402300799000**.
    /// the timestamp should be in `UTC` timezone
    pub good_till_date: Option<u64>,
    /// Only for `LIMIT`/`STOP`/`TAKE_PROFIT` orders; cannot be combined with `price`
    pub price_match: Option<PriceMatch>,
    pub self_trade_prevention_mode: Option<SelfTradePreventionMode>,
    /// `ACK` (default) or `RESULT`
    pub new_order_resp_type: Option<String>,
}

impl CustomOrderRequest {
    pub fn new<S: Into<String>>(symbol: S, side: OrderSide, order_type: OrderType) -> Self {
        CustomOrderRequest {
            symbol: symbol.into(),
            side,
            position_side: None,
            order_type,
            time_in_force: None,
            qty: None,
            reduce_only: None,
            price: None,
            stop_price: None,
            close_position: None,
            activation_price: None,
            callback_rate: None,
            working_type: None,
            price_protect: None,
            new_client_order_id: None,
            client_algo_id: None,
            good_till_date: None,
            price_match: None,
            self_trade_prevention_mode: None,
            new_order_resp_type: None,
        }
    }
}

/// `PUT /fapi/v1/order` request. Either `order_id` or `orig_client_order_id` is required;
/// both `quantity` and `price` (or `price_match`) must be sent.
#[derive(Debug, Clone)]
pub struct ModifyOrderRequest {
    pub symbol: String,
    pub side: OrderSide,
    pub order_id: Option<u64>,
    pub orig_client_order_id: Option<String>,
    pub quantity: f64,
    pub price: Option<f64>,
    pub price_match: Option<PriceMatch>,
    /// User-defined id echoed back in the response and in `ORDER_TRADE_UPDATE`
    pub modify_id: Option<i64>,
    /// When `true` and the original order is reduce-only, the `MIN_NOTIONAL` check is skipped
    pub reduce_only: Option<bool>,
}

pub struct IncomeRequest {
    pub symbol: Option<String>,
    pub income_type: Option<IncomeType>,
    pub start_time: Option<u64>,
    pub end_time: Option<u64>,
    pub limit: Option<u32>,
}

#[allow(non_camel_case_types)]
pub enum IncomeType {
    TRANSFER,
    WELCOME_BONUS,
    REALIZED_PNL,
    FUNDING_FEE,
    /// TradFi dividend funding fee
    SPECIAL_FUNDING_FEE,
    COMMISSION,
    INSURANCE_CLEAR,
    REFERRAL_KICKBACK,
    COMMISSION_REBATE,
    API_REBATE,
    CONTEST_REWARD,
    CROSS_COLLATERAL_TRANSFER,
    OPTIONS_PREMIUM_FEE,
    OPTIONS_SETTLE_PROFIT,
    INTERNAL_TRANSFER,
    AUTO_EXCHANGE,
    DELIVERED_SETTELMENT,
    COIN_SWAP_DEPOSIT,
    COIN_SWAP_WITHDRAW,
    POSITION_LIMIT_INCREASE_FEE,
    STRATEGY_UMFUTURES_TRANSFER,
    FEE_RETURN,
    BFUSD_REWARD,
}

impl Display for IncomeType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::TRANSFER => write!(f, "TRANSFER"),
            Self::WELCOME_BONUS => write!(f, "WELCOME_BONUS"),
            Self::REALIZED_PNL => write!(f, "REALIZED_PNL"),
            Self::FUNDING_FEE => write!(f, "FUNDING_FEE"),
            Self::SPECIAL_FUNDING_FEE => write!(f, "SPECIAL_FUNDING_FEE"),
            Self::COMMISSION => write!(f, "COMMISSION"),
            Self::INSURANCE_CLEAR => write!(f, "INSURANCE_CLEAR"),
            Self::REFERRAL_KICKBACK => write!(f, "REFERRAL_KICKBACK"),
            Self::COMMISSION_REBATE => write!(f, "COMMISSION_REBATE"),
            Self::API_REBATE => write!(f, "API_REBATE"),
            Self::CONTEST_REWARD => write!(f, "CONTEST_REWARD"),
            Self::CROSS_COLLATERAL_TRANSFER => write!(f, "CROSS_COLLATERAL_TRANSFER"),
            Self::OPTIONS_PREMIUM_FEE => write!(f, "OPTIONS_PREMIUM_FEE"),
            Self::OPTIONS_SETTLE_PROFIT => write!(f, "OPTIONS_SETTLE_PROFIT"),
            Self::INTERNAL_TRANSFER => write!(f, "INTERNAL_TRANSFER"),
            Self::AUTO_EXCHANGE => write!(f, "AUTO_EXCHANGE"),
            Self::DELIVERED_SETTELMENT => write!(f, "DELIVERED_SETTELMENT"),
            Self::COIN_SWAP_DEPOSIT => write!(f, "COIN_SWAP_DEPOSIT"),
            Self::COIN_SWAP_WITHDRAW => write!(f, "COIN_SWAP_WITHDRAW"),
            Self::POSITION_LIMIT_INCREASE_FEE => write!(f, "POSITION_LIMIT_INCREASE_FEE"),
            Self::STRATEGY_UMFUTURES_TRANSFER => write!(f, "STRATEGY_UMFUTURES_TRANSFER"),
            Self::FEE_RETURN => write!(f, "FEE_RETURN"),
            Self::BFUSD_REWARD => write!(f, "BFUSD_REWARD"),
        }
    }
}

impl FuturesAccount {
    pub fn limit_buy(
        &self, symbol: impl Into<String>, qty: impl Into<f64>, price: f64,
        time_in_force: TimeInForce,
    ) -> Result<Transaction> {
        let mut buy = CustomOrderRequest::new(symbol, OrderSide::Buy, OrderType::Limit);
        buy.time_in_force = Some(time_in_force);
        buy.qty = Some(qty.into());
        buy.price = Some(price);
        self.custom_order(buy)
    }

    pub fn limit_sell(
        &self, symbol: impl Into<String>, qty: impl Into<f64>, price: f64,
        time_in_force: TimeInForce,
    ) -> Result<Transaction> {
        let mut sell = CustomOrderRequest::new(symbol, OrderSide::Sell, OrderType::Limit);
        sell.time_in_force = Some(time_in_force);
        sell.qty = Some(qty.into());
        sell.price = Some(price);
        self.custom_order(sell)
    }

    // Place a MARKET order - BUY
    pub fn market_buy<S, F>(&self, symbol: S, qty: F) -> Result<Transaction>
    where
        S: Into<String>,
        F: Into<f64>,
    {
        let mut buy = CustomOrderRequest::new(symbol, OrderSide::Buy, OrderType::Market);
        buy.qty = Some(qty.into());
        self.custom_order(buy)
    }

    // Place a MARKET order - SELL
    pub fn market_sell<S, F>(&self, symbol: S, qty: F) -> Result<Transaction>
    where
        S: Into<String>,
        F: Into<f64>,
    {
        let mut sell = CustomOrderRequest::new(symbol, OrderSide::Sell, OrderType::Market);
        sell.qty = Some(qty.into());
        self.custom_order(sell)
    }

    pub fn cancel_order<S>(&self, symbol: S, order_id: u64) -> Result<CanceledOrder>
    where
        S: Into<String>,
    {
        let mut parameters = BTreeMap::new();
        parameters.insert("symbol".into(), symbol.into());
        parameters.insert("orderId".into(), order_id.to_string());

        let request = build_signed_request(parameters, self.recv_window)?;
        self.client
            .delete_signed(API::Futures(Futures::Order), Some(request))
    }

    pub fn cancel_order_with_client_id<S>(
        &self, symbol: S, orig_client_order_id: String,
    ) -> Result<CanceledOrder>
    where
        S: Into<String>,
    {
        let mut parameters = BTreeMap::new();
        parameters.insert("symbol".into(), symbol.into());
        parameters.insert("origClientOrderId".into(), orig_client_order_id);

        let request = build_signed_request(parameters, self.recv_window)?;
        self.client
            .delete_signed(API::Futures(Futures::Order), Some(request))
    }

    /// Cancel several orders at once (`DELETE /fapi/v1/batchOrders`), max 10 ids.
    pub fn cancel_batch_orders<S>(
        &self, symbol: S, order_ids: &[u64],
    ) -> Result<Vec<BatchCancelResponse>>
    where
        S: Into<String>,
    {
        let mut parameters = BTreeMap::new();
        parameters.insert("symbol".into(), symbol.into());
        parameters.insert(
            "orderIdList".into(),
            serde_json::Value::Array(
                order_ids
                    .iter()
                    .map(|id| serde_json::Value::from(*id))
                    .collect(),
            )
            .to_string(),
        );
        let request = build_signed_request(parameters, self.recv_window)?;
        self.client
            .delete_signed(API::Futures(Futures::BatchOrders), Some(request))
    }

    /// Cancel several orders by client order id (`DELETE /fapi/v1/batchOrders`), max 10 ids.
    pub fn cancel_batch_orders_with_client_ids<S, C>(
        &self, symbol: S, orig_client_order_ids: &[C],
    ) -> Result<Vec<BatchCancelResponse>>
    where
        S: Into<String>,
        C: AsRef<str>,
    {
        let mut parameters = BTreeMap::new();
        parameters.insert("symbol".into(), symbol.into());
        parameters.insert(
            "origClientOrderIdList".into(),
            json_string_array(orig_client_order_ids),
        );
        let request = build_signed_request(parameters, self.recv_window)?;
        self.client
            .delete_signed(API::Futures(Futures::BatchOrders), Some(request))
    }

    /// Cancel all open orders of a symbol after `countdown_time_ms` milliseconds unless the
    /// countdown is refreshed (`POST /fapi/v1/countdownCancelAll`). `0` cancels the timer.
    pub fn countdown_cancel_all<S>(
        &self, symbol: S, countdown_time_ms: u64,
    ) -> Result<CountdownCancelAll>
    where
        S: Into<String>,
    {
        let mut parameters = BTreeMap::new();
        parameters.insert("symbol".into(), symbol.into());
        parameters.insert("countdownTime".into(), countdown_time_ms.to_string());
        let request = build_signed_request(parameters, self.recv_window)?;
        self.client
            .post_signed(API::Futures(Futures::CountdownCancelAll), request)
    }

    /// Place a STOP_MARKET close-all order - BUY (closes a short position).
    ///
    /// Conditional orders are placed through the Algo service and identified by `algo_id`.
    pub fn stop_market_close_buy<S, F>(&self, symbol: S, stop_price: F) -> Result<AlgoOrder>
    where
        S: Into<String>,
        F: Into<f64>,
    {
        let mut order = CustomOrderRequest::new(symbol, OrderSide::Buy, OrderType::StopMarket);
        order.stop_price = Some(stop_price.into());
        order.close_position = Some(true);
        self.custom_algo_order(order)
    }

    /// Place a STOP_MARKET close-all order - SELL (closes a long position).
    ///
    /// Conditional orders are placed through the Algo service and identified by `algo_id`.
    pub fn stop_market_close_sell<S, F>(&self, symbol: S, stop_price: F) -> Result<AlgoOrder>
    where
        S: Into<String>,
        F: Into<f64>,
    {
        let mut order = CustomOrderRequest::new(symbol, OrderSide::Sell, OrderType::StopMarket);
        order.stop_price = Some(stop_price.into());
        order.close_position = Some(true);
        self.custom_algo_order(order)
    }

    /// Place a `LIMIT` or `MARKET` order (`POST /fapi/v1/order`).
    ///
    /// Conditional order types must go through [`FuturesAccount::custom_algo_order`];
    /// this method refuses them instead of letting Binance answer `-4120`.
    pub fn custom_order(&self, order_request: CustomOrderRequest) -> Result<Transaction> {
        self.custom_order_with_params(order_request, BTreeMap::new())
    }

    /// Same as [`FuturesAccount::custom_order`] with extra raw request parameters.
    pub fn custom_order_with_params(
        &self, order_request: CustomOrderRequest, request_params: BTreeMap<String, String>,
    ) -> Result<Transaction> {
        if order_request.order_type.is_conditional() {
            bail!(format!(
                "{} orders are conditional and must be placed with custom_algo_order",
                order_request.order_type
            ));
        }
        let order = Self::build_order(order_request, Some(request_params));
        let request = build_signed_request(order, self.recv_window)?;
        self.client
            .post_signed(API::Futures(Futures::Order), request)
    }

    /// Place a conditional order (`STOP`, `STOP_MARKET`, `TAKE_PROFIT`, `TAKE_PROFIT_MARKET`,
    /// `TRAILING_STOP_MARKET`) through the Algo service (`POST /fapi/v1/algoOrder`).
    pub fn custom_algo_order(&self, order_request: CustomOrderRequest) -> Result<AlgoOrder> {
        self.custom_algo_order_with_params(order_request, BTreeMap::new())
    }

    /// Same as [`FuturesAccount::custom_algo_order`] with extra raw request parameters.
    pub fn custom_algo_order_with_params(
        &self, order_request: CustomOrderRequest, request_params: BTreeMap<String, String>,
    ) -> Result<AlgoOrder> {
        if !order_request.order_type.is_conditional() {
            bail!(format!(
                "{} orders are not conditional and must be placed with custom_order",
                order_request.order_type
            ));
        }
        let order = Self::build_algo_order(order_request, Some(request_params));
        let request = build_signed_request(order, self.recv_window)?;
        self.client
            .post_signed(API::Futures(Futures::AlgoOrder), request)
    }

    /// Place up to 5 `LIMIT`/`MARKET` orders in one request (`POST /fapi/v1/batchOrders`).
    ///
    /// Results come back in the same order as the requests; each entry is either the
    /// placed order or the error for that order.
    pub fn custom_batch_orders(
        &self, order_requests: Vec<CustomOrderRequest>,
    ) -> Result<Vec<BatchOrderResponse>> {
        self.custom_batch_orders_with_params(order_requests, BTreeMap::new())
    }

    /// Same as [`FuturesAccount::custom_batch_orders`] with extra raw request parameters
    /// applied to every order of the batch.
    pub fn custom_batch_orders_with_params(
        &self, order_requests: Vec<CustomOrderRequest>, request_params: BTreeMap<String, String>,
    ) -> Result<Vec<BatchOrderResponse>> {
        if order_requests.is_empty() || order_requests.len() > 5 {
            bail!("batchOrders accepts between 1 and 5 orders");
        }
        let mut batch = Vec::with_capacity(order_requests.len());
        for order_request in order_requests {
            if order_request.order_type.is_conditional() {
                bail!(format!(
                    "{} orders are conditional and cannot be placed in a batch",
                    order_request.order_type
                ));
            }
            let order = Self::build_order(order_request, Some(request_params.clone()));
            batch.push(serde_json::Value::Object(
                order
                    .into_iter()
                    .map(|(k, v)| (k, serde_json::Value::String(v)))
                    .collect(),
            ));
        }
        let mut parameters = BTreeMap::new();
        parameters.insert(
            "batchOrders".into(),
            serde_json::Value::Array(batch).to_string(),
        );
        let request = build_signed_request(parameters, self.recv_window)?;
        self.client
            .post_signed(API::Futures(Futures::BatchOrders), request)
    }

    /// Modify the price and/or quantity of an open `LIMIT` order (`PUT /fapi/v1/order`).
    pub fn modify_order(&self, req: ModifyOrderRequest) -> Result<ModifiedOrder> {
        let mut parameters = BTreeMap::new();
        parameters.insert("symbol".into(), req.symbol);
        parameters.insert("side".into(), req.side.to_string());
        parameters.insert("quantity".into(), req.quantity.to_string());
        if let Some(id) = req.order_id {
            parameters.insert("orderId".into(), id.to_string());
        }
        if let Some(id) = req.orig_client_order_id {
            parameters.insert("origClientOrderId".into(), id);
        }
        if let Some(price) = req.price {
            parameters.insert("price".into(), price.to_string());
        }
        if let Some(pm) = req.price_match {
            parameters.insert("priceMatch".into(), pm.to_string());
        }
        if let Some(id) = req.modify_id {
            parameters.insert("modifyId".into(), id.to_string());
        }
        if let Some(ro) = req.reduce_only {
            parameters.insert("reduceOnly".into(), ro.to_string());
        }
        let request = build_signed_request(parameters, self.recv_window)?;
        self.client
            .put_signed(API::Futures(Futures::Order), request)
    }

    /// Modification history of an order (`GET /fapi/v1/orderAmendment`).
    pub fn get_order_amendments<S, O, C, T1, T2, L>(
        &self, symbol: S, order_id: O, orig_client_order_id: C, start_time: T1, end_time: T2,
        limit: L,
    ) -> Result<Vec<OrderAmendment>>
    where
        S: Into<String>,
        O: Into<Option<u64>>,
        C: Into<Option<String>>,
        T1: Into<Option<u64>>,
        T2: Into<Option<u64>>,
        L: Into<Option<u16>>,
    {
        let mut parameters = BTreeMap::new();
        parameters.insert("symbol".into(), symbol.into());
        if let Some(id) = order_id.into() {
            parameters.insert("orderId".into(), id.to_string());
        }
        if let Some(id) = orig_client_order_id.into() {
            parameters.insert("origClientOrderId".into(), id);
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
            .get_signed(API::Futures(Futures::OrderAmendment), Some(request))
    }

    /// Query an order (`GET /fapi/v1/order`).
    pub fn order_status<S>(&self, symbol: S, order_id: u64) -> Result<Order>
    where
        S: Into<String>,
    {
        let mut parameters = BTreeMap::new();
        parameters.insert("symbol".into(), symbol.into());
        parameters.insert("orderId".into(), order_id.to_string());
        let request = build_signed_request(parameters, self.recv_window)?;
        self.client
            .get_signed(API::Futures(Futures::Order), Some(request))
    }

    /// Query an order by client order id (`GET /fapi/v1/order`).
    pub fn order_status_with_client_id<S>(
        &self, symbol: S, orig_client_order_id: String,
    ) -> Result<Order>
    where
        S: Into<String>,
    {
        let mut parameters = BTreeMap::new();
        parameters.insert("symbol".into(), symbol.into());
        parameters.insert("origClientOrderId".into(), orig_client_order_id);
        let request = build_signed_request(parameters, self.recv_window)?;
        self.client
            .get_signed(API::Futures(Futures::Order), Some(request))
    }

    /// Query a currently open order (`GET /fapi/v1/openOrder`); returns an error if the
    /// order is filled or canceled.
    pub fn get_open_order<S>(&self, symbol: S, order_id: u64) -> Result<Order>
    where
        S: Into<String>,
    {
        let mut parameters = BTreeMap::new();
        parameters.insert("symbol".into(), symbol.into());
        parameters.insert("orderId".into(), order_id.to_string());
        let request = build_signed_request(parameters, self.recv_window)?;
        self.client
            .get_signed(API::Futures(Futures::OpenOrder), Some(request))
    }

    /// Cancel a conditional order by `algoId` (`DELETE /fapi/v1/algoOrder`).
    pub fn cancel_algo_order(&self, algo_id: u64) -> Result<CanceledAlgoOrder> {
        let mut parameters = BTreeMap::new();
        parameters.insert("algoId".into(), algo_id.to_string());
        let request = build_signed_request(parameters, self.recv_window)?;
        self.client
            .delete_signed(API::Futures(Futures::AlgoOrder), Some(request))
    }

    /// Cancel a conditional order by `clientAlgoId` (`DELETE /fapi/v1/algoOrder`).
    pub fn cancel_algo_order_with_client_id<S>(
        &self, client_algo_id: S,
    ) -> Result<CanceledAlgoOrder>
    where
        S: Into<String>,
    {
        let mut parameters = BTreeMap::new();
        parameters.insert("clientAlgoId".into(), client_algo_id.into());
        let request = build_signed_request(parameters, self.recv_window)?;
        self.client
            .delete_signed(API::Futures(Futures::AlgoOrder), Some(request))
    }

    /// Cancel all open conditional orders of a symbol (`DELETE /fapi/v1/algoOpenOrders`).
    pub fn cancel_all_open_algo_orders<S>(&self, symbol: S) -> Result<CodeMsg>
    where
        S: Into<String>,
    {
        let mut parameters = BTreeMap::new();
        parameters.insert("symbol".into(), symbol.into());
        let request = build_signed_request(parameters, self.recv_window)?;
        self.client
            .delete_signed(API::Futures(Futures::AlgoOpenOrders), Some(request))
    }

    /// Query a conditional order by `algoId` (`GET /fapi/v1/algoOrder`).
    pub fn get_algo_order(&self, algo_id: u64) -> Result<AlgoOrder> {
        let mut parameters = BTreeMap::new();
        parameters.insert("algoId".into(), algo_id.to_string());
        let request = build_signed_request(parameters, self.recv_window)?;
        self.client
            .get_signed(API::Futures(Futures::AlgoOrder), Some(request))
    }

    /// Query a conditional order by `clientAlgoId` (`GET /fapi/v1/algoOrder`).
    pub fn get_algo_order_with_client_id<S>(&self, client_algo_id: S) -> Result<AlgoOrder>
    where
        S: Into<String>,
    {
        let mut parameters = BTreeMap::new();
        parameters.insert("clientAlgoId".into(), client_algo_id.into());
        let request = build_signed_request(parameters, self.recv_window)?;
        self.client
            .get_signed(API::Futures(Futures::AlgoOrder), Some(request))
    }

    /// Open conditional orders (`GET /fapi/v1/openAlgoOrders`), all symbols when `None`.
    pub fn get_open_algo_orders<S>(&self, symbol: S) -> Result<Vec<AlgoOrder>>
    where
        S: Into<Option<String>>,
    {
        let mut parameters = BTreeMap::new();
        parameters.insert("algoType".into(), AlgoType::Conditional.to_string());
        if let Some(symbol) = symbol.into() {
            parameters.insert("symbol".into(), symbol);
        }
        let request = build_signed_request(parameters, self.recv_window)?;
        self.client
            .get_signed(API::Futures(Futures::OpenAlgoOrders), Some(request))
    }

    /// All conditional orders of a symbol (`GET /fapi/v1/allAlgoOrders`).
    ///
    /// If `algo_id` is set, orders >= that id are returned; otherwise the most recent ones.
    pub fn get_all_algo_orders<S, A, T1, T2, L>(
        &self, symbol: S, algo_id: A, start_time: T1, end_time: T2, limit: L,
    ) -> Result<Vec<AlgoOrder>>
    where
        S: Into<String>,
        A: Into<Option<u64>>,
        T1: Into<Option<u64>>,
        T2: Into<Option<u64>>,
        L: Into<Option<u16>>,
    {
        let mut parameters = BTreeMap::new();
        parameters.insert("symbol".into(), symbol.into());
        if let Some(id) = algo_id.into() {
            parameters.insert("algoId".into(), id.to_string());
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
            .get_signed(API::Futures(Futures::AllAlgoOrders), Some(request))
    }

    pub fn get_all_orders<S, F, N>(
        &self, symbol: S, order_id: F, start_time: F, end_time: F, limit: N,
    ) -> Result<Vec<Order>>
    where
        S: Into<String>,
        F: Into<Option<u64>>,
        N: Into<Option<u16>>,
    {
        let mut parameters = BTreeMap::new();
        parameters.insert("symbol".into(), symbol.into());
        if let Some(order_id) = order_id.into() {
            parameters.insert("orderId".into(), order_id.to_string());
        }
        if let Some(start_time) = start_time.into() {
            parameters.insert("startTime".into(), start_time.to_string());
        }
        if let Some(end_time) = end_time.into() {
            parameters.insert("endTime".into(), end_time.to_string());
        }
        if let Some(limit) = limit.into() {
            parameters.insert("limit".into(), limit.to_string());
        }

        let request = build_signed_request(parameters, self.recv_window)?;
        self.client
            .get_signed(API::Futures(Futures::AllOrders), Some(request))
    }

    pub fn get_user_trades<S, F, N>(
        &self, symbol: S, from_id: F, start_time: F, end_time: F, limit: N,
    ) -> Result<Vec<TradeHistory>>
    where
        S: Into<String>,
        F: Into<Option<u64>>,
        N: Into<Option<u16>>,
    {
        let mut parameters = BTreeMap::new();
        parameters.insert("symbol".into(), symbol.into());
        if let Some(order_id) = from_id.into() {
            parameters.insert("fromId".into(), order_id.to_string());
        }
        if let Some(start_time) = start_time.into() {
            parameters.insert("startTime".into(), start_time.to_string());
        }
        if let Some(end_time) = end_time.into() {
            parameters.insert("endTime".into(), end_time.to_string());
        }
        if let Some(limit) = limit.into() {
            parameters.insert("limit".into(), limit.to_string());
        }

        let request = build_signed_request(parameters, self.recv_window)?;
        self.client
            .get_signed(API::Futures(Futures::UserTrades), Some(request))
    }

    /// The account's liquidation / ADL orders (`GET /fapi/v1/forceOrders`), last 90 days.
    ///
    /// `auto_close_type`: `LIQUIDATION` or `ADL`; both when `None`.
    pub fn get_force_orders<S, A, T1, T2, L>(
        &self, symbol: S, auto_close_type: A, start_time: T1, end_time: T2, limit: L,
    ) -> Result<Vec<ForceOrder>>
    where
        S: Into<Option<String>>,
        A: Into<Option<String>>,
        T1: Into<Option<u64>>,
        T2: Into<Option<u64>>,
        L: Into<Option<u16>>,
    {
        let mut parameters = BTreeMap::new();
        if let Some(symbol) = symbol.into() {
            parameters.insert("symbol".into(), symbol);
        }
        if let Some(t) = auto_close_type.into() {
            parameters.insert("autoCloseType".into(), t);
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
            .get_signed(API::Futures(Futures::ForceOrders), Some(request))
    }

    /// Parameters for `POST /fapi/v1/order` and `POST /fapi/v1/batchOrders`.
    fn build_order(
        order: CustomOrderRequest, request_params: Option<BTreeMap<String, String>>,
    ) -> BTreeMap<String, String> {
        let mut parameters = BTreeMap::new();
        parameters.insert("symbol".into(), order.symbol);
        parameters.insert("side".into(), order.side.to_string());
        parameters.insert("type".into(), order.order_type.to_string());

        if let Some(position_side) = order.position_side {
            parameters.insert("positionSide".into(), position_side.to_string());
        }
        if let Some(time_in_force) = order.time_in_force {
            parameters.insert("timeInForce".into(), time_in_force.to_string());
        }
        if let Some(qty) = order.qty {
            parameters.insert("quantity".into(), qty.to_string());
        }
        if let Some(reduce_only) = order.reduce_only {
            parameters.insert("reduceOnly".into(), reduce_only.to_string().to_uppercase());
        }
        if let Some(price) = order.price {
            parameters.insert("price".into(), price.to_string());
        }
        if let Some(stop_price) = order.stop_price {
            parameters.insert("stopPrice".into(), stop_price.to_string());
        }
        if let Some(close_position) = order.close_position {
            parameters.insert(
                "closePosition".into(),
                close_position.to_string().to_uppercase(),
            );
        }
        if let Some(activation_price) = order.activation_price {
            parameters.insert("activationPrice".into(), activation_price.to_string());
        }
        if let Some(callback_rate) = order.callback_rate {
            parameters.insert("callbackRate".into(), callback_rate.to_string());
        }
        if let Some(working_type) = order.working_type {
            parameters.insert("workingType".into(), working_type.to_string());
        }
        if let Some(price_protect) = order.price_protect {
            parameters.insert(
                "priceProtect".into(),
                price_protect.to_string().to_uppercase(),
            );
        }
        parameters.insert(
            "newClientOrderId".into(),
            order.new_client_order_id.unwrap_or_else(uuid_futures),
        );
        if let Some(good_till_date) = order.good_till_date {
            parameters.insert("goodTillDate".into(), good_till_date.to_string());
        }
        if let Some(price_match) = order.price_match {
            parameters.insert("priceMatch".into(), price_match.to_string());
        }
        if let Some(stp) = order.self_trade_prevention_mode {
            parameters.insert("selfTradePreventionMode".into(), stp.to_string());
        }
        if let Some(rt) = order.new_order_resp_type {
            parameters.insert("newOrderRespType".into(), rt);
        }

        if let Some(params) = request_params {
            for (key, value) in params {
                parameters.insert(key, value);
            }
        }

        parameters
    }

    /// Parameters for `POST /fapi/v1/algoOrder`: same as a regular order except that the
    /// trigger price is `triggerPrice`, the activation price `activatePrice` and the client
    /// id `clientAlgoId`.
    fn build_algo_order(
        order: CustomOrderRequest, request_params: Option<BTreeMap<String, String>>,
    ) -> BTreeMap<String, String> {
        let mut parameters = BTreeMap::new();
        parameters.insert("algoType".into(), AlgoType::Conditional.to_string());
        parameters.insert("symbol".into(), order.symbol);
        parameters.insert("side".into(), order.side.to_string());
        parameters.insert("type".into(), order.order_type.to_string());

        if let Some(position_side) = order.position_side {
            parameters.insert("positionSide".into(), position_side.to_string());
        }
        if let Some(time_in_force) = order.time_in_force {
            parameters.insert("timeInForce".into(), time_in_force.to_string());
        }
        if let Some(qty) = order.qty {
            parameters.insert("quantity".into(), qty.to_string());
        }
        if let Some(reduce_only) = order.reduce_only {
            parameters.insert("reduceOnly".into(), reduce_only.to_string().to_uppercase());
        }
        if let Some(price) = order.price {
            parameters.insert("price".into(), price.to_string());
        }
        if let Some(stop_price) = order.stop_price {
            parameters.insert("triggerPrice".into(), stop_price.to_string());
        }
        if let Some(close_position) = order.close_position {
            parameters.insert(
                "closePosition".into(),
                close_position.to_string().to_uppercase(),
            );
        }
        if let Some(activation_price) = order.activation_price {
            parameters.insert("activatePrice".into(), activation_price.to_string());
        }
        if let Some(callback_rate) = order.callback_rate {
            parameters.insert("callbackRate".into(), callback_rate.to_string());
        }
        if let Some(working_type) = order.working_type {
            parameters.insert("workingType".into(), working_type.to_string());
        }
        if let Some(price_protect) = order.price_protect {
            parameters.insert(
                "priceProtect".into(),
                price_protect.to_string().to_uppercase(),
            );
        }
        let client_algo_id = order
            .client_algo_id
            .or(order.new_client_order_id)
            .unwrap_or_else(uuid_futures);
        parameters.insert("clientAlgoId".into(), client_algo_id);
        if let Some(good_till_date) = order.good_till_date {
            parameters.insert("goodTillDate".into(), good_till_date.to_string());
        }
        if let Some(price_match) = order.price_match {
            parameters.insert("priceMatch".into(), price_match.to_string());
        }
        if let Some(stp) = order.self_trade_prevention_mode {
            parameters.insert("selfTradePreventionMode".into(), stp.to_string());
        }
        if let Some(rt) = order.new_order_resp_type {
            parameters.insert("newOrderRespType".into(), rt);
        }

        if let Some(params) = request_params {
            for (key, value) in params {
                parameters.insert(key, value);
            }
        }

        parameters
    }

    /// Position information (`GET /fapi/v2/positionRisk`).
    #[deprecated(note = "deprecated by Binance, use position_information_v3")]
    pub fn position_information<S>(&self, symbol: S) -> Result<Vec<PositionRisk>>
    where
        S: Into<String>,
    {
        let mut parameters = BTreeMap::new();
        parameters.insert("symbol".into(), symbol.into());

        let request = build_signed_request(parameters, self.recv_window)?;
        self.client
            .get_signed(API::Futures(Futures::PositionRisk), Some(request))
    }

    /// Position information (`GET /fapi/v3/positionRisk`).
    ///
    /// Only symbols with an open position or open orders are returned; all symbols when
    /// `symbol` is `None`.
    pub fn position_information_v3<S>(&self, symbol: S) -> Result<Vec<PositionRiskV3>>
    where
        S: Into<Option<String>>,
    {
        let mut parameters = BTreeMap::new();
        if let Some(symbol) = symbol.into() {
            parameters.insert("symbol".into(), symbol);
        }
        let request = build_signed_request(parameters, self.recv_window)?;
        self.client
            .get_signed(API::Futures(Futures::PositionRiskV3), Some(request))
    }

    /// Account information (`GET /fapi/v2/account`).
    #[deprecated(note = "deprecated by Binance, use account_information_v3")]
    pub fn account_information(&self) -> Result<AccountInformation> {
        let parameters = BTreeMap::new();

        let request = build_signed_request(parameters, self.recv_window)?;
        self.client
            .get_signed(API::Futures(Futures::Account), Some(request))
    }

    /// Account information (`GET /fapi/v3/account`).
    ///
    /// Only symbols with an open position or open orders are listed in `positions`; the
    /// account/symbol configuration fields of V2 moved to
    /// [`FuturesAccount::get_account_config`] and [`FuturesAccount::get_symbol_config`].
    pub fn account_information_v3(&self) -> Result<AccountInformationV3> {
        let request = build_signed_request(BTreeMap::new(), self.recv_window)?;
        self.client
            .get_signed(API::Futures(Futures::AccountV3), Some(request))
    }

    /// Account balance (`GET /fapi/v2/balance`).
    #[deprecated(note = "deprecated by Binance, use account_balance_v3")]
    pub fn account_balance(&self) -> Result<Vec<AccountBalance>> {
        let parameters = BTreeMap::new();

        let request = build_signed_request(parameters, self.recv_window)?;
        self.client
            .get_signed(API::Futures(Futures::Balance), Some(request))
    }

    /// Account balance (`GET /fapi/v3/balance`).
    pub fn account_balance_v3(&self) -> Result<Vec<AccountBalance>> {
        let request = build_signed_request(BTreeMap::new(), self.recv_window)?;
        self.client
            .get_signed(API::Futures(Futures::BalanceV3), Some(request))
    }

    /// Account configuration (`GET /fapi/v1/accountConfig`): fee tier, position mode,
    /// multi-assets mode, ...
    pub fn get_account_config(&self) -> Result<AccountConfig> {
        let request = build_signed_request(BTreeMap::new(), self.recv_window)?;
        self.client
            .get_signed(API::Futures(Futures::AccountConfig), Some(request))
    }

    /// Symbol configuration (`GET /fapi/v1/symbolConfig`): margin type, leverage, max
    /// notional. All symbols when `symbol` is `None`.
    pub fn get_symbol_config<S>(&self, symbol: S) -> Result<Vec<SymbolConfig>>
    where
        S: Into<Option<String>>,
    {
        let mut parameters = BTreeMap::new();
        if let Some(symbol) = symbol.into() {
            parameters.insert("symbol".into(), symbol);
        }
        let request = build_signed_request(parameters, self.recv_window)?;
        self.client
            .get_signed(API::Futures(Futures::SymbolConfig), Some(request))
    }

    /// User commission rate for a symbol (`GET /fapi/v1/commissionRate`).
    pub fn get_commission_rate<S>(&self, symbol: S) -> Result<CommissionRate>
    where
        S: Into<String>,
    {
        let mut parameters = BTreeMap::new();
        parameters.insert("symbol".into(), symbol.into());
        let request = build_signed_request(parameters, self.recv_window)?;
        self.client
            .get_signed(API::Futures(Futures::CommissionRate), Some(request))
    }

    /// Notional and leverage brackets (`GET /fapi/v1/leverageBracket`). All symbols when
    /// `symbol` is `None`.
    pub fn get_leverage_brackets<S>(&self, symbol: S) -> Result<Vec<LeverageBracket>>
    where
        S: Into<Option<String>>,
    {
        let mut parameters = BTreeMap::new();
        if let Some(symbol) = symbol.into() {
            parameters.insert("symbol".into(), symbol);
        }
        let request = build_signed_request(parameters, self.recv_window)?;
        self.client
            .get_signed(API::Futures(Futures::LeverageBracket), Some(request))
    }

    /// Position ADL quantile estimation (`GET /fapi/v1/adlQuantile`).
    pub fn get_adl_quantile<S>(&self, symbol: S) -> Result<Vec<AdlQuantile>>
    where
        S: Into<Option<String>>,
    {
        let mut parameters = BTreeMap::new();
        if let Some(symbol) = symbol.into() {
            parameters.insert("symbol".into(), symbol);
        }
        let request = build_signed_request(parameters, self.recv_window)?;
        self.client
            .get_signed(API::Futures(Futures::AdlQuantile), Some(request))
    }

    /// Futures trading quantitative rules indicators (`GET /fapi/v1/apiTradingStatus`).
    pub fn get_api_trading_status<S>(&self, symbol: S) -> Result<ApiTradingStatus>
    where
        S: Into<Option<String>>,
    {
        let mut parameters = BTreeMap::new();
        if let Some(symbol) = symbol.into() {
            parameters.insert("symbol".into(), symbol);
        }
        let request = build_signed_request(parameters, self.recv_window)?;
        self.client
            .get_signed(API::Futures(Futures::ApiTradingStatus), Some(request))
    }

    /// Current order count usage (`GET /fapi/v1/rateLimit/order`).
    pub fn get_order_rate_limits(&self) -> Result<Vec<OrderRateLimit>> {
        let request = build_signed_request(BTreeMap::new(), self.recv_window)?;
        self.client
            .get_signed(API::Futures(Futures::RateLimitOrder), Some(request))
    }

    /// Position margin change history (`GET /fapi/v1/positionMargin/history`), last 30 days.
    ///
    /// `change_type`: 1 for added margin, 2 for reduced margin.
    pub fn get_position_margin_history<S, C, T1, T2, L>(
        &self, symbol: S, change_type: C, start_time: T1, end_time: T2, limit: L,
    ) -> Result<Vec<PositionMarginChange>>
    where
        S: Into<String>,
        C: Into<Option<u8>>,
        T1: Into<Option<u64>>,
        T2: Into<Option<u64>>,
        L: Into<Option<u16>>,
    {
        let mut parameters = BTreeMap::new();
        parameters.insert("symbol".into(), symbol.into());
        if let Some(t) = change_type.into() {
            parameters.insert("type".into(), t.to_string());
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
            .get_signed(API::Futures(Futures::PositionMarginHistory), Some(request))
    }

    pub fn change_initial_leverage<S>(
        &self, symbol: S, leverage: u8,
    ) -> Result<ChangeLeverageResponse>
    where
        S: Into<String>,
    {
        let mut parameters: BTreeMap<String, String> = BTreeMap::new();
        parameters.insert("symbol".into(), symbol.into());
        parameters.insert("leverage".into(), leverage.to_string());

        let request = build_signed_request(parameters, self.recv_window)?;
        self.client
            .post_signed(API::Futures(Futures::ChangeInitialLeverage), request)
    }

    pub fn change_margin_type<S>(&self, symbol: S, isolated: bool) -> Result<()>
    where
        S: Into<String>,
    {
        let mut parameters: BTreeMap<String, String> = BTreeMap::new();
        let margin_type = if isolated { "ISOLATED" } else { "CROSSED" };
        parameters.insert("symbol".into(), symbol.into());
        parameters.insert("marginType".into(), margin_type.into());

        let request = build_signed_request(parameters, self.recv_window)?;
        self.client
            .post_signed::<Empty>(API::Futures(Futures::MarginType), request)
            .map(|_| ())
    }

    pub fn change_position_margin<S>(
        &self, symbol: S, amount: f64, is_adding_margin: bool,
    ) -> Result<()>
    where
        S: Into<String>,
    {
        let mut parameters: BTreeMap<String, String> = BTreeMap::new();
        let margin = if is_adding_margin { "1" } else { "2" };
        parameters.insert("symbol".into(), symbol.into());
        parameters.insert("amount".into(), amount.to_string());
        parameters.insert("type".into(), margin.into());

        let request = build_signed_request(parameters, self.recv_window)?;
        self.client
            .post_signed::<Empty>(API::Futures(Futures::PositionMargin), request)
            .map(|_| ())
    }

    /// Switch between One-way (`false`) and Hedge (`true`) position mode.
    pub fn change_position_mode(&self, dual_side_position: bool) -> Result<()> {
        let mut parameters: BTreeMap<String, String> = BTreeMap::new();
        let dual_side = if dual_side_position { "true" } else { "false" };
        parameters.insert("dualSidePosition".into(), dual_side.into());

        let request = build_signed_request(parameters, self.recv_window)?;
        self.client
            .post_signed::<Empty>(API::Futures(Futures::PositionSide), request)
            .map(|_| ())
    }

    /// Current position mode (`GET /fapi/v1/positionSide/dual`).
    pub fn get_position_mode(&self) -> Result<PositionMode> {
        let request = build_signed_request(BTreeMap::new(), self.recv_window)?;
        self.client
            .get_signed(API::Futures(Futures::PositionSide), Some(request))
    }

    /// Switch between Single-Asset (`false`) and Multi-Assets (`true`) margin mode
    /// (`POST /fapi/v1/multiAssetsMargin`).
    pub fn change_multi_assets_margin(&self, multi_assets_margin: bool) -> Result<CodeMsg> {
        let mut parameters: BTreeMap<String, String> = BTreeMap::new();
        parameters.insert("multiAssetsMargin".into(), multi_assets_margin.to_string());
        let request = build_signed_request(parameters, self.recv_window)?;
        self.client
            .post_signed(API::Futures(Futures::MultiAssetsMargin), request)
    }

    /// Current margin mode (`GET /fapi/v1/multiAssetsMargin`).
    pub fn get_multi_assets_margin(&self) -> Result<MultiAssetsMargin> {
        let request = build_signed_request(BTreeMap::new(), self.recv_window)?;
        self.client
            .get_signed(API::Futures(Futures::MultiAssetsMargin), Some(request))
    }

    /// Toggle BNB fee burn on futures trades (`POST /fapi/v1/feeBurn`).
    pub fn set_fee_burn(&self, fee_burn: bool) -> Result<CodeMsg> {
        let mut parameters: BTreeMap<String, String> = BTreeMap::new();
        parameters.insert("feeBurn".into(), fee_burn.to_string());
        let request = build_signed_request(parameters, self.recv_window)?;
        self.client
            .post_signed(API::Futures(Futures::FeeBurn), request)
    }

    /// BNB fee burn status (`GET /fapi/v1/feeBurn`).
    pub fn get_fee_burn(&self) -> Result<FeeBurn> {
        let request = build_signed_request(BTreeMap::new(), self.recv_window)?;
        self.client
            .get_signed(API::Futures(Futures::FeeBurn), Some(request))
    }

    pub fn cancel_all_open_orders<S>(&self, symbol: S) -> Result<()>
    where
        S: Into<String>,
    {
        let mut parameters: BTreeMap<String, String> = BTreeMap::new();
        parameters.insert("symbol".into(), symbol.into());
        let request = build_signed_request(parameters, self.recv_window)?;
        self.client
            .delete_signed::<Empty>(API::Futures(Futures::AllOpenOrders), Some(request))
            .map(|_| ())
    }

    pub fn get_all_open_orders<S>(&self, symbol: S) -> Result<Vec<crate::futures::model::Order>>
    where
        S: Into<String>,
    {
        let mut parameters: BTreeMap<String, String> = BTreeMap::new();
        parameters.insert("symbol".into(), symbol.into());
        let request = build_signed_request(parameters, self.recv_window)?;
        self.client
            .get_signed(API::Futures(Futures::OpenOrders), Some(request))
    }

    pub fn get_income(
        &self, income_request: IncomeRequest,
    ) -> Result<Vec<crate::futures::model::Income>> {
        let mut parameters: BTreeMap<String, String> = BTreeMap::new();
        if let Some(symbol) = income_request.symbol {
            parameters.insert("symbol".into(), symbol);
        }
        if let Some(income_type) = income_request.income_type {
            parameters.insert("incomeType".into(), income_type.to_string());
        }
        if let Some(start_time) = income_request.start_time {
            parameters.insert("startTime".into(), start_time.to_string());
        }
        if let Some(end_time) = income_request.end_time {
            parameters.insert("endTime".into(), end_time.to_string());
        }
        if let Some(limit) = income_request.limit {
            parameters.insert("limit".into(), limit.to_string());
        }

        let request = build_signed_request(parameters, self.recv_window)?;
        self.client
            .get_signed(API::Futures(Futures::Income), Some(request))
    }
}
