use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use crate::model::{string_or_float, string_or_float_opt, string_or_bool, string_or_i64};

pub use crate::model::{
    Asks, Bids, BookTickers, Filters, KlineSummaries, KlineSummary, RateLimit, ServerTime,
    SymbolPrice, Tickers,
};

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ExchangeInformation {
    pub timezone: String,
    pub server_time: u64,
    pub rate_limits: Vec<RateLimit>,
    #[serde(default)]
    pub exchange_filters: Vec<serde_json::Value>,
    /// Margin assets available in Multi-Assets mode
    #[serde(default)]
    pub assets: Vec<ExchangeAsset>,
    pub symbols: Vec<Symbol>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ExchangeAsset {
    pub asset: String,
    pub margin_available: bool,
    #[serde(default, with = "string_or_float_opt")]
    pub auto_asset_exchange: Option<f64>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Symbol {
    pub symbol: String,
    pub pair: Option<String>,
    /// `PERPETUAL`, `CURRENT_QUARTER`, `NEXT_QUARTER`, `TRADIFI_PERPETUAL`, ...
    pub contract_type: Option<String>,
    pub delivery_date: Option<u64>,
    pub status: String,
    pub maint_margin_percent: String,
    pub required_margin_percent: String,
    pub base_asset: String,
    pub quote_asset: String,
    pub margin_asset: Option<String>,
    pub onboard_date: u128,
    pub price_precision: u16,
    pub quantity_precision: u16,
    pub base_asset_precision: u64,
    pub quote_precision: u64,
    pub underlying_type: Option<String>,
    #[serde(default)]
    pub underlying_sub_type: Vec<String>,
    pub settle_plan: Option<i64>,
    #[serde(default, with = "string_or_float_opt")]
    pub trigger_protect: Option<f64>,
    #[serde(default, with = "string_or_float_opt")]
    pub liquidation_fee: Option<f64>,
    #[serde(default, with = "string_or_float_opt")]
    pub market_take_bound: Option<f64>,
    pub max_move_order_limit: Option<u64>,
    pub filters: Vec<Filters>,
    pub order_types: Vec<String>,
    pub time_in_force: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct OrderBook {
    pub last_update_id: u64,
    // Undocumented
    #[serde(rename = "E")]
    pub event_time: u64,
    // Undocumented
    #[serde(rename = "T")]
    pub trade_order_time: u64,
    pub bids: Vec<Bids>,
    pub asks: Vec<Asks>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct PriceStats {
    pub symbol: String,
    pub price_change: String,
    pub price_change_percent: String,
    pub weighted_avg_price: String,
    #[serde(with = "string_or_float")]
    pub last_price: f64,
    #[serde(with = "string_or_float")]
    pub open_price: f64,
    #[serde(with = "string_or_float")]
    pub high_price: f64,
    #[serde(with = "string_or_float")]
    pub low_price: f64,
    #[serde(with = "string_or_float")]
    pub volume: f64,
    #[serde(with = "string_or_float")]
    pub quote_volume: f64,
    #[serde(with = "string_or_float")]
    pub last_qty: f64,
    pub open_time: u64,
    pub close_time: u64,
    pub first_id: u64,
    pub last_id: u64,
    pub count: u64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct TradeHistory {
    pub buyer: bool,
    #[serde(with = "string_or_float")]
    pub commission: f64,
    pub commission_asset: String,
    pub id: u64,
    pub maker: bool,
    pub order_id: u64,
    #[serde(with = "string_or_float")]
    pub price: f64,
    #[serde(with = "string_or_float")]
    pub qty: f64,
    #[serde(with = "string_or_float")]
    pub quote_qty: f64,
    #[serde(with = "string_or_float")]
    pub realized_pnl: f64,
    pub side: String,
    pub position_side: String,
    pub symbol: String,
    pub time: u64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(untagged)]
pub enum Trades {
    AllTrades(Vec<Trade>),
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Trade {
    pub id: u64,
    pub is_buyer_maker: bool,
    #[serde(with = "string_or_float")]
    pub price: f64,
    #[serde(with = "string_or_float")]
    pub qty: f64,
    #[serde(with = "string_or_float")]
    pub quote_qty: f64,
    pub time: u64,
    /// Whether this is a Retail Price Improvement trade
    #[serde(rename = "isRPITrade")]
    pub is_rpi_trade: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(untagged)]
pub enum AggTrades {
    AllAggTrades(Vec<AggTrade>),
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
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
    #[serde(rename = "p", with = "string_or_float")]
    pub price: f64,
    #[serde(rename = "q", with = "string_or_float")]
    pub qty: f64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(untagged)]
pub enum MarkPrices {
    AllMarkPrices(Vec<MarkPrice>),
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct MarkPrice {
    pub symbol: String,
    #[serde(with = "string_or_float")]
    pub mark_price: f64,
    #[serde(default, with = "string_or_float_opt")]
    pub index_price: Option<f64>,
    #[serde(default, with = "string_or_float_opt")]
    pub estimated_settle_price: Option<f64>,
    #[serde(with = "string_or_float")]
    pub last_funding_rate: f64,
    #[serde(default, with = "string_or_float_opt")]
    pub interest_rate: Option<f64>,
    pub next_funding_time: u64,
    pub time: u64,
}

/// Element of `GET /fapi/v1/fundingRate`
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct FundingRate {
    pub symbol: String,
    #[serde(with = "string_or_float")]
    pub funding_rate: f64,
    pub funding_time: u64,
    #[serde(default, with = "string_or_float_opt")]
    pub mark_price: Option<f64>,
    /// `Regular` or `Special` (stock-dividend funding fee)
    pub rate_type: Option<String>,
}

/// Element of `GET /fapi/v1/fundingInfo`
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct FundingInfo {
    pub symbol: String,
    #[serde(with = "string_or_float")]
    pub adjusted_funding_rate_cap: f64,
    #[serde(with = "string_or_float")]
    pub adjusted_funding_rate_floor: f64,
    pub funding_interval_hours: u32,
    pub disclaimer: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(untagged)]
pub enum LiquidationOrders {
    AllLiquidationOrders(Vec<LiquidationOrder>),
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct LiquidationOrder {
    #[serde(with = "string_or_float")]
    pub average_price: f64,
    #[serde(with = "string_or_float")]
    pub executed_qty: f64,
    #[serde(with = "string_or_float")]
    pub orig_qty: f64,
    #[serde(with = "string_or_float")]
    pub price: f64,
    pub side: String,
    pub status: String,
    pub symbol: String,
    pub time: u64,
    pub time_in_force: String,
    pub r#type: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct OpenInterest {
    #[serde(with = "string_or_float")]
    pub open_interest: f64,
    pub symbol: String,
}

#[derive(Debug, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct OpenInterestHist {
    pub symbol: String,
    pub sum_open_interest: String,
    pub sum_open_interest_value: String,
    #[serde(rename = "CMCCirculatingSupply")]
    pub cmc_circulating_supply: Option<String>,
    pub timestamp: u64,
}

/// Element of the `topLongShortAccountRatio`, `topLongShortPositionRatio` and
/// `globalLongShortAccountRatio` endpoints
#[derive(Debug, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct LongShortRatio {
    pub symbol: String,
    #[serde(with = "string_or_float")]
    pub long_short_ratio: f64,
    #[serde(with = "string_or_float")]
    pub long_account: f64,
    #[serde(with = "string_or_float")]
    pub short_account: f64,
    pub timestamp: u64,
}

/// Element of `GET /futures/data/takerlongshortRatio`
#[derive(Debug, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct TakerLongShortRatio {
    #[serde(with = "string_or_float")]
    pub buy_sell_ratio: f64,
    #[serde(with = "string_or_float")]
    pub buy_vol: f64,
    #[serde(with = "string_or_float")]
    pub sell_vol: f64,
    pub timestamp: u64,
}

/// Element of `GET /futures/data/basis`
#[derive(Debug, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Basis {
    pub pair: String,
    pub contract_type: String,
    #[serde(with = "string_or_float")]
    pub index_price: f64,
    #[serde(with = "string_or_float")]
    pub futures_price: f64,
    #[serde(with = "string_or_float")]
    pub basis: f64,
    #[serde(with = "string_or_float")]
    pub basis_rate: f64,
    /// Empty for perpetual contracts
    #[serde(default, with = "string_or_float_opt")]
    pub annualized_basis_rate: Option<f64>,
    pub timestamp: u64,
}

/// Element of `GET /fapi/v1/insuranceBalance`
#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct InsuranceBalance {
    pub symbols: Vec<String>,
    pub assets: Vec<InsuranceAsset>,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct InsuranceAsset {
    pub asset: String,
    #[serde(with = "string_or_float")]
    pub margin_balance: f64,
    pub update_time: u64,
}

/// Element of `GET /fapi/v1/symbolAdlRisk`
#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct AdlRisk {
    pub symbol: String,
    /// `low`, `medium` or `high`
    pub adl_risk: String,
    pub update_time: u64,
}

/// Element of `GET /fapi/v1/indexInfo`
#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct IndexInfo {
    pub symbol: String,
    pub time: u64,
    pub component: String,
    pub base_asset_list: Vec<IndexBaseAsset>,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct IndexBaseAsset {
    pub base_asset: String,
    pub quote_asset: String,
    #[serde(with = "string_or_float")]
    pub weight_in_quantity: f64,
    #[serde(with = "string_or_float")]
    pub weight_in_percentage: f64,
}

/// Element of `GET /fapi/v1/assetIndex`
#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct AssetIndex {
    pub symbol: String,
    pub time: u64,
    #[serde(with = "string_or_float")]
    pub index: f64,
    #[serde(with = "string_or_float")]
    pub bid_buffer: f64,
    #[serde(with = "string_or_float")]
    pub ask_buffer: f64,
    #[serde(with = "string_or_float")]
    pub bid_rate: f64,
    #[serde(with = "string_or_float")]
    pub ask_rate: f64,
    #[serde(with = "string_or_float")]
    pub auto_exchange_bid_buffer: f64,
    #[serde(with = "string_or_float")]
    pub auto_exchange_ask_buffer: f64,
    #[serde(with = "string_or_float")]
    pub auto_exchange_bid_rate: f64,
    #[serde(with = "string_or_float")]
    pub auto_exchange_ask_rate: f64,
}

/// Response of `GET /fapi/v1/constituents`
#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct IndexConstituents {
    pub symbol: String,
    pub time: u64,
    pub constituents: Vec<IndexConstituent>,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct IndexConstituent {
    pub exchange: String,
    pub symbol: String,
    #[serde(default, with = "string_or_float_opt")]
    pub price: Option<f64>,
    #[serde(default, with = "string_or_float_opt")]
    pub weight: Option<f64>,
}

/// Element of `GET /futures/data/delivery-price`
#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct DeliveryPrice {
    pub delivery_time: u64,
    pub delivery_price: f64,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Order {
    pub client_order_id: String,
    #[serde(with = "string_or_float", default = "default_stop_price")]
    pub cum_qty: f64,
    #[serde(with = "string_or_float")]
    pub cum_quote: f64,
    #[serde(with = "string_or_float")]
    pub executed_qty: f64,
    pub order_id: u64,
    #[serde(with = "string_or_float")]
    pub avg_price: f64,
    #[serde(with = "string_or_float")]
    pub orig_qty: f64,
    #[serde(with = "string_or_float")]
    pub price: f64,
    pub side: String,
    pub reduce_only: bool,
    pub position_side: String,
    pub status: String,
    #[serde(with = "string_or_float", default = "default_stop_price")]
    pub stop_price: f64,
    pub close_position: bool,
    pub symbol: String,
    pub time: Option<u64>,
    pub time_in_force: String,
    #[serde(rename = "type")]
    pub order_type: String,
    pub orig_type: String,
    /// Activation price of `TRAILING_STOP_MARKET` orders (`activatePrice` in the API)
    #[serde(
        rename = "activatePrice",
        with = "string_or_float",
        default = "default_activation_price"
    )]
    pub activation_price: f64,
    #[serde(with = "string_or_float", default = "default_price_rate")]
    pub price_rate: f64,
    pub update_time: u64,
    pub working_type: String,
    pub price_protect: bool,
    pub price_match: Option<String>,
    pub self_trade_prevention_mode: Option<String>,
    pub good_till_date: Option<u64>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Transaction {
    pub client_order_id: String,
    #[serde(with = "string_or_float")]
    pub cum_qty: f64,
    #[serde(with = "string_or_float", default = "default_stop_price")]
    pub cum_quote: f64,
    #[serde(with = "string_or_float")]
    pub executed_qty: f64,
    pub order_id: u64,
    #[serde(with = "string_or_float", default = "default_stop_price")]
    pub avg_price: f64,
    #[serde(with = "string_or_float")]
    pub orig_qty: f64,
    #[serde(default, with = "string_or_float_opt")]
    pub price: Option<f64>,
    pub reduce_only: bool,
    pub side: String,
    pub position_side: String,
    pub status: String,
    #[serde(with = "string_or_float", default = "default_stop_price")]
    pub stop_price: f64,
    pub close_position: bool,
    pub symbol: String,
    pub time_in_force: String,
    #[serde(rename = "type")]
    pub type_name: String,
    pub orig_type: String,
    #[serde(default)]
    #[serde(with = "string_or_float_opt")]
    pub activate_price: Option<f64>,
    #[serde(default)]
    #[serde(with = "string_or_float_opt")]
    pub price_rate: Option<f64>,
    pub update_time: u64,
    pub working_type: String,
    pub price_protect: bool,
    pub price_match: Option<String>,
    pub self_trade_prevention_mode: Option<String>,
    pub good_till_date: Option<u64>,
}

/// Element of the `POST /fapi/v1/batchOrders` and `PUT /fapi/v1/batchOrders` responses.
///
/// Each entry is either the placed/modified order or a Binance error for that order;
/// results are returned in the same order as the submitted list.
#[allow(clippy::large_enum_variant)]
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(untagged)]
pub enum BatchOrderResponse {
    Error { code: i64, msg: String },
    Order(Transaction),
}

/// Element of the `DELETE /fapi/v1/batchOrders` response.
#[allow(clippy::large_enum_variant)]
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(untagged)]
pub enum BatchCancelResponse {
    Error { code: i64, msg: String },
    Order(CanceledOrder),
}

/// Response of `PUT /fapi/v1/order` (modify order)
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ModifiedOrder {
    pub order_id: u64,
    pub symbol: String,
    pub pair: Option<String>,
    pub status: String,
    pub client_order_id: String,
    /// Echo of the `modifyId` sent in the request, if any
    pub modify_id: Option<i64>,
    #[serde(with = "string_or_float")]
    pub price: f64,
    #[serde(with = "string_or_float")]
    pub orig_qty: f64,
    #[serde(with = "string_or_float")]
    pub executed_qty: f64,
    #[serde(with = "string_or_float")]
    pub cum_qty: f64,
    #[serde(with = "string_or_float", default = "default_stop_price")]
    pub cum_base: f64,
    pub time_in_force: String,
    #[serde(rename = "type")]
    pub type_name: String,
    pub reduce_only: bool,
    pub close_position: bool,
    pub side: String,
    pub position_side: String,
    #[serde(with = "string_or_float", default = "default_stop_price")]
    pub stop_price: f64,
    pub working_type: String,
    pub price_protect: bool,
    pub orig_type: String,
    pub price_match: Option<String>,
    pub self_trade_prevention_mode: Option<String>,
    pub good_till_date: Option<u64>,
    pub update_time: u64,
}

/// Element of `GET /fapi/v1/orderAmendment`
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct OrderAmendment {
    pub amendment_id: u64,
    pub symbol: String,
    pub pair: Option<String>,
    pub order_id: u64,
    pub client_order_id: String,
    pub time: u64,
    pub amendment: Amendment,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Amendment {
    pub price: AmendedValue,
    pub orig_qty: AmendedValue,
    pub count: u64,
    pub modify_id: Option<i64>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct AmendedValue {
    #[serde(with = "string_or_float")]
    pub before: f64,
    #[serde(with = "string_or_float")]
    pub after: f64,
}

/// Response of `POST /fapi/v1/countdownCancelAll`
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CountdownCancelAll {
    pub symbol: String,
    #[serde(with = "string_or_i64")]
    pub countdown_time: i64,
}

/// Element of `GET /fapi/v1/forceOrders`
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ForceOrder {
    pub order_id: u64,
    pub symbol: String,
    pub pair: Option<String>,
    pub status: String,
    pub client_order_id: String,
    #[serde(with = "string_or_float")]
    pub price: f64,
    #[serde(with = "string_or_float")]
    pub avg_price: f64,
    #[serde(with = "string_or_float")]
    pub orig_qty: f64,
    #[serde(with = "string_or_float")]
    pub executed_qty: f64,
    #[serde(with = "string_or_float", default = "default_stop_price")]
    pub cum_quote: f64,
    #[serde(with = "string_or_float", default = "default_stop_price")]
    pub cum_base: f64,
    pub time_in_force: String,
    #[serde(rename = "type")]
    pub type_name: String,
    pub reduce_only: bool,
    pub close_position: bool,
    pub side: String,
    pub position_side: String,
    #[serde(with = "string_or_float", default = "default_stop_price")]
    pub stop_price: f64,
    pub working_type: String,
    pub orig_type: String,
    pub time: u64,
    pub update_time: u64,
}

/// A conditional (algo) order as returned by the `/fapi/v1/algoOrder` family of endpoints.
///
/// Since 2025-12-09 the conditional order types `STOP`, `STOP_MARKET`, `TAKE_PROFIT`,
/// `TAKE_PROFIT_MARKET` and `TRAILING_STOP_MARKET` are handled by the Algo service and
/// identified by `algo_id` rather than `order_id`. `actual_*` fields are only populated
/// once the order has been triggered and placed in the matching engine.
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct AlgoOrder {
    pub algo_id: u64,
    pub client_algo_id: String,
    pub algo_type: String,
    pub order_type: String,
    pub symbol: String,
    pub side: String,
    pub position_side: String,
    pub time_in_force: String,
    #[serde(with = "string_or_float", default = "default_stop_price")]
    pub quantity: f64,
    /// `NEW`, `CANCELED`, `TRIGGERING`, `TRIGGERED`, `FINISHED`, `REJECTED`, `EXPIRED`
    pub algo_status: String,
    pub actual_order_id: Option<String>,
    #[serde(default, with = "string_or_float_opt")]
    pub actual_price: Option<f64>,
    pub actual_type: Option<String>,
    #[serde(default, with = "string_or_float_opt")]
    pub actual_qty: Option<f64>,
    #[serde(with = "string_or_float", default = "default_stop_price")]
    pub trigger_price: f64,
    #[serde(with = "string_or_float", default = "default_stop_price")]
    pub price: f64,
    #[serde(default, with = "string_or_float_opt")]
    pub iceberg_quantity: Option<f64>,
    #[serde(default, with = "string_or_float_opt")]
    pub tp_trigger_price: Option<f64>,
    #[serde(default, with = "string_or_float_opt")]
    pub tp_price: Option<f64>,
    #[serde(default, with = "string_or_float_opt")]
    pub sl_trigger_price: Option<f64>,
    #[serde(default, with = "string_or_float_opt")]
    pub sl_price: Option<f64>,
    pub tp_order_type: Option<String>,
    pub self_trade_prevention_mode: Option<String>,
    pub working_type: String,
    pub price_match: Option<String>,
    pub close_position: bool,
    pub price_protect: bool,
    pub reduce_only: bool,
    #[serde(default, with = "string_or_float_opt")]
    pub activate_price: Option<f64>,
    #[serde(default, with = "string_or_float_opt")]
    pub callback_rate: Option<f64>,
    pub create_time: u64,
    pub update_time: u64,
    pub trigger_time: Option<u64>,
    pub good_till_date: Option<u64>,
}

/// Response of `DELETE /fapi/v1/algoOrder`
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CanceledAlgoOrder {
    pub algo_id: u64,
    pub client_algo_id: String,
    #[serde(with = "string_or_i64")]
    pub code: i64,
    pub msg: String,
}

/// Generic `{"code": 200, "msg": "..."}` acknowledgement
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CodeMsg {
    #[serde(with = "string_or_i64")]
    pub code: i64,
    pub msg: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CanceledOrder {
    pub client_order_id: String,
    #[serde(with = "string_or_float")]
    pub cum_qty: f64,
    #[serde(with = "string_or_float", default = "default_stop_price")]
    pub cum_quote: f64,
    #[serde(with = "string_or_float")]
    pub executed_qty: f64,
    pub order_id: u64,
    #[serde(with = "string_or_float")]
    pub orig_qty: f64,
    pub orig_type: String,
    #[serde(with = "string_or_float")]
    pub price: f64,
    pub reduce_only: bool,
    pub side: String,
    pub position_side: String,
    pub status: String,
    #[serde(with = "string_or_float", default = "default_stop_price")]
    pub stop_price: f64,
    pub close_position: bool,
    pub symbol: String,
    pub time_in_force: String,
    #[serde(rename = "type")]
    pub type_name: String,
    #[serde(default)]
    #[serde(with = "string_or_float_opt")]
    pub activate_price: Option<f64>,
    #[serde(default)]
    #[serde(with = "string_or_float_opt")]
    pub price_rate: Option<f64>,
    pub update_time: u64,
    pub working_type: String,
    pub price_protect: bool,
    pub price_match: Option<String>,
    pub self_trade_prevention_mode: Option<String>,
    pub good_till_date: Option<u64>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct PositionRisk {
    #[serde(with = "string_or_float")]
    pub entry_price: f64,
    pub margin_type: String,
    #[serde(with = "string_or_bool")]
    pub is_auto_add_margin: bool,
    #[serde(with = "string_or_float")]
    pub isolated_margin: f64,
    pub leverage: String,
    #[serde(with = "string_or_float")]
    pub liquidation_price: f64,
    #[serde(with = "string_or_float")]
    pub mark_price: f64,
    #[serde(with = "string_or_float")]
    pub max_notional_value: f64,
    #[serde(with = "string_or_float", rename = "positionAmt")]
    pub position_amount: f64,
    pub symbol: String,
    #[serde(with = "string_or_float", rename = "unRealizedProfit")]
    pub unrealized_profit: f64,
    pub position_side: String,
    #[serde(with = "string_or_float")]
    pub notional: f64,
    #[serde(with = "string_or_float")]
    pub isolated_wallet: f64,
    pub update_time: u64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct FuturesAsset {
    pub asset: String,
    #[serde(with = "string_or_float")]
    pub wallet_balance: f64,
    #[serde(with = "string_or_float")]
    pub unrealized_profit: f64,
    #[serde(with = "string_or_float")]
    pub margin_balance: f64,
    #[serde(with = "string_or_float")]
    pub maint_margin: f64,
    #[serde(with = "string_or_float")]
    pub initial_margin: f64,
    #[serde(with = "string_or_float")]
    pub position_initial_margin: f64,
    #[serde(with = "string_or_float")]
    pub open_order_initial_margin: f64,
    #[serde(with = "string_or_float")]
    pub max_withdraw_amount: f64,
    #[serde(with = "string_or_float")]
    pub cross_wallet_balance: f64,
    #[serde(with = "string_or_float")]
    pub cross_un_pnl: f64,
    #[serde(with = "string_or_float")]
    pub available_balance: f64,
    #[serde(with = "string_or_bool")]
    pub margin_available: bool,
    pub update_time: u64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct FuturesPosition {
    pub symbol: String,
    #[serde(with = "string_or_float")]
    pub initial_margin: f64,
    #[serde(with = "string_or_float")]
    pub maint_margin: f64,
    #[serde(with = "string_or_float")]
    pub unrealized_profit: f64,
    #[serde(with = "string_or_float")]
    pub position_initial_margin: f64,
    #[serde(with = "string_or_float")]
    pub open_order_initial_margin: f64,
    pub leverage: String,
    #[serde(with = "string_or_bool")]
    pub isolated: bool,
    #[serde(with = "string_or_float")]
    pub entry_price: f64,
    #[serde(with = "string_or_float")]
    pub max_notional: f64,
    pub position_side: String,
    #[serde(with = "string_or_float", rename = "positionAmt")]
    pub position_amount: f64,
    #[serde(with = "string_or_float")]
    pub notional: f64,
    #[serde(with = "string_or_float")]
    pub isolated_wallet: f64,
    pub update_time: u64,
    #[serde(with = "string_or_float")]
    pub bid_notional: f64,
    #[serde(with = "string_or_float")]
    pub ask_notional: f64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct AccountInformation {
    #[serde(with = "string_or_float")]
    pub fee_tier: f64,
    #[serde(with = "string_or_bool")]
    pub can_trade: bool,
    #[serde(with = "string_or_bool")]
    pub can_deposit: bool,
    #[serde(with = "string_or_bool")]
    pub can_withdraw: bool,
    #[serde(with = "string_or_float")]
    pub update_time: f64,
    #[serde(with = "string_or_float")]
    pub total_initial_margin: f64,
    #[serde(with = "string_or_float")]
    pub total_maint_margin: f64,
    #[serde(with = "string_or_float")]
    pub total_wallet_balance: f64,
    #[serde(with = "string_or_float")]
    pub total_unrealized_profit: f64,
    #[serde(with = "string_or_float")]
    pub total_margin_balance: f64,
    #[serde(with = "string_or_float")]
    pub total_position_initial_margin: f64,
    #[serde(with = "string_or_float")]
    pub total_open_order_initial_margin: f64,
    #[serde(with = "string_or_float")]
    pub total_cross_wallet_balance: f64,
    #[serde(with = "string_or_float")]
    pub total_cross_un_pnl: f64,
    #[serde(with = "string_or_float")]
    pub available_balance: f64,
    #[serde(with = "string_or_float")]
    pub max_withdraw_amount: f64,
    pub assets: Vec<FuturesAsset>,
    pub positions: Vec<FuturesPosition>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct AccountBalance {
    pub account_alias: String,
    pub asset: String,
    #[serde(with = "string_or_float")]
    pub balance: f64,
    #[serde(with = "string_or_float")]
    pub cross_wallet_balance: f64,
    #[serde(with = "string_or_float", rename = "crossUnPnl")]
    pub cross_unrealized_pnl: f64,
    #[serde(with = "string_or_float")]
    pub available_balance: f64,
    #[serde(with = "string_or_float")]
    pub max_withdraw_amount: f64,
    pub margin_available: bool,
    pub update_time: u64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ChangeLeverageResponse {
    pub leverage: u8,
    #[serde(with = "string_or_float")]
    pub max_notional_value: f64,
    pub symbol: String,
}

/// Element of `GET /fapi/v3/positionRisk`.
///
/// Unlike V2, only symbols with an open position or open orders are returned and the
/// symbol configuration fields (leverage, margin type, ...) are not included; use
/// `GET /fapi/v1/symbolConfig` for those.
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct PositionRiskV3 {
    pub symbol: String,
    pub position_side: String,
    #[serde(with = "string_or_float", rename = "positionAmt")]
    pub position_amount: f64,
    #[serde(with = "string_or_float")]
    pub entry_price: f64,
    #[serde(with = "string_or_float", default = "default_stop_price")]
    pub break_even_price: f64,
    #[serde(with = "string_or_float")]
    pub mark_price: f64,
    #[serde(with = "string_or_float", rename = "unRealizedProfit")]
    pub unrealized_profit: f64,
    #[serde(with = "string_or_float")]
    pub liquidation_price: f64,
    #[serde(with = "string_or_float")]
    pub isolated_margin: f64,
    #[serde(with = "string_or_float")]
    pub notional: f64,
    pub margin_asset: String,
    #[serde(with = "string_or_float")]
    pub isolated_wallet: f64,
    #[serde(with = "string_or_float")]
    pub initial_margin: f64,
    #[serde(with = "string_or_float")]
    pub maint_margin: f64,
    #[serde(with = "string_or_float")]
    pub position_initial_margin: f64,
    #[serde(with = "string_or_float")]
    pub open_order_initial_margin: f64,
    /// ADL quantile (0-4)
    pub adl: u8,
    #[serde(with = "string_or_float")]
    pub bid_notional: f64,
    #[serde(with = "string_or_float")]
    pub ask_notional: f64,
    pub update_time: u64,
}

/// Response of `GET /fapi/v3/account`
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct AccountInformationV3 {
    #[serde(with = "string_or_float")]
    pub total_initial_margin: f64,
    #[serde(with = "string_or_float")]
    pub total_maint_margin: f64,
    #[serde(with = "string_or_float")]
    pub total_wallet_balance: f64,
    #[serde(with = "string_or_float")]
    pub total_unrealized_profit: f64,
    #[serde(with = "string_or_float")]
    pub total_margin_balance: f64,
    #[serde(with = "string_or_float")]
    pub total_position_initial_margin: f64,
    #[serde(with = "string_or_float")]
    pub total_open_order_initial_margin: f64,
    #[serde(with = "string_or_float")]
    pub total_cross_wallet_balance: f64,
    #[serde(with = "string_or_float")]
    pub total_cross_un_pnl: f64,
    #[serde(with = "string_or_float")]
    pub available_balance: f64,
    #[serde(with = "string_or_float")]
    pub max_withdraw_amount: f64,
    pub assets: Vec<FuturesAssetV3>,
    pub positions: Vec<FuturesPositionV3>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct FuturesAssetV3 {
    pub asset: String,
    #[serde(with = "string_or_float")]
    pub wallet_balance: f64,
    #[serde(with = "string_or_float")]
    pub unrealized_profit: f64,
    #[serde(with = "string_or_float")]
    pub margin_balance: f64,
    #[serde(with = "string_or_float")]
    pub maint_margin: f64,
    #[serde(with = "string_or_float")]
    pub initial_margin: f64,
    #[serde(with = "string_or_float")]
    pub position_initial_margin: f64,
    #[serde(with = "string_or_float")]
    pub open_order_initial_margin: f64,
    #[serde(with = "string_or_float")]
    pub cross_wallet_balance: f64,
    #[serde(with = "string_or_float")]
    pub cross_un_pnl: f64,
    #[serde(with = "string_or_float")]
    pub available_balance: f64,
    #[serde(with = "string_or_float")]
    pub max_withdraw_amount: f64,
    pub margin_available: Option<bool>,
    pub update_time: u64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct FuturesPositionV3 {
    pub symbol: String,
    pub position_side: String,
    #[serde(with = "string_or_float", rename = "positionAmt")]
    pub position_amount: f64,
    #[serde(with = "string_or_float")]
    pub unrealized_profit: f64,
    #[serde(with = "string_or_float")]
    pub isolated_margin: f64,
    #[serde(with = "string_or_float")]
    pub notional: f64,
    #[serde(with = "string_or_float")]
    pub isolated_wallet: f64,
    #[serde(with = "string_or_float")]
    pub initial_margin: f64,
    #[serde(with = "string_or_float")]
    pub maint_margin: f64,
    pub update_time: u64,
}

/// Element of `GET /fapi/v1/symbolConfig`
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SymbolConfig {
    pub symbol: String,
    pub margin_type: String,
    pub is_auto_add_margin: bool,
    pub leverage: u16,
    #[serde(with = "string_or_float")]
    pub max_notional_value: f64,
}

/// Response of `GET /fapi/v1/accountConfig`
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct AccountConfig {
    pub fee_tier: u16,
    pub can_trade: bool,
    pub can_deposit: bool,
    pub can_withdraw: bool,
    pub dual_side_position: bool,
    pub update_time: u64,
    pub multi_assets_margin: bool,
    pub trade_group_id: i64,
}

/// Response of `GET /fapi/v1/commissionRate`
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CommissionRate {
    pub symbol: String,
    #[serde(with = "string_or_float")]
    pub maker_commission_rate: f64,
    #[serde(with = "string_or_float")]
    pub taker_commission_rate: f64,
    #[serde(default, with = "string_or_float_opt")]
    pub rpi_commission_rate: Option<f64>,
}

/// Element of `GET /fapi/v1/leverageBracket`
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct LeverageBracket {
    pub symbol: String,
    pub notional_coef: Option<f64>,
    pub brackets: Vec<Bracket>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Bracket {
    pub bracket: u16,
    pub initial_leverage: u16,
    pub notional_cap: f64,
    pub notional_floor: f64,
    pub maint_margin_ratio: f64,
    pub cum: f64,
}

/// Element of `GET /fapi/v1/positionMargin/history`
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct PositionMarginChange {
    pub symbol: String,
    /// 1: add position margin, 2: reduce position margin
    #[serde(rename = "type")]
    pub change_type: u8,
    pub delta_type: Option<String>,
    #[serde(with = "string_or_float")]
    pub amount: f64,
    pub asset: String,
    pub time: u64,
    pub position_side: String,
}

/// Element of `GET /fapi/v1/adlQuantile`
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct AdlQuantile {
    pub symbol: String,
    pub adl_quantile: AdlQuantileValues,
}

/// ADL quantile per position side. In one-way mode only `both` is set; in hedge mode
/// `long`/`short` are set, and `hedge` replaces them when the position sizes are equal.
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct AdlQuantileValues {
    #[serde(rename = "LONG")]
    pub long: Option<u8>,
    #[serde(rename = "SHORT")]
    pub short: Option<u8>,
    #[serde(rename = "HEDGE")]
    pub hedge: Option<u8>,
    #[serde(rename = "BOTH")]
    pub both: Option<u8>,
}

/// Response of `GET /fapi/v1/apiTradingStatus`
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ApiTradingStatus {
    /// Keyed by symbol (or `ACCOUNT` for account level indicators)
    pub indicators: BTreeMap<String, Vec<TradingIndicator>>,
    pub update_time: u64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct TradingIndicator {
    pub is_locked: bool,
    pub planned_recover_time: u64,
    /// `UFR`, `IFER`, `GCR`, `DR`, `TMV`
    pub indicator: String,
    pub value: f64,
    pub trigger_value: f64,
}

/// Response of `GET /fapi/v1/positionSide/dual`
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct PositionMode {
    /// `true` for Hedge mode, `false` for One-way mode
    pub dual_side_position: bool,
}

/// Response of `GET /fapi/v1/multiAssetsMargin`
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct MultiAssetsMargin {
    pub multi_assets_margin: bool,
}

/// Response of `GET /fapi/v1/feeBurn`
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct FeeBurn {
    pub fee_burn: bool,
}

/// Element of `GET /fapi/v1/rateLimit/order`
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct OrderRateLimit {
    pub rate_limit_type: String,
    pub interval: String,
    pub interval_num: u16,
    pub limit: u64,
}

fn default_stop_price() -> f64 {
    0.0
}
fn default_activation_price() -> f64 {
    0.0
}
fn default_price_rate() -> f64 {
    0.0
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct OrderUpdate {
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

    #[serde(rename = "ap")]
    pub average_price: String,

    #[serde(rename = "sp")]
    pub stop_price: String,

    #[serde(rename = "x")]
    pub execution_type: String,

    #[serde(rename = "X")]
    pub order_status: String,

    #[serde(rename = "i")]
    pub order_id: u64,

    #[serde(rename = "l")]
    pub qty_last_filled_trade: String,

    #[serde(rename = "z")]
    pub accumulated_qty_filled_trades: String,

    #[serde(rename = "L")]
    pub price_last_filled_trade: String,

    #[serde(skip, rename = "N")]
    pub asset_commisioned: Option<String>,

    #[serde(rename = "n")]
    pub commission: Option<String>,

    #[serde(rename = "T")]
    pub trade_order_time: u64,

    #[serde(rename = "t")]
    pub trade_id: i64,

    #[serde(rename = "b")]
    pub bids_notional: String,

    #[serde(rename = "a")]
    pub ask_notional: String,

    #[serde(rename = "m")]
    pub is_buyer_maker: bool,

    #[serde(rename = "R")]
    pub is_reduce_only: bool,

    #[serde(rename = "wt")]
    pub stop_price_working_type: String,

    #[serde(rename = "ot")]
    pub original_order_type: String,

    #[serde(rename = "ps")]
    pub position_side: String,

    #[serde(rename = "cp")]
    pub close_all: Option<bool>,

    #[serde(rename = "AP")]
    pub activation_price: Option<String>,

    #[serde(rename = "cr")]
    pub callback_rate: Option<String>,

    #[serde(rename = "pP")]
    pub pp_ignore: bool,

    #[serde(rename = "si")]
    pub si_ignore: i32,

    #[serde(rename = "ss")]
    pub ss_ignore: i32,

    #[serde(rename = "rp")]
    pub realized_profit: String,

    /// Self trade prevention mode
    #[serde(rename = "V")]
    pub self_trade_prevention_mode: Option<String>,

    /// Price match mode
    #[serde(rename = "pm")]
    pub price_match: Option<String>,

    /// Auto-cancel time for GTD orders
    #[serde(rename = "gtd")]
    pub good_till_date: Option<u64>,

    /// Expiry reason, only pushed for expired orders
    #[serde(rename = "er")]
    pub expiry_reason: Option<String>,

    /// `modifyId` echoed on `AMENDMENT` executions when one was sent with the request
    #[serde(rename = "M")]
    pub modify_id: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct OrderTradeEvent {
    #[serde(rename = "e")]
    pub event_type: String,

    #[serde(rename = "E")]
    pub event_time: u64,

    #[serde(rename = "T")]
    pub transaction_time: u64,

    #[serde(rename = "o")]
    pub order: OrderUpdate,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Income {
    pub symbol: String,
    pub income_type: String,
    #[serde(with = "string_or_float")]
    pub income: f64,
    pub asset: String,
    pub info: String,
    pub time: u64,
    pub tran_id: u64,
    pub trade_id: String,
}

/// `TRADE_LITE` user data event: a lightweight, lower-latency version of
/// `ORDER_TRADE_UPDATE` pushed only for `TRADE` executions.
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct TradeLiteEvent {
    #[serde(rename = "e")]
    pub event_type: String,

    #[serde(rename = "E")]
    pub event_time: u64,

    #[serde(rename = "T")]
    pub transaction_time: u64,

    #[serde(rename = "s")]
    pub symbol: String,

    #[serde(rename = "q")]
    pub original_qty: String,

    #[serde(rename = "p")]
    pub original_price: String,

    #[serde(rename = "m")]
    pub is_buyer_maker: bool,

    #[serde(rename = "c")]
    pub client_order_id: String,

    #[serde(rename = "S")]
    pub side: String,

    #[serde(rename = "L")]
    pub last_filled_price: String,

    #[serde(rename = "l")]
    pub last_filled_qty: String,

    #[serde(rename = "t")]
    pub trade_id: u64,

    #[serde(rename = "i")]
    pub order_id: u64,
}

/// `ALGO_UPDATE` user data event: status change of a conditional (algo) order.
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct AlgoUpdateEvent {
    #[serde(rename = "e")]
    pub event_type: String,

    #[serde(rename = "E")]
    pub event_time: u64,

    #[serde(rename = "T")]
    pub transaction_time: u64,

    #[serde(rename = "o")]
    pub algo_order: AlgoOrderUpdate,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct AlgoOrderUpdate {
    #[serde(rename = "caid")]
    pub client_algo_id: String,

    #[serde(rename = "aid")]
    pub algo_id: u64,

    #[serde(rename = "at")]
    pub algo_type: String,

    #[serde(rename = "o")]
    pub order_type: String,

    #[serde(rename = "s")]
    pub symbol: String,

    #[serde(rename = "S")]
    pub side: String,

    #[serde(rename = "ps")]
    pub position_side: String,

    #[serde(rename = "f")]
    pub time_in_force: String,

    #[serde(rename = "q")]
    pub quantity: String,

    /// `NEW`, `CANCELED`, `TRIGGERING`, `TRIGGERED`, `FINISHED`, `REJECTED`, `EXPIRED`
    #[serde(rename = "X")]
    pub algo_status: String,

    /// Order id in the matching engine, only once triggered
    #[serde(rename = "ai")]
    pub order_id: Option<String>,

    /// Average fill price in the matching engine, only once triggered
    #[serde(rename = "ap")]
    pub avg_price: Option<String>,

    /// Executed quantity in the matching engine, only once triggered
    #[serde(rename = "aq")]
    pub executed_qty: Option<String>,

    /// Actual order type in the matching engine, only once triggered
    #[serde(rename = "act")]
    pub actual_type: Option<String>,

    #[serde(rename = "tp")]
    pub trigger_price: Option<String>,

    #[serde(rename = "p")]
    pub price: Option<String>,

    #[serde(rename = "V")]
    pub self_trade_prevention_mode: Option<String>,

    #[serde(rename = "wt")]
    pub working_type: Option<String>,

    #[serde(rename = "pm")]
    pub price_match: Option<String>,

    #[serde(rename = "cp")]
    pub close_position: Option<bool>,

    #[serde(rename = "pP")]
    pub price_protect: Option<bool>,

    #[serde(rename = "R")]
    pub reduce_only: Option<bool>,

    #[serde(rename = "tt")]
    pub trigger_time: Option<u64>,

    #[serde(rename = "gtd")]
    pub good_till_date: Option<u64>,

    /// Reason the algo order failed
    #[serde(rename = "rm")]
    pub reject_message: Option<String>,

    /// Whether a trailing stop has been activated
    #[serde(rename = "ia")]
    pub is_activated: Option<bool>,
}

/// `MARGIN_CALL` user data event
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct MarginCallEvent {
    #[serde(rename = "e")]
    pub event_type: String,

    #[serde(rename = "E")]
    pub event_time: u64,

    /// Cross wallet balance, only pushed with crossed positions
    #[serde(rename = "cw")]
    pub cross_wallet_balance: Option<String>,

    #[serde(rename = "p")]
    pub positions: Vec<MarginCallPosition>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct MarginCallPosition {
    #[serde(rename = "s")]
    pub symbol: String,

    #[serde(rename = "ps")]
    pub position_side: String,

    #[serde(rename = "pa")]
    pub position_amount: String,

    #[serde(rename = "mt")]
    pub margin_type: String,

    /// Isolated wallet, only pushed for isolated positions
    #[serde(rename = "iw")]
    pub isolated_wallet: Option<String>,

    #[serde(rename = "mp")]
    pub mark_price: String,

    #[serde(rename = "up")]
    pub unrealized_pnl: String,

    #[serde(rename = "mm")]
    pub maintenance_margin_required: String,
}

/// `ACCOUNT_CONFIG_UPDATE` user data event: leverage or multi-assets mode change
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct AccountConfigUpdateEvent {
    #[serde(rename = "e")]
    pub event_type: String,

    #[serde(rename = "E")]
    pub event_time: u64,

    #[serde(rename = "T")]
    pub transaction_time: u64,

    /// Leverage change of a symbol
    #[serde(rename = "ac")]
    pub leverage_config: Option<LeverageConfigUpdate>,

    /// Multi-assets mode change
    #[serde(rename = "ai")]
    pub account_config: Option<MultiAssetsConfigUpdate>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct LeverageConfigUpdate {
    #[serde(rename = "s")]
    pub symbol: String,
    #[serde(rename = "l")]
    pub leverage: u16,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MultiAssetsConfigUpdate {
    #[serde(rename = "j")]
    pub multi_assets_mode: bool,
}
