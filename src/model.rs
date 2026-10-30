use serde::{Deserialize, Serialize};
use serde_json::{from_value, Value};
use crate::errors::{Error, ErrorKind, Result};

#[derive(Deserialize, Clone)]
pub struct Empty {}

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ServerTime {
    pub server_time: u64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ExchangeInformation {
    pub timezone: String,
    pub server_time: u64,
    pub rate_limits: Vec<RateLimit>,
    #[serde(default)]
    pub exchange_filters: Vec<Filters>,
    pub symbols: Vec<Symbol>,
    /// Smart Order Routing groups, only present when SOR is available
    #[serde(default)]
    pub sors: Option<Vec<Sor>>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Sor {
    pub base_asset: String,
    pub symbols: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct RateLimit {
    pub rate_limit_type: String,
    pub interval: String,
    pub interval_num: u16,
    pub limit: u64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Symbol {
    pub symbol: String,
    pub status: String,
    pub base_asset: String,
    pub base_asset_precision: u64,
    pub quote_asset: String,
    pub quote_precision: u64,
    pub quote_asset_precision: Option<u64>,
    pub base_commission_precision: Option<u64>,
    pub quote_commission_precision: Option<u64>,
    pub order_types: Vec<String>,
    pub iceberg_allowed: bool,
    pub oco_allowed: Option<bool>,
    pub oto_allowed: Option<bool>,
    pub opo_allowed: Option<bool>,
    pub quote_order_qty_market_allowed: Option<bool>,
    pub allow_trailing_stop: Option<bool>,
    pub cancel_replace_allowed: Option<bool>,
    pub amend_allowed: Option<bool>,
    pub peg_instructions_allowed: Option<bool>,
    pub is_spot_trading_allowed: bool,
    pub is_margin_trading_allowed: bool,
    pub filters: Vec<Filters>,
    /// Deprecated by Binance: always empty, use `permission_sets`
    #[serde(default)]
    pub permissions: Vec<String>,
    #[serde(default)]
    pub permission_sets: Vec<Vec<String>>,
    pub default_self_trade_prevention_mode: Option<String>,
    #[serde(default)]
    pub allowed_self_trade_prevention_modes: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(tag = "filterType")]
pub enum Filters {
    #[serde(rename = "PRICE_FILTER")]
    #[serde(rename_all = "camelCase")]
    PriceFilter {
        min_price: String,
        max_price: String,
        tick_size: String,
    },
    #[serde(rename = "PERCENT_PRICE")]
    #[serde(rename_all = "camelCase")]
    PercentPrice {
        multiplier_up: String,
        multiplier_down: String,
        avg_price_mins: Option<f64>,
    },
    #[serde(rename = "POSITION_RISK_CONTROL")]
    #[serde(rename_all = "camelCase")]
    PositionRiskControl { position_control_side: String },
    #[serde(rename = "PERCENT_PRICE_BY_SIDE")]
    #[serde(rename_all = "camelCase")]
    PercentPriceBySide {
        bid_multiplier_up: String,
        bid_multiplier_down: String,
        ask_multiplier_up: String,
        ask_multiplier_down: String,
        avg_price_mins: Option<f64>,
    },
    #[serde(rename = "LOT_SIZE")]
    #[serde(rename_all = "camelCase")]
    LotSize {
        min_qty: String,
        max_qty: String,
        step_size: String,
    },
    #[serde(rename = "MIN_NOTIONAL")]
    #[serde(rename_all = "camelCase")]
    MinNotional {
        notional: Option<String>,
        min_notional: Option<String>,
        apply_to_market: Option<bool>,
        avg_price_mins: Option<f64>,
    },
    #[serde(rename = "NOTIONAL")]
    #[serde(rename_all = "camelCase")]
    Notional {
        notional: Option<String>,
        min_notional: Option<String>,
        apply_to_market: Option<bool>,
        avg_price_mins: Option<f64>,
    },
    #[serde(rename = "ICEBERG_PARTS")]
    #[serde(rename_all = "camelCase")]
    IcebergParts { limit: Option<u16> },
    #[serde(rename = "MAX_NUM_ORDERS")]
    #[serde(rename_all = "camelCase")]
    MaxNumOrders { max_num_orders: Option<u16> },
    #[serde(rename = "MAX_NUM_ALGO_ORDERS")]
    #[serde(rename_all = "camelCase")]
    MaxNumAlgoOrders { max_num_algo_orders: Option<u16> },
    #[serde(rename = "MAX_NUM_ICEBERG_ORDERS")]
    #[serde(rename_all = "camelCase")]
    MaxNumIcebergOrders { max_num_iceberg_orders: u16 },
    #[serde(rename = "MAX_POSITION")]
    #[serde(rename_all = "camelCase")]
    MaxPosition { max_position: String },
    #[serde(rename = "MARKET_LOT_SIZE")]
    #[serde(rename_all = "camelCase")]
    MarketLotSize {
        min_qty: String,
        max_qty: String,
        step_size: String,
    },
    #[serde(rename = "TRAILING_DELTA")]
    #[serde(rename_all = "camelCase")]
    TrailingData {
        min_trailing_above_delta: Option<u16>,
        max_trailing_above_delta: Option<u16>,
        min_trailing_below_delta: Option<u16>,
        max_trailing_below_delta: Option<u16>,
    },
    #[serde(rename = "MAX_NUM_ORDER_LISTS")]
    #[serde(rename_all = "camelCase")]
    MaxNumOrderLists { max_num_order_lists: Option<u16> },
    #[serde(rename = "MAX_NUM_ORDER_AMENDS")]
    #[serde(rename_all = "camelCase")]
    MaxNumOrderAmends { max_num_order_amends: Option<u16> },
    /// Account-level asset filter, only visible through `GET /api/v3/myFilters`
    #[serde(rename = "MAX_ASSET")]
    #[serde(rename_all = "camelCase")]
    MaxAsset {
        limit: Option<String>,
        asset: Option<String>,
    },
    #[serde(rename = "EXCHANGE_MAX_NUM_ORDERS")]
    #[serde(rename_all = "camelCase")]
    ExchangeMaxNumOrders { max_num_orders: Option<u32> },
    #[serde(rename = "EXCHANGE_MAX_NUM_ALGO_ORDERS")]
    #[serde(rename_all = "camelCase")]
    ExchangeMaxNumAlgoOrders { max_num_algo_orders: Option<u32> },
    #[serde(rename = "EXCHANGE_MAX_NUM_ICEBERG_ORDERS")]
    #[serde(rename_all = "camelCase")]
    ExchangeMaxNumIcebergOrders { max_num_iceberg_orders: Option<u32> },
    #[serde(rename = "EXCHANGE_MAX_NUM_ORDER_LISTS")]
    #[serde(rename_all = "camelCase")]
    ExchangeMaxNumOrderLists { max_num_order_lists: Option<u32> },
    /// Any filter type this library does not know about yet.
    ///
    /// Binance regularly adds new filters; this variant keeps `exchangeInfo`
    /// deserialization working instead of failing on the first unknown `filterType`.
    #[serde(other)]
    Unknown,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct AccountInformation {
    pub maker_commission: f32,
    pub taker_commission: f32,
    pub buyer_commission: f32,
    pub seller_commission: f32,
    pub commission_rates: Option<CommissionRates>,
    pub can_trade: bool,
    pub can_withdraw: bool,
    pub can_deposit: bool,
    pub brokered: Option<bool>,
    pub require_self_trade_prevention: Option<bool>,
    pub prevent_sor: Option<bool>,
    pub update_time: Option<u64>,
    pub account_type: Option<String>,
    pub balances: Vec<Balance>,
    #[serde(default)]
    pub permissions: Vec<String>,
    pub uid: Option<u64>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CommissionRates {
    #[serde(with = "string_or_float")]
    pub maker: f64,
    #[serde(with = "string_or_float")]
    pub taker: f64,
    #[serde(with = "string_or_float")]
    pub buyer: f64,
    #[serde(with = "string_or_float")]
    pub seller: f64,
}

/// Response of `GET /api/v3/account/commission`
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct AccountCommission {
    pub symbol: String,
    pub standard_commission: CommissionRates,
    pub special_commission: Option<CommissionRates>,
    pub tax_commission: CommissionRates,
    pub discount: CommissionDiscount,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CommissionDiscount {
    pub enabled_for_account: bool,
    pub enabled_for_symbol: bool,
    pub discount_asset: Option<String>,
    #[serde(with = "string_or_float")]
    pub discount: f64,
}

/// Response of `GET /api/v3/myFilters`
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct MyFilters {
    #[serde(default)]
    pub exchange_filters: Vec<Filters>,
    #[serde(default)]
    pub symbol_filters: Vec<Filters>,
    #[serde(default)]
    pub asset_filters: Vec<Filters>,
    #[serde(default)]
    pub rate_limits: Vec<OrderRateLimit>,
}

/// Rate limit including the current usage count, as returned by
/// `GET /api/v3/rateLimit/order` and `GET /api/v3/myFilters`.
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct OrderRateLimit {
    pub rate_limit_type: String,
    pub interval: String,
    pub interval_num: u16,
    pub limit: u64,
    pub count: Option<u64>,
}

/// Element of the `GET /api/v3/myPreventedMatches` response
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct PreventedMatch {
    pub symbol: String,
    pub prevented_match_id: u64,
    pub taker_order_id: u64,
    pub maker_symbol: Option<String>,
    pub maker_order_id: u64,
    pub trade_group_id: u64,
    pub self_trade_prevention_mode: String,
    #[serde(with = "string_or_float")]
    pub price: f64,
    #[serde(default, with = "string_or_float_opt")]
    pub maker_prevented_quantity: Option<f64>,
    #[serde(default, with = "string_or_float_opt")]
    pub taker_prevented_quantity: Option<f64>,
    pub transact_time: u64,
}

/// Element of the `GET /api/v3/myAllocations` response
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Allocation {
    pub symbol: String,
    pub allocation_id: u64,
    pub allocation_type: String,
    pub order_id: u64,
    pub order_list_id: i64,
    #[serde(with = "string_or_float")]
    pub price: f64,
    #[serde(with = "string_or_float")]
    pub qty: f64,
    #[serde(with = "string_or_float")]
    pub quote_qty: f64,
    #[serde(with = "string_or_float")]
    pub commission: f64,
    pub commission_asset: String,
    pub time: u64,
    pub is_buyer: bool,
    pub is_maker: bool,
    pub is_allocator: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Balance {
    pub asset: String,
    pub free: String,
    pub locked: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Order {
    pub symbol: String,
    pub order_id: u64,
    pub order_list_id: i64,
    pub client_order_id: String,
    #[serde(with = "string_or_float")]
    pub price: f64,
    pub orig_qty: String,
    pub executed_qty: String,
    pub cummulative_quote_qty: String,
    pub status: String,
    pub time_in_force: String,
    #[serde(rename = "type")]
    pub type_name: String,
    pub side: String,
    /// Only present for `STOP_LOSS`, `TAKE_PROFIT`, `STOP_LOSS_LIMIT` and `TAKE_PROFIT_LIMIT`
    /// orders; `0.0` otherwise.
    #[serde(with = "string_or_float", default = "default_stop_price")]
    pub stop_price: f64,
    /// Only present if the order was placed with `icebergQty`; empty otherwise.
    #[serde(default)]
    pub iceberg_qty: String,
    pub time: u64,
    pub update_time: u64,
    pub is_working: bool,
    #[serde(default)]
    pub orig_quote_order_qty: String,
    pub working_time: Option<i64>,
    pub self_trade_prevention_mode: Option<String>,
    pub prevented_match_id: Option<u64>,
    #[serde(default, with = "string_or_float_opt")]
    pub prevented_quantity: Option<f64>,
    pub strategy_id: Option<u64>,
    pub strategy_type: Option<u64>,
    pub trailing_delta: Option<u64>,
    pub trailing_time: Option<i64>,
    pub used_sor: Option<bool>,
    pub working_floor: Option<String>,
    pub peg_price_type: Option<String>,
    pub peg_offset_type: Option<String>,
    pub peg_offset_value: Option<i64>,
    #[serde(default, with = "string_or_float_opt")]
    pub pegged_price: Option<f64>,
    /// Cause of the order's expiration, only present for expired orders
    pub expiry_reason: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct OrderCanceled {
    pub symbol: String,
    pub orig_client_order_id: Option<String>,
    pub order_id: Option<u64>,
    pub order_list_id: Option<i64>,
    pub client_order_id: Option<String>,
    pub transact_time: Option<u64>,
    #[serde(default, with = "string_or_float_opt")]
    pub price: Option<f64>,
    #[serde(default, with = "string_or_float_opt")]
    pub orig_qty: Option<f64>,
    #[serde(default, with = "string_or_float_opt")]
    pub executed_qty: Option<f64>,
    #[serde(default, with = "string_or_float_opt")]
    pub orig_quote_order_qty: Option<f64>,
    #[serde(default, with = "string_or_float_opt")]
    pub cummulative_quote_qty: Option<f64>,
    pub status: Option<String>,
    pub time_in_force: Option<String>,
    #[serde(rename = "type")]
    pub type_name: Option<String>,
    pub side: Option<String>,
    #[serde(default, with = "string_or_float_opt")]
    pub stop_price: Option<f64>,
    #[serde(default, with = "string_or_float_opt")]
    pub iceberg_qty: Option<f64>,
    pub self_trade_prevention_mode: Option<String>,
    pub expiry_reason: Option<String>,
}

/// Result of `DELETE /api/v3/openOrders`, which cancels both plain orders and order lists.
#[allow(clippy::large_enum_variant)]
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(untagged)]
pub enum CanceledOrderOrList {
    OrderList(OrderList),
    Order(OrderCanceled),
}

/// One of the orders making up an order list (OCO/OTO/OTOCO/OPO/OPOCO)
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct OrderListOrder {
    pub symbol: String,
    pub order_id: u64,
    pub client_order_id: String,
}

/// An order list as returned by the `orderList` family of endpoints.
///
/// `order_reports` is only present on placement and cancellation responses.
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct OrderList {
    pub order_list_id: i64,
    pub contingency_type: String,
    pub list_status_type: String,
    pub list_order_status: String,
    pub list_client_order_id: String,
    pub transaction_time: u64,
    pub symbol: String,
    pub orders: Vec<OrderListOrder>,
    #[serde(default)]
    pub order_reports: Option<Vec<OrderCanceled>>,
}

/// Response of `POST /api/v3/order/cancelReplace`.
///
/// `cancel_response` and `new_order_response` either hold the order payload or,
/// when that step failed, a [`CancelReplaceOutcome::Error`] with the Binance error.
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CancelReplaceResponse {
    pub cancel_result: String,
    pub new_order_result: String,
    pub cancel_response: Option<CancelReplaceOutcome>,
    pub new_order_response: Option<CancelReplaceOutcome>,
}

#[allow(clippy::large_enum_variant)]
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(untagged)]
pub enum CancelReplaceOutcome {
    Order(OrderCanceled),
    Error { code: i64, msg: String },
}

/// Response of `PUT /api/v3/order/amend/keepPriority`
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct AmendKeepPriorityResponse {
    pub transact_time: u64,
    pub execution_id: u64,
    pub amended_order: AmendedOrder,
    #[serde(default)]
    pub list_status: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct AmendedOrder {
    pub symbol: String,
    pub order_id: u64,
    pub order_list_id: i64,
    pub orig_client_order_id: String,
    pub client_order_id: String,
    #[serde(with = "string_or_float")]
    pub price: f64,
    #[serde(with = "string_or_float")]
    pub qty: f64,
    #[serde(with = "string_or_float")]
    pub executed_qty: f64,
    #[serde(default, with = "string_or_float_opt")]
    pub prevented_qty: Option<f64>,
    #[serde(default, with = "string_or_float_opt")]
    pub quote_order_qty: Option<f64>,
    #[serde(with = "string_or_float")]
    pub cumulative_quote_qty: f64,
    pub status: String,
    pub time_in_force: String,
    #[serde(rename = "type")]
    pub type_name: String,
    pub side: String,
    pub working_time: Option<i64>,
    pub self_trade_prevention_mode: Option<String>,
}

/// Element of the `GET /api/v3/order/amendments` response
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SpotOrderAmendment {
    pub symbol: String,
    pub order_id: u64,
    pub execution_id: u64,
    pub orig_client_order_id: String,
    pub new_client_order_id: String,
    #[serde(with = "string_or_float")]
    pub orig_qty: f64,
    #[serde(with = "string_or_float")]
    pub new_qty: f64,
    pub time: u64,
}
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub enum SpotFuturesTransferType {
    SpotToUsdtFutures = 1,
    UsdtFuturesToSpot = 2,
    SpotToCoinFutures = 3,
    CoinFuturesToSpot = 4,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct TransactionId {
    pub tran_id: u64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Transaction {
    pub symbol: String,
    pub order_id: u64,
    pub order_list_id: Option<i64>,
    pub client_order_id: String,
    pub transact_time: u64,
    #[serde(with = "string_or_float")]
    pub price: f64,
    #[serde(with = "string_or_float")]
    pub orig_qty: f64,
    #[serde(with = "string_or_float")]
    pub executed_qty: f64,
    #[serde(default, with = "string_or_float_opt")]
    pub orig_quote_order_qty: Option<f64>,
    #[serde(with = "string_or_float")]
    pub cummulative_quote_qty: f64,
    #[serde(with = "string_or_float", default = "default_stop_price")]
    pub stop_price: f64,
    pub status: String,
    pub time_in_force: String,
    #[serde(rename = "type")]
    pub type_name: String,
    pub side: String,
    pub working_time: Option<i64>,
    pub self_trade_prevention_mode: Option<String>,
    pub expiry_reason: Option<String>,
    pub fills: Option<Vec<FillInfo>>,
}

fn default_stop_price() -> f64 {
    0.0
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct FillInfo {
    #[serde(with = "string_or_float")]
    pub price: f64,
    #[serde(with = "string_or_float")]
    pub qty: f64,
    #[serde(with = "string_or_float")]
    pub commission: f64,
    pub commission_asset: String,
    pub trade_id: Option<u64>,
}
/// Response to a test order (endpoint /api/v3/order/test).
///
/// Currently, the API responds {} on a successfull test transaction,
/// hence this struct has no fields.
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct TestResponse {}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct OrderBook {
    pub last_update_id: u64,
    pub bids: Vec<Bids>,
    pub asks: Vec<Asks>,
}

#[derive(PartialEq, Debug, Serialize, Deserialize, Clone)]
pub struct Bids {
    #[serde(with = "string_or_float")]
    pub price: f64,
    #[serde(with = "string_or_float")]
    pub qty: f64,
}

impl Bids {
    pub fn new(price: f64, qty: f64) -> Bids {
        Bids { price, qty }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Asks {
    #[serde(with = "string_or_float")]
    pub price: f64,
    #[serde(with = "string_or_float")]
    pub qty: f64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct UserDataStream {
    pub listen_key: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Success {}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
#[serde(untagged)]
pub enum Prices {
    AllPrices(Vec<SymbolPrice>),
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SymbolPrice {
    pub symbol: String,
    #[serde(with = "string_or_float")]
    pub price: f64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AveragePrice {
    pub mins: u64,
    #[serde(with = "string_or_float")]
    pub price: f64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
#[serde(untagged)]
pub enum BookTickers {
    AllBookTickers(Vec<Tickers>),
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum KlineSummaries {
    AllKlineSummaries(Vec<KlineSummary>),
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Tickers {
    pub symbol: String,
    #[serde(with = "string_or_float")]
    pub bid_price: f64,
    #[serde(with = "string_or_float")]
    pub bid_qty: f64,
    #[serde(with = "string_or_float")]
    pub ask_price: f64,
    #[serde(with = "string_or_float")]
    pub ask_qty: f64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct TradeHistory {
    pub symbol: Option<String>,
    pub id: u64,
    pub order_id: Option<u64>,
    pub order_list_id: Option<i64>,
    #[serde(with = "string_or_float")]
    pub price: f64,
    #[serde(with = "string_or_float")]
    pub qty: f64,
    #[serde(default, with = "string_or_float_opt")]
    pub quote_qty: Option<f64>,
    pub commission: String,
    pub commission_asset: String,
    pub time: u64,
    pub is_buyer: bool,
    pub is_maker: bool,
    pub is_best_match: bool,
}

/// Element of `GET /api/v3/trades` and `GET /api/v3/historicalTrades`
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Trade {
    pub id: u64,
    #[serde(with = "string_or_float")]
    pub price: f64,
    #[serde(with = "string_or_float")]
    pub qty: f64,
    #[serde(with = "string_or_float")]
    pub quote_qty: f64,
    pub time: u64,
    pub is_buyer_maker: bool,
    pub is_best_match: bool,
}

/// Price change statistics returned by `GET /api/v3/ticker/tradingDay` and the
/// rolling-window `GET /api/v3/ticker` endpoint.
///
/// The `MINI` response type omits `price_change`, `price_change_percent` and
/// `weighted_avg_price`.
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct TickerStats {
    pub symbol: String,
    pub price_change: Option<String>,
    pub price_change_percent: Option<String>,
    pub weighted_avg_price: Option<String>,
    #[serde(with = "string_or_float")]
    pub open_price: f64,
    #[serde(with = "string_or_float")]
    pub high_price: f64,
    #[serde(with = "string_or_float")]
    pub low_price: f64,
    #[serde(with = "string_or_float")]
    pub last_price: f64,
    #[serde(with = "string_or_float")]
    pub volume: f64,
    #[serde(with = "string_or_float")]
    pub quote_volume: f64,
    pub open_time: u64,
    pub close_time: u64,
    pub first_id: i64,
    pub last_id: i64,
    pub count: u64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct PriceStats {
    pub symbol: String,
    pub price_change: String,
    pub price_change_percent: String,
    pub weighted_avg_price: String,
    #[serde(with = "string_or_float")]
    pub prev_close_price: f64,
    #[serde(with = "string_or_float")]
    pub last_price: f64,
    #[serde(with = "string_or_float")]
    pub bid_price: f64,
    #[serde(with = "string_or_float")]
    pub ask_price: f64,
    #[serde(with = "string_or_float")]
    pub open_price: f64,
    #[serde(with = "string_or_float")]
    pub high_price: f64,
    #[serde(with = "string_or_float")]
    pub low_price: f64,
    #[serde(with = "string_or_float")]
    pub volume: f64,
    pub open_time: u64,
    pub close_time: u64,
    pub first_id: i64,
    pub last_id: i64,
    pub count: u64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AggTrade {
    #[serde(rename = "T")]
    pub time: u64,
    #[serde(rename = "a")]
    pub agg_id: u64,
    #[serde(rename = "f")]
    pub first_id: u64,
    #[serde(rename = "l")]
    pub last_id: u64,
    #[serde(rename = "m")]
    pub maker: bool,
    #[serde(rename = "M")]
    pub best_match: bool,
    #[serde(rename = "p", with = "string_or_float")]
    pub price: f64,
    #[serde(rename = "q", with = "string_or_float")]
    pub qty: f64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct UserDataStreamExpiredEvent {
    #[serde(rename = "e")]
    pub event_type: String,

    #[serde(rename = "E")]
    pub event_time: u64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct AccountUpdateEvent {
    #[serde(rename = "e")]
    pub event_type: String,

    #[serde(rename = "E")]
    pub event_time: u64,

    #[serde(rename = "a")]
    pub data: AccountUpdateDataEvent,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct AccountUpdateDataEvent {
    #[serde(rename = "m")]
    pub reason: String,

    #[serde(rename = "B")]
    pub balances: Vec<EventBalance>,

    #[serde(rename = "P")]
    pub positions: Vec<EventPosition>,

    /// Symbol, only pushed when the reason is `FUNDING_FEE`
    #[serde(rename = "S")]
    pub symbol: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct EventBalance {
    #[serde(rename = "a")]
    pub asset: String,
    #[serde(rename = "wb")]
    pub wallet_balance: String,
    #[serde(rename = "cw")]
    pub cross_wallet_balance: String,
    #[serde(rename = "bc")]
    pub balance_change: String, // Balance Change except PnL and Commission
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct EventPosition {
    #[serde(rename = "s")]
    pub symbol: String,
    #[serde(rename = "pa")]
    pub position_amount: String,
    #[serde(rename = "ep")]
    pub entry_price: String,
    #[serde(rename = "bep")]
    pub breakeven_price: Option<String>,
    #[serde(rename = "cr")]
    pub accumulated_realized: String, // (Pre-fee) Accumulated Realized
    #[serde(rename = "up")]
    pub unrealized_pnl: String,
    #[serde(rename = "mt")]
    pub margin_type: String,
    #[serde(rename = "iw")]
    pub isolated_wallet: String,
    #[serde(rename = "ps")]
    pub position_side: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct BalanceUpdateEvent {
    #[serde(rename = "B")]
    pub balance: Vec<EventBalance>,

    #[serde(rename = "e")]
    pub event_type: String,

    #[serde(rename = "E")]
    pub event_time: u64,

    #[serde(rename = "u")]
    pub last_account_update_time: u64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct OrderTradeEvent {
    #[serde(rename = "e")]
    pub event_type: String,

    #[serde(rename = "E")]
    pub event_time: u64,

    #[serde(rename = "s")]
    pub symbol: String,

    #[serde(rename = "c")]
    pub new_client_order_id: String,

    #[serde(rename = "S")]
    pub side: String,

    #[serde(rename = "o")]
    pub order_type: String,

    #[serde(rename = "f")]
    pub time_in_force: String,

    #[serde(rename = "q")]
    pub qty: String,

    #[serde(rename = "p")]
    pub price: String,

    #[serde(skip, rename = "P")]
    pub p_ignore: String,

    #[serde(skip, rename = "F")]
    pub f_ignore: String,

    #[serde(skip)]
    pub g: i32,

    #[serde(skip, rename = "C")]
    pub c_ignore: Option<String>,

    #[serde(rename = "x")]
    pub execution_type: String,

    #[serde(rename = "X")]
    pub order_status: String,

    #[serde(rename = "r")]
    pub order_reject_reason: String,

    #[serde(rename = "i")]
    pub order_id: u64,

    #[serde(rename = "l")]
    pub qty_last_filled_trade: String,

    #[serde(rename = "z")]
    pub accumulated_qty_filled_trades: String,

    #[serde(rename = "L")]
    pub price_last_filled_trade: String,

    #[serde(rename = "n")]
    pub commission: String,

    #[serde(skip, rename = "N")]
    pub asset_commisioned: Option<String>,

    #[serde(rename = "T")]
    pub trade_order_time: u64,

    #[serde(rename = "t")]
    pub trade_id: i64,

    #[serde(skip, rename = "I")]
    pub i_ignore: u64,

    #[serde(skip)]
    pub w: bool,

    #[serde(rename = "m")]
    pub is_buyer_maker: bool,

    #[serde(skip, rename = "M")]
    pub m_ignore: bool,

    /// Order creation time
    #[serde(rename = "O")]
    pub order_creation_time: Option<u64>,

    /// Cumulative quote asset transacted quantity
    #[serde(rename = "Z")]
    pub cumulative_quote_qty: Option<String>,

    /// Last quote asset transacted quantity (lastPrice * lastQty)
    #[serde(rename = "Y")]
    pub last_quote_qty: Option<String>,

    /// Quote order quantity
    #[serde(rename = "Q")]
    pub quote_order_qty: Option<String>,

    /// Working time; only present once the order is on the book
    #[serde(rename = "W")]
    pub working_time: Option<u64>,

    /// Self trade prevention mode
    #[serde(rename = "V")]
    pub self_trade_prevention_mode: Option<String>,

    /// Prevented match id, only present if the order expired due to STP
    #[serde(rename = "v")]
    pub prevented_match_id: Option<u64>,

    /// Prevented quantity, only present if the order expired due to STP
    #[serde(rename = "A")]
    pub prevented_quantity: Option<String>,

    /// Trailing delta, only present for trailing stop orders
    #[serde(rename = "d")]
    pub trailing_delta: Option<u64>,

    /// Trailing time, only present for trailing stop orders
    #[serde(rename = "D")]
    pub trailing_time: Option<i64>,

    /// Working floor (`SOR` or `EXCHANGE`), only present for SOR orders
    #[serde(rename = "k")]
    pub working_floor: Option<String>,

    /// Pegged price, only present for pegged orders
    #[serde(rename = "gp")]
    pub pegged_price: Option<String>,

    /// Expiry reason, only present when the order has expired
    #[serde(rename = "eR")]
    pub expiry_reason: Option<String>,
}

/// Spot user data stream `outboundAccountPosition` event
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct OutboundAccountPositionEvent {
    #[serde(rename = "e")]
    pub event_type: String,

    #[serde(rename = "E")]
    pub event_time: u64,

    /// Time of last account update
    #[serde(rename = "u")]
    pub last_account_update_time: u64,

    #[serde(rename = "B")]
    pub balances: Vec<SpotEventBalance>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SpotEventBalance {
    #[serde(rename = "a")]
    pub asset: String,
    #[serde(rename = "f")]
    pub free: String,
    #[serde(rename = "l")]
    pub locked: String,
}

/// Spot user data stream `balanceUpdate` event (deposit, withdrawal, transfer)
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SpotBalanceUpdateEvent {
    #[serde(rename = "e")]
    pub event_type: String,

    #[serde(rename = "E")]
    pub event_time: u64,

    #[serde(rename = "a")]
    pub asset: String,

    /// Balance delta
    #[serde(rename = "d")]
    pub delta: String,

    /// Clear time
    #[serde(rename = "T")]
    pub clear_time: u64,
}

/// Spot user data stream `listStatus` event (order list status change)
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ListStatusEvent {
    #[serde(rename = "e")]
    pub event_type: String,

    #[serde(rename = "E")]
    pub event_time: u64,

    #[serde(rename = "s")]
    pub symbol: String,

    #[serde(rename = "g")]
    pub order_list_id: i64,

    #[serde(rename = "c")]
    pub contingency_type: String,

    #[serde(rename = "l")]
    pub list_status_type: String,

    #[serde(rename = "L")]
    pub list_order_status: String,

    #[serde(rename = "r")]
    pub list_reject_reason: String,

    #[serde(rename = "C")]
    pub list_client_order_id: String,

    #[serde(rename = "T")]
    pub transaction_time: u64,

    #[serde(rename = "O")]
    pub orders: Vec<ListStatusOrder>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ListStatusOrder {
    #[serde(rename = "s")]
    pub symbol: String,
    #[serde(rename = "i")]
    pub order_id: u64,
    #[serde(rename = "c")]
    pub client_order_id: String,
}

/// Spot user data stream `externalLockUpdate` event
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ExternalLockUpdateEvent {
    #[serde(rename = "e")]
    pub event_type: String,

    #[serde(rename = "E")]
    pub event_time: u64,

    #[serde(rename = "a")]
    pub asset: String,

    #[serde(rename = "d")]
    pub delta: String,

    #[serde(rename = "T")]
    pub transaction_time: u64,
}

/// Generic event carrying only a type and a time: `eventStreamTerminated`,
/// `listenKeyExpired`, `serverShutdown`.
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct GenericEvent {
    #[serde(rename = "e")]
    pub event_type: String,

    #[serde(rename = "E")]
    pub event_time: u64,
}

/// The Aggregate Trade Streams push trade information that is aggregated for a single taker order.
///
/// Stream Name: \<symbol\>@aggTrade
///
/// Update Speed: Real-time
///
/// <https://github.com/binance/binance-spot-api-docs/blob/master/web-socket-streams.md#aggregate-trade-streams>
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct AggrTradesEvent {
    #[serde(rename = "e")]
    pub event_type: String,

    #[serde(rename = "E")]
    pub event_time: u64,

    #[serde(rename = "s")]
    pub symbol: String,

    #[serde(rename = "a")]
    pub aggregated_trade_id: u64,

    #[serde(rename = "p")]
    pub price: String,

    #[serde(rename = "q")]
    pub qty: String,

    #[serde(rename = "f")]
    pub first_break_trade_id: u64,

    #[serde(rename = "l")]
    pub last_break_trade_id: u64,

    #[serde(rename = "T")]
    pub trade_order_time: u64,

    #[serde(rename = "m")]
    pub is_buyer_maker: bool,

    #[serde(skip, rename = "M")]
    pub m_ignore: bool,
}

/// The Trade Streams push raw trade information; each trade has a unique buyer and seller.
///
/// Stream Name: \<symbol\>@trade
///
/// Update Speed: Real-time
///
/// <https://github.com/binance/binance-spot-api-docs/blob/master/web-socket-streams.md#trade-streams>
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct TradeEvent {
    #[serde(rename = "e")]
    pub event_type: String,

    #[serde(rename = "E")]
    pub event_time: u64,

    #[serde(rename = "s")]
    pub symbol: String,

    #[serde(rename = "t")]
    pub trade_id: u64,

    #[serde(rename = "p")]
    pub price: String,

    #[serde(rename = "q")]
    pub qty: String,

    /// Buyer order id; removed from spot trade streams by Binance on 2024-06-18
    #[serde(rename = "b")]
    pub buyer_order_id: Option<u64>,

    /// Seller order id; removed from spot trade streams by Binance on 2024-06-18
    #[serde(rename = "a")]
    pub seller_order_id: Option<u64>,

    #[serde(rename = "T")]
    pub trade_order_time: u64,

    #[serde(rename = "m")]
    pub is_buyer_maker: bool,

    #[serde(skip, rename = "M")]
    pub m_ignore: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct IndexPriceEvent {
    #[serde(rename = "e")]
    pub event_type: String,

    #[serde(rename = "E")]
    pub event_time: u64,

    #[serde(rename = "i")]
    pub pair: String,

    #[serde(rename = "p")]
    pub price: String,
}
// https://binance-docs.github.io/apidocs/futures/en/#mark-price-stream
// https://binance-docs.github.io/apidocs/delivery/en/#mark-price-stream
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct MarkPriceEvent {
    #[serde(rename = "E")]
    pub event_time: u64,

    #[serde(rename = "P")]
    pub estimate_settle_price: String,

    #[serde(rename = "T")]
    pub next_funding_time: u64,

    #[serde(rename = "e")]
    pub event_type: String,

    #[serde(rename = "i")]
    pub index_price: Option<String>,

    #[serde(rename = "p")]
    pub mark_price: String,

    #[serde(rename = "r")]
    pub funding_rate: String,

    #[serde(rename = "s")]
    pub symbol: String,

    /// Mark price moving average
    #[serde(rename = "ap")]
    pub mark_price_moving_average: Option<String>,
}

// Object({"E": Number(1626118018407), "e": String("forceOrder"), "o": Object({"S": String("SELL"), "T": Number(1626118018404), "X": String("FILLED"), "ap": String("33028.07"), "f": String("IOC"), "l": String("0.010"), "o": String("LIMIT"), "p": String("32896.00"), "q": String("0.010"), "s": String("BTCUSDT"), "z": String("0.010")})})
// https://binance-docs.github.io/apidocs/futures/en/#liquidation-order-streams

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct LiquidationEvent {
    #[serde(rename = "e")]
    pub event_type: String,

    #[serde(rename = "E")]
    pub event_time: u64,

    #[serde(rename = "o")]
    pub liquidation_order: LiquidationOrder,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct LiquidationOrder {
    #[serde(rename = "s")]
    pub symbol: String,

    #[serde(rename = "S")]
    pub side: String,

    #[serde(rename = "o")]
    pub order_type: String,

    #[serde(rename = "f")]
    pub time_in_force: String,

    #[serde(rename = "q")]
    pub original_quantity: String,

    #[serde(rename = "p")]
    pub price: String,

    #[serde(rename = "ap")]
    pub average_price: String,

    #[serde(rename = "X")]
    pub order_status: String,

    #[serde(rename = "l")]
    pub order_last_filled_quantity: String,

    #[serde(rename = "z")]
    pub order_filled_accumulated_quantity: String,

    #[serde(rename = "T")]
    pub order_trade_time: u64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct BookTickerEvent {
    #[serde(rename = "u")]
    pub update_id: u64,

    #[serde(rename = "s")]
    pub symbol: String,

    #[serde(rename = "b")]
    pub best_bid: String,

    #[serde(rename = "B")]
    pub best_bid_qty: String,

    #[serde(rename = "a")]
    pub best_ask: String,

    #[serde(rename = "A")]
    pub best_ask_qty: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct DayTickerEvent {
    #[serde(rename = "e")]
    pub event_type: String,

    #[serde(rename = "E")]
    pub event_time: u64,

    #[serde(rename = "s")]
    pub symbol: String,

    #[serde(rename = "p")]
    pub price_change: String,

    #[serde(rename = "P")]
    pub price_change_percent: String,

    #[serde(rename = "w")]
    pub average_price: String,

    #[serde(rename = "x")]
    pub prev_close: String,

    #[serde(rename = "c")]
    pub current_close: String,

    #[serde(rename = "Q")]
    pub current_close_qty: String,

    #[serde(rename = "b")]
    pub best_bid: String,

    #[serde(rename = "B")]
    pub best_bid_qty: String,

    #[serde(rename = "a")]
    pub best_ask: String,

    #[serde(rename = "A")]
    pub best_ask_qty: String,

    #[serde(rename = "o")]
    pub open: String,

    #[serde(rename = "h")]
    pub high: String,

    #[serde(rename = "l")]
    pub low: String,

    #[serde(rename = "v")]
    pub volume: String,

    #[serde(rename = "q")]
    pub quote_volume: String,

    #[serde(rename = "O")]
    pub open_time: u64,

    #[serde(rename = "C")]
    pub close_time: u64,

    #[serde(rename = "F")]
    pub first_trade_id: i64,

    #[serde(rename = "L")]
    pub last_trade_id: i64,

    #[serde(rename = "n")]
    pub num_trades: u64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct WindowTickerEvent {
    #[serde(rename = "e")]
    pub event_type: String,

    #[serde(rename = "E")]
    pub event_time: u64,

    #[serde(rename = "s")]
    pub symbol: String,

    #[serde(rename = "p")]
    pub price_change: String,

    #[serde(rename = "P")]
    pub price_change_percent: String,

    #[serde(rename = "o")]
    pub open: String,

    #[serde(rename = "h")]
    pub high: String,

    #[serde(rename = "l")]
    pub low: String,

    #[serde(rename = "c")]
    pub current_close: String,

    #[serde(rename = "w")]
    pub average_price: String,

    #[serde(rename = "v")]
    pub volume: String,

    #[serde(rename = "q")]
    pub quote_volume: String,

    #[serde(rename = "O")]
    pub open_time: u64,

    #[serde(rename = "C")]
    pub close_time: u64,

    #[serde(rename = "F")]
    pub first_trade_id: i64,

    #[serde(rename = "L")]
    pub last_trade_id: i64,

    #[serde(rename = "n")]
    pub num_trades: u64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct MiniTickerEvent {
    #[serde(rename = "e")]
    pub event_type: String,

    #[serde(rename = "E")]
    pub event_time: u64,

    #[serde(rename = "s")]
    pub symbol: String,

    #[serde(rename = "c")]
    pub close: String,

    #[serde(rename = "o")]
    pub open: String,

    #[serde(rename = "h")]
    pub high: String,

    #[serde(rename = "l")]
    pub low: String,

    #[serde(rename = "v")]
    pub volume: String,

    #[serde(rename = "q")]
    pub quote_volume: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct KlineEvent {
    #[serde(rename = "e")]
    pub event_type: String,

    #[serde(rename = "E")]
    pub event_time: u64,

    #[serde(rename = "s")]
    pub symbol: String,

    #[serde(rename = "k")]
    pub kline: Kline,
}

// https://binance-docs.github.io/apidocs/futures/en/#continuous-contract-kline-candlestick-streams

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ContinuousKlineEvent {
    #[serde(rename = "e")]
    pub event_type: String,

    #[serde(rename = "E")]
    pub event_time: u64,

    #[serde(rename = "ps")]
    pub pair: String,

    #[serde(rename = "ct")]
    pub contract_type: String,

    #[serde(rename = "k")]
    pub kline: ContinuousKline,
}

// https://binance-docs.github.io/apidocs/delivery/en/#index-kline-candlestick-streams

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct IndexKlineEvent {
    #[serde(rename = "e")]
    pub event_type: String,

    #[serde(rename = "E")]
    pub event_time: u64,

    #[serde(rename = "ps")]
    pub pair: String,

    #[serde(rename = "k")]
    pub kline: IndexKline,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct KlineSummary {
    pub open_time: i64,

    pub open: String,

    pub high: String,

    pub low: String,

    pub close: String,

    pub volume: String,

    pub close_time: i64,

    pub quote_asset_volume: String,

    pub number_of_trades: i64,

    pub taker_buy_base_asset_volume: String,

    pub taker_buy_quote_asset_volume: String,
}

fn get_value(row: &[Value], index: usize, name: &'static str) -> Result<Value> {
    Ok(row
        .get(index)
        .ok_or_else(|| ErrorKind::KlineValueMissingError(index, name))?
        .clone())
}

impl TryFrom<&Vec<Value>> for KlineSummary {
    type Error = Error;

    fn try_from(row: &Vec<Value>) -> Result<Self> {
        Ok(Self {
            open_time: from_value(get_value(row, 0, "open_time")?)?,
            open: from_value(get_value(row, 1, "open")?)?,
            high: from_value(get_value(row, 2, "high")?)?,
            low: from_value(get_value(row, 3, "low")?)?,
            close: from_value(get_value(row, 4, "close")?)?,
            volume: from_value(get_value(row, 5, "volume")?)?,
            close_time: from_value(get_value(row, 6, "close_time")?)?,
            quote_asset_volume: from_value(get_value(row, 7, "quote_asset_volume")?)?,
            number_of_trades: from_value(get_value(row, 8, "number_of_trades")?)?,
            taker_buy_base_asset_volume: from_value(get_value(
                row,
                9,
                "taker_buy_base_asset_volume",
            )?)?,
            taker_buy_quote_asset_volume: from_value(get_value(
                row,
                10,
                "taker_buy_quote_asset_volume",
            )?)?,
        })
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Kline {
    #[serde(rename = "t")]
    pub open_time: i64,

    #[serde(rename = "T")]
    pub close_time: i64,

    #[serde(rename = "s")]
    pub symbol: String,

    #[serde(rename = "i")]
    pub interval: String,

    #[serde(rename = "f")]
    pub first_trade_id: i64,

    #[serde(rename = "L")]
    pub last_trade_id: i64,

    #[serde(rename = "o")]
    pub open: String,

    #[serde(rename = "c")]
    pub close: String,

    #[serde(rename = "h")]
    pub high: String,

    #[serde(rename = "l")]
    pub low: String,

    #[serde(rename = "v")]
    pub volume: String,

    #[serde(rename = "n")]
    pub number_of_trades: i64,

    #[serde(rename = "x")]
    pub is_final_bar: bool,

    #[serde(rename = "q")]
    pub quote_asset_volume: String,

    #[serde(rename = "V")]
    pub taker_buy_base_asset_volume: String,

    #[serde(rename = "Q")]
    pub taker_buy_quote_asset_volume: String,

    #[serde(skip, rename = "B")]
    pub ignore_me: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ContinuousKline {
    #[serde(rename = "t")]
    pub start_time: i64,

    #[serde(rename = "T")]
    pub end_time: i64,

    #[serde(rename = "i")]
    pub interval: String,

    #[serde(rename = "f")]
    pub first_trade_id: i64,

    #[serde(rename = "L")]
    pub last_trade_id: i64,

    #[serde(rename = "o")]
    pub open: String,

    #[serde(rename = "c")]
    pub close: String,

    #[serde(rename = "h")]
    pub high: String,

    #[serde(rename = "l")]
    pub low: String,

    #[serde(rename = "v")]
    pub volume: String,

    #[serde(rename = "n")]
    pub number_of_trades: i64,

    #[serde(rename = "x")]
    pub is_final_bar: bool,

    #[serde(rename = "q")]
    pub quote_volume: String,

    #[serde(rename = "V")]
    pub active_buy_volume: String,

    #[serde(rename = "Q")]
    pub active_volume_buy_quote: String,

    #[serde(skip, rename = "B")]
    pub ignore_me: String,
}

// https://binance-docs.github.io/apidocs/delivery/en/#index-kline-candlestick-streams
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct IndexKline {
    #[serde(rename = "t")]
    pub start_time: i64,

    #[serde(rename = "T")]
    pub end_time: i64,

    #[serde(skip, rename = "s")]
    pub ignore_me: String,

    #[serde(rename = "i")]
    pub interval: String,

    #[serde(rename = "f")]
    pub first_trade_id: i64,

    #[serde(rename = "L")]
    pub last_trade_id: i64,

    #[serde(rename = "o")]
    pub open: String,

    #[serde(rename = "c")]
    pub close: String,

    #[serde(rename = "h")]
    pub high: String,

    #[serde(rename = "l")]
    pub low: String,

    #[serde(rename = "v")]
    pub volume: String,

    #[serde(rename = "n")]
    pub number_of_trades: i64,

    #[serde(rename = "x")]
    pub is_final_bar: bool,

    #[serde(skip, rename = "q")]
    pub ignore_me2: String,

    #[serde(skip, rename = "V")]
    pub ignore_me3: String,

    #[serde(skip, rename = "Q")]
    pub ignore_me4: String,

    #[serde(skip, rename = "B")]
    pub ignore_me5: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct DepthOrderBookEvent {
    #[serde(rename = "e")]
    pub event_type: String,

    #[serde(rename = "E")]
    pub event_time: u64,

    #[serde(rename = "s")]
    pub symbol: String,

    #[serde(rename = "U")]
    pub first_update_id: u64,

    #[serde(rename = "u")]
    pub final_update_id: u64,

    #[serde(rename = "pu")]
    #[serde(default)]
    pub previous_final_update_id: Option<u64>,

    #[serde(rename = "b")]
    pub bids: Vec<Bids>,

    #[serde(rename = "a")]
    pub asks: Vec<Asks>,
}

/// Response to the Savings API get all coins request
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CoinInfo {
    pub coin: String,
    pub deposit_all_enable: bool,
    #[serde(with = "string_or_float")]
    pub free: f64,
    #[serde(with = "string_or_float")]
    pub freeze: f64,
    #[serde(with = "string_or_float")]
    pub ipoable: f64,
    #[serde(with = "string_or_float")]
    pub ipoing: f64,
    pub is_legal_money: bool,
    #[serde(with = "string_or_float")]
    pub locked: f64,
    pub name: String,
    pub network_list: Vec<Network>,
    #[serde(with = "string_or_float")]
    pub storage: f64,
    pub trading: bool,
    pub withdraw_all_enable: bool,
    #[serde(with = "string_or_float")]
    pub withdrawing: f64,
}

/// Part of the Savings API get all coins response
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Network {
    pub address_regex: String,
    pub coin: String,
    /// shown only when "depositEnable" is false.
    pub deposit_desc: Option<String>,
    pub deposit_enable: bool,
    pub is_default: bool,
    pub memo_regex: String,
    /// min number for balance confirmation
    pub min_confirm: u32,
    pub name: String,
    pub network: String,
    pub reset_address_status: bool,
    pub special_tips: Option<String>,
    /// confirmation number for balance unlock
    pub un_lock_confirm: u32,
    /// shown only when "withdrawEnable" is false.
    pub withdraw_desc: Option<String>,
    pub withdraw_enable: bool,
    #[serde(with = "string_or_float")]
    pub withdraw_fee: f64,
    #[serde(with = "string_or_float")]
    pub withdraw_min: f64,
    // pub insert_time: Option<u64>, //commented out for now, because they are not inside the actual response (only the api doc example)
    // pub update_time: Option<u64>,
    pub withdraw_integer_multiple: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct AssetDetail {
    #[serde(with = "string_or_float")]
    pub min_withdraw_amount: f64,
    /// false if ALL of networks' are false
    pub deposit_status: bool,
    #[serde(with = "string_or_float")]
    pub withdraw_fee: f64,
    /// false if ALL of networks' are false
    pub withdraw_status: bool,
    /// reason
    pub deposit_tip: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DepositAddress {
    pub address: String,
    pub coin: String,
    pub tag: String,
    pub url: String,
}

pub(crate) mod string_or_float {
    use std::fmt;

    use serde::{de, Serializer, Deserialize, Deserializer};

    pub fn serialize<T, S>(value: &T, serializer: S) -> Result<S::Ok, S::Error>
    where
        T: fmt::Display,
        S: Serializer,
    {
        serializer.collect_str(value)
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<f64, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        #[serde(untagged)]
        enum StringOrFloat {
            String(String),
            Float(f64),
        }

        match StringOrFloat::deserialize(deserializer)? {
            StringOrFloat::String(s) => {
                if s == "INF" {
                    Ok(f64::INFINITY)
                } else {
                    s.parse().map_err(de::Error::custom)
                }
            }
            StringOrFloat::Float(i) => Ok(i),
        }
    }
}

pub(crate) mod string_or_float_opt {
    use std::fmt;

    use serde::{de, Serializer, Deserialize, Deserializer};

    pub fn serialize<T, S>(value: &Option<T>, serializer: S) -> Result<S::Ok, S::Error>
    where
        T: fmt::Display,
        S: Serializer,
    {
        match value {
            Some(v) => crate::model::string_or_float::serialize(v, serializer),
            None => serializer.serialize_none(),
        }
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Option<f64>, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        #[serde(untagged)]
        enum StringOrFloat {
            String(String),
            Float(f64),
        }

        match Option::<StringOrFloat>::deserialize(deserializer)? {
            None => Ok(None),
            Some(StringOrFloat::Float(f)) => Ok(Some(f)),
            Some(StringOrFloat::String(s)) => {
                if s.is_empty() || s == "null" {
                    Ok(None)
                } else if s == "INF" {
                    Ok(Some(f64::INFINITY))
                } else {
                    s.parse().map(Some).map_err(de::Error::custom)
                }
            }
        }
    }
}

/// Deserialize a value that Binance sends either as a JSON integer or as a numeric string
/// (e.g. the `code` field of algo order cancellations).
pub(crate) mod string_or_i64 {
    use serde::{de, Serializer, Deserialize, Deserializer};

    pub fn serialize<S>(value: &i64, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_i64(*value)
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<i64, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        #[serde(untagged)]
        enum StringOrInt {
            String(String),
            Int(i64),
        }

        match StringOrInt::deserialize(deserializer)? {
            StringOrInt::String(s) => s.parse().map_err(de::Error::custom),
            StringOrInt::Int(i) => Ok(i),
        }
    }
}

pub(crate) mod string_or_bool {
    use std::fmt;

    use serde::{de, Serializer, Deserialize, Deserializer};

    pub fn serialize<T, S>(value: &T, serializer: S) -> Result<S::Ok, S::Error>
    where
        T: fmt::Display,
        S: Serializer,
    {
        serializer.collect_str(value)
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<bool, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        #[serde(untagged)]
        enum StringOrFloat {
            String(String),
            Bool(bool),
        }

        match StringOrFloat::deserialize(deserializer)? {
            StringOrFloat::String(s) => s.parse().map_err(de::Error::custom),
            StringOrFloat::Bool(i) => Ok(i),
        }
    }
}

#[test]
fn test_account_update_event() {
    let json = r#"
    {
  "e": "ACCOUNT_UPDATE",
  "E": 1564745798939,
  "T": 1564745798938,
  "a": {
    "m": "ORDER",
    "B": [
      {
        "a": "USDT",
        "wb": "122624.12345678",
        "cw": "100.12345678",
        "bc": "50.12345678"
      },
      {
        "a": "BUSD",
        "wb": "1.00000000",
        "cw": "0.00000000",
        "bc": "-49.12345678"
      }
    ],
    "P": [
      {
        "s": "BTCUSDT",
        "pa": "0",
        "ep": "0.00000",
        "cr": "200",
        "up": "0",
        "mt": "isolated",
        "iw": "0.00000000",
        "ps": "BOTH"
      },
      {
        "s": "BTCUSDT",
        "pa": "20",
        "ep": "6563.66500",
        "cr": "0",
        "up": "2850.21200",
        "mt": "isolated",
        "iw": "13200.70726908",
        "ps": "LONG"
      },
      {
        "s": "BTCUSDT",
        "pa": "-10",
        "ep": "6563.86000",
        "cr": "-45.04000000",
        "up": "-1423.15600",
        "mt": "isolated",
        "iw": "6570.42511771",
        "ps": "SHORT"
      }
    ]
  }
}
    "#;

    let res = r#"AccountUpdateEvent { event_type: "ACCOUNT_UPDATE", event_time: 1564745798939, data: AccountUpdateDataEvent { reason: "ORDER", balances: [EventBalance { asset: "USDT", wallet_balance: "122624.12345678", cross_wallet_balance: "100.12345678", balance_change: "50.12345678" }, EventBalance { asset: "BUSD", wallet_balance: "1.00000000", cross_wallet_balance: "0.00000000", balance_change: "-49.12345678" }], positions: [EventPosition { symbol: "BTCUSDT", position_amount: "0", entry_price: "0.00000", breakeven_price: None, accumulated_realized: "200", unrealized_pnl: "0", margin_type: "isolated", isolated_wallet: "0.00000000", position_side: "BOTH" }, EventPosition { symbol: "BTCUSDT", position_amount: "20", entry_price: "6563.66500", breakeven_price: None, accumulated_realized: "0", unrealized_pnl: "2850.21200", margin_type: "isolated", isolated_wallet: "13200.70726908", position_side: "LONG" }, EventPosition { symbol: "BTCUSDT", position_amount: "-10", entry_price: "6563.86000", breakeven_price: None, accumulated_realized: "-45.04000000", unrealized_pnl: "-1423.15600", margin_type: "isolated", isolated_wallet: "6570.42511771", position_side: "SHORT" }], symbol: None } }"#;
    let v: AccountUpdateEvent = serde_json::from_str(json).unwrap();
    assert_eq!(format!("{:?}", v), res);
    //let event =  from_value::<AccountUpdateEvent>(json).unwrap();
}
