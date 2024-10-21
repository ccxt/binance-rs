use serde::{Deserialize, Serialize};
use crate::model::{string_or_decimal, string_or_decimal_opt, string_or_bool};
use rust_decimal::prelude::*;

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
    pub exchange_filters: Vec<String>,
    pub symbols: Vec<Symbol>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Symbol {
    pub symbol: String,
    pub status: String,
    pub maint_margin_percent: String,
    pub required_margin_percent: String,
    pub base_asset: String,
    pub quote_asset: String,
    pub onboard_date: u128,
    pub price_precision: u16,
    pub quantity_precision: u16,
    pub base_asset_precision: u64,
    pub quote_precision: u64,
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
    #[serde(with = "string_or_decimal")]
    pub last_price: Decimal,
    #[serde(with = "string_or_decimal")]
    pub open_price: Decimal,
    #[serde(with = "string_or_decimal")]
    pub high_price: Decimal,
    #[serde(with = "string_or_decimal")]
    pub low_price: Decimal,
    #[serde(with = "string_or_decimal")]
    pub volume: Decimal,
    #[serde(with = "string_or_decimal")]
    pub quote_volume: Decimal,
    #[serde(with = "string_or_decimal")]
    pub last_qty: Decimal,
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
    #[serde(with = "string_or_decimal")]
    pub commission: Decimal,
    pub commission_asset: String,
    pub id: u64,
    pub maker: bool,
    pub order_id: u64,
    #[serde(with = "string_or_decimal")]
    pub price: Decimal,
    #[serde(with = "string_or_decimal")]
    pub qty: Decimal,
    #[serde(with = "string_or_decimal")]
    pub quote_qty: Decimal,
    #[serde(with = "string_or_decimal")]
    pub realized_pnl: Decimal,
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
    #[serde(with = "string_or_decimal")]
    pub price: Decimal,
    #[serde(with = "string_or_decimal")]
    pub qty: Decimal,
    #[serde(with = "string_or_decimal")]
    pub quote_qty: Decimal,
    pub time: u64,
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
    #[serde(rename = "p", with = "string_or_decimal")]
    pub price: Decimal,
    #[serde(rename = "q", with = "string_or_decimal")]
    pub qty: Decimal,
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
    #[serde(with = "string_or_decimal")]
    pub mark_price: Decimal,
    #[serde(with = "string_or_decimal")]
    pub last_funding_rate: Decimal,
    pub next_funding_time: u64,
    pub time: u64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(untagged)]
pub enum LiquidationOrders {
    AllLiquidationOrders(Vec<LiquidationOrder>),
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct LiquidationOrder {
    #[serde(with = "string_or_decimal")]
    pub average_price: Decimal,
    #[serde(with = "string_or_decimal")]
    pub executed_qty: Decimal,
    #[serde(with = "string_or_decimal")]
    pub orig_qty: Decimal,
    #[serde(with = "string_or_decimal")]
    pub price: Decimal,
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
    #[serde(with = "string_or_decimal")]
    pub open_interest: Decimal,
    pub symbol: String,
}

#[derive(Debug, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct OpenInterestHist {
    pub symbol: String,
    pub sum_open_interest: String,
    pub sum_open_interest_value: String,
    pub timestamp: u64,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Order {
    pub client_order_id: String,
    #[serde(with = "string_or_decimal", default = "default_stop_price")]
    pub cum_qty: Decimal,
    #[serde(with = "string_or_decimal")]
    pub cum_quote: Decimal,
    #[serde(with = "string_or_decimal")]
    pub executed_qty: Decimal,
    pub order_id: u64,
    #[serde(with = "string_or_decimal")]
    pub avg_price: Decimal,
    #[serde(with = "string_or_decimal")]
    pub orig_qty: Decimal,
    #[serde(with = "string_or_decimal")]
    pub price: Decimal,
    pub side: String,
    pub reduce_only: bool,
    pub position_side: String,
    pub status: String,
    #[serde(with = "string_or_decimal", default = "default_stop_price")]
    pub stop_price: Decimal,
    pub close_position: bool,
    pub symbol: String,
    pub time_in_force: String,
    #[serde(rename = "type")]
    pub order_type: String,
    pub orig_type: String,
    #[serde(with = "string_or_decimal", default = "default_activation_price")]
    pub activation_price: Decimal,
    #[serde(with = "string_or_decimal", default = "default_price_rate")]
    pub price_rate: Decimal,
    pub update_time: u64,
    pub working_type: String,
    pub price_protect: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Transaction {
    pub client_order_id: String,
    #[serde(with = "string_or_decimal")]
    pub cum_qty: Decimal,
    #[serde(with = "string_or_decimal")]
    pub cum_quote: Decimal,
    #[serde(with = "string_or_decimal")]
    pub executed_qty: Decimal,
    pub order_id: u64,
    #[serde(with = "string_or_decimal")]
    pub avg_price: Decimal,
    #[serde(with = "string_or_decimal")]
    pub orig_qty: Decimal,
    pub reduce_only: bool,
    pub side: String,
    pub position_side: String,
    pub status: String,
    #[serde(with = "string_or_decimal")]
    pub stop_price: Decimal,
    pub close_position: bool,
    pub symbol: String,
    pub time_in_force: String,
    #[serde(rename = "type")]
    pub type_name: String,
    pub orig_type: String,
    #[serde(default)]
    #[serde(with = "string_or_decimal_opt")]
    pub activate_price: Option<Decimal>,
    #[serde(default)]
    #[serde(with = "string_or_decimal_opt")]
    pub price_rate: Option<Decimal>,
    pub update_time: u64,
    pub working_type: String,
    price_protect: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CanceledOrder {
    pub client_order_id: String,
    #[serde(with = "string_or_decimal")]
    pub cum_qty: Decimal,
    #[serde(with = "string_or_decimal")]
    pub cum_quote: Decimal,
    #[serde(with = "string_or_decimal")]
    pub executed_qty: Decimal,
    pub order_id: u64,
    #[serde(with = "string_or_decimal")]
    pub orig_qty: Decimal,
    pub orig_type: String,
    #[serde(with = "string_or_decimal")]
    pub price: Decimal,
    pub reduce_only: bool,
    pub side: String,
    pub position_side: String,
    pub status: String,
    #[serde(with = "string_or_decimal")]
    pub stop_price: Decimal,
    pub close_position: bool,
    pub symbol: String,
    pub time_in_force: String,
    #[serde(rename = "type")]
    pub type_name: String,
    #[serde(default)]
    #[serde(with = "string_or_decimal_opt")]
    pub activate_price: Option<Decimal>,
    #[serde(default)]
    #[serde(with = "string_or_decimal_opt")]
    pub price_rate: Option<Decimal>,
    pub update_time: u64,
    pub working_type: String,
    price_protect: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct PositionRisk {
    #[serde(with = "string_or_decimal")]
    pub entry_price: Decimal,
    pub margin_type: String,
    #[serde(with = "string_or_bool")]
    pub is_auto_add_margin: bool,
    #[serde(with = "string_or_decimal")]
    pub isolated_margin: Decimal,
    pub leverage: String,
    #[serde(with = "string_or_decimal")]
    pub liquidation_price: Decimal,
    #[serde(with = "string_or_decimal")]
    pub mark_price: Decimal,
    #[serde(with = "string_or_decimal")]
    pub max_notional_value: Decimal,
    #[serde(with = "string_or_decimal", rename = "positionAmt")]
    pub position_amount: Decimal,
    pub symbol: String,
    #[serde(with = "string_or_decimal", rename = "unRealizedProfit")]
    pub unrealized_profit: Decimal,
    pub position_side: String,
    #[serde(with = "string_or_decimal")]
    pub notional: Decimal,
    #[serde(with = "string_or_decimal")]
    pub isolated_wallet: Decimal,
    pub update_time: u64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct FuturesAsset {
    pub asset: String,
    #[serde(with = "string_or_decimal")]
    pub wallet_balance: Decimal,
    #[serde(with = "string_or_decimal")]
    pub unrealized_profit: Decimal,
    #[serde(with = "string_or_decimal")]
    pub margin_balance: Decimal,
    #[serde(with = "string_or_decimal")]
    pub maint_margin: Decimal,
    #[serde(with = "string_or_decimal")]
    pub initial_margin: Decimal,
    #[serde(with = "string_or_decimal")]
    pub position_initial_margin: Decimal,
    #[serde(with = "string_or_decimal")]
    pub open_order_initial_margin: Decimal,
    #[serde(with = "string_or_decimal")]
    pub max_withdraw_amount: Decimal,
    #[serde(with = "string_or_decimal")]
    pub cross_wallet_balance: Decimal,
    #[serde(with = "string_or_decimal")]
    pub cross_un_pnl: Decimal,
    #[serde(with = "string_or_decimal")]
    pub available_balance: Decimal,
    #[serde(with = "string_or_bool")]
    pub margin_available: bool,
    pub update_time: u64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct FuturesPosition {
    pub symbol: String,
    #[serde(with = "string_or_decimal")]
    pub initial_margin: Decimal,
    #[serde(with = "string_or_decimal")]
    pub maint_margin: Decimal,
    #[serde(with = "string_or_decimal")]
    pub unrealized_profit: Decimal,
    #[serde(with = "string_or_decimal")]
    pub position_initial_margin: Decimal,
    #[serde(with = "string_or_decimal")]
    pub open_order_initial_margin: Decimal,
    pub leverage: String,
    #[serde(with = "string_or_bool")]
    pub isolated: bool,
    #[serde(with = "string_or_decimal")]
    pub entry_price: Decimal,
    #[serde(with = "string_or_decimal")]
    pub max_notional: Decimal,
    pub position_side: String,
    #[serde(with = "string_or_decimal", rename = "positionAmt")]
    pub position_amount: Decimal,
    #[serde(with = "string_or_decimal")]
    pub notional: Decimal,
    #[serde(with = "string_or_decimal")]
    pub isolated_wallet: Decimal,
    pub update_time: u64,
    #[serde(with = "string_or_decimal")]
    pub bid_notional: Decimal,
    #[serde(with = "string_or_decimal")]
    pub ask_notional: Decimal,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct AccountInformation {
    #[serde(with = "string_or_decimal")]
    pub fee_tier: Decimal,
    #[serde(with = "string_or_bool")]
    pub can_trade: bool,
    #[serde(with = "string_or_bool")]
    pub can_deposit: bool,
    #[serde(with = "string_or_bool")]
    pub can_withdraw: bool,
    #[serde(with = "string_or_decimal")]
    pub update_time: Decimal,
    #[serde(with = "string_or_decimal")]
    pub total_initial_margin: Decimal,
    #[serde(with = "string_or_decimal")]
    pub total_maint_margin: Decimal,
    #[serde(with = "string_or_decimal")]
    pub total_wallet_balance: Decimal,
    #[serde(with = "string_or_decimal")]
    pub total_unrealized_profit: Decimal,
    #[serde(with = "string_or_decimal")]
    pub total_margin_balance: Decimal,
    #[serde(with = "string_or_decimal")]
    pub total_position_initial_margin: Decimal,
    #[serde(with = "string_or_decimal")]
    pub total_open_order_initial_margin: Decimal,
    #[serde(with = "string_or_decimal")]
    pub total_cross_wallet_balance: Decimal,
    #[serde(with = "string_or_decimal")]
    pub total_cross_un_pnl: Decimal,
    #[serde(with = "string_or_decimal")]
    pub available_balance: Decimal,
    #[serde(with = "string_or_decimal")]
    pub max_withdraw_amount: Decimal,
    pub assets: Vec<FuturesAsset>,
    pub positions: Vec<FuturesPosition>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct AccountBalance {
    pub account_alias: String,
    pub asset: String,
    #[serde(with = "string_or_decimal")]
    pub balance: Decimal,
    #[serde(with = "string_or_decimal")]
    pub cross_wallet_balance: Decimal,
    #[serde(with = "string_or_decimal", rename = "crossUnPnl")]
    pub cross_unrealized_pnl: Decimal,
    #[serde(with = "string_or_decimal")]
    pub available_balance: Decimal,
    #[serde(with = "string_or_decimal")]
    pub max_withdraw_amount: Decimal,
    pub margin_available: bool,
    pub update_time: u64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ChangeLeverageResponse {
    pub leverage: u8,
    #[serde(with = "string_or_decimal")]
    pub max_notional_value: Decimal,
    pub symbol: String,
}

fn default_stop_price() -> Decimal {
    Decimal::ZERO
}
fn default_activation_price() -> Decimal {
    Decimal::ZERO
}
fn default_price_rate() -> Decimal {
    Decimal::ZERO
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
    #[serde(with = "string_or_decimal")]
    pub income: Decimal,
    pub asset: String,
    pub info: String,
    pub time: u64,
    pub tran_id: u64,
    pub trade_id: String,
}
