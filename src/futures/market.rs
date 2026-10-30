/*!
## Implemented functionality
- [x] `Order Book` / `RPI Order Book`
- [x] `Recent Trades List`
- [x] `Old Trades Lookup (MARKET_DATA)`
- [x] `Compressed/Aggregate Trades List`
- [x] `Kline/Candlestick Data` (plus continuous, index price, mark price, premium index)
- [x] `Mark Price`
- [x] `Get Funding Rate History` / `Get Funding Rate Info`
- [x] `24hr Ticker Price Change Statistics`
- [x] `Symbol Price Ticker`
- [x] `Symbol Order Book Ticker`
- [x] `Open Interest` / `Open Interest Statistics`
- [x] `Top Trader Long/Short Ratio (Accounts)` / `(Positions)`
- [x] `Long/Short Ratio`
- [x] `Taker Buy/Sell Volume`
- [x] `Basis`
- [x] `Composite Index Symbol Information`
- [x] `Multi-Assets Mode Asset Index`
- [x] `Query Index Price Constituents`
- [x] `Query Insurance Fund Balance Snapshot`
- [x] `ADL Risk`
- [x] `Quarterly Contract Settlement Price`

`Notional and Leverage Brackets` and the user's liquidation orders are signed endpoints,
see [`crate::futures::account::FuturesAccount`].
*/

use crate::util::build_request;
use crate::futures::model::{
    AdlRisk, AggTrades, AssetIndex, Basis, BookTickers, DeliveryPrice, FundingInfo, FundingRate,
    IndexConstituents, IndexInfo, InsuranceBalance, KlineSummaries, KlineSummary,
    LiquidationOrders, LongShortRatio, MarkPrice, MarkPrices, OpenInterest, OpenInterestHist,
    OrderBook, PriceStats, SymbolPrice, TakerLongShortRatio, Tickers, Trades,
};
use crate::client::Client;
use crate::errors::Result;
use std::collections::BTreeMap;
use serde_json::Value;
use crate::api::API;
use crate::api::Futures;

#[derive(Clone)]
pub struct FuturesMarket {
    pub client: Client,
    pub recv_window: u64,
}

impl FuturesMarket {
    // Order book (Default 100; max 1000)
    pub fn get_depth<S>(&self, symbol: S) -> Result<OrderBook>
    where
        S: Into<String>,
    {
        let mut parameters: BTreeMap<String, String> = BTreeMap::new();

        parameters.insert("symbol".into(), symbol.into());
        let request = build_request(parameters);

        self.client.get(API::Futures(Futures::Depth), Some(request))
    }

    // Order book at a custom depth. Currently supported values
    // are 5, 10, 20, 50, 100, 500, 1000
    pub fn get_custom_depth<S>(&self, symbol: S, depth: u64) -> Result<OrderBook>
    where
        S: Into<String>,
    {
        let mut parameters: BTreeMap<String, String> = BTreeMap::new();
        parameters.insert("symbol".into(), symbol.into());
        parameters.insert("limit".into(), depth.to_string());
        let request = build_request(parameters);
        self.client.get(API::Futures(Futures::Depth), Some(request))
    }

    pub fn get_trades<S>(&self, symbol: S) -> Result<Trades>
    where
        S: Into<String>,
    {
        let mut parameters: BTreeMap<String, String> = BTreeMap::new();
        parameters.insert("symbol".into(), symbol.into());
        let request = build_request(parameters);
        self.client
            .get(API::Futures(Futures::Trades), Some(request))
    }

    /// Older trades (`GET /fapi/v1/historicalTrades`). Requires an API key; only the last
    /// month of trades is available and `limit` is capped at 500 (default 100).
    pub fn get_historical_trades<S1, S2, S3>(
        &self, symbol: S1, from_id: S2, limit: S3,
    ) -> Result<Trades>
    where
        S1: Into<String>,
        S2: Into<Option<u64>>,
        S3: Into<Option<u16>>,
    {
        let mut parameters: BTreeMap<String, String> = BTreeMap::new();

        parameters.insert("symbol".into(), symbol.into());

        if let Some(lt) = limit.into() {
            parameters.insert("limit".into(), format!("{}", lt));
        }
        if let Some(fi) = from_id.into() {
            parameters.insert("fromId".into(), format!("{}", fi));
        }

        let request = build_request(parameters);

        self.client
            .get_with_api_key(API::Futures(Futures::HistoricalTrades), Some(request))
    }

    pub fn get_agg_trades<S1, S2, S3, S4, S5>(
        &self, symbol: S1, from_id: S2, start_time: S3, end_time: S4, limit: S5,
    ) -> Result<AggTrades>
    where
        S1: Into<String>,
        S2: Into<Option<u64>>,
        S3: Into<Option<u64>>,
        S4: Into<Option<u64>>,
        S5: Into<Option<u16>>,
    {
        let mut parameters: BTreeMap<String, String> = BTreeMap::new();

        parameters.insert("symbol".into(), symbol.into());

        // Add three optional parameters
        if let Some(lt) = limit.into() {
            parameters.insert("limit".into(), format!("{}", lt));
        }
        if let Some(st) = start_time.into() {
            parameters.insert("startTime".into(), format!("{}", st));
        }
        if let Some(et) = end_time.into() {
            parameters.insert("endTime".into(), format!("{}", et));
        }
        if let Some(fi) = from_id.into() {
            parameters.insert("fromId".into(), format!("{}", fi));
        }

        let request = build_request(parameters);

        self.client
            .get(API::Futures(Futures::AggTrades), Some(request))
    }

    // Returns up to 'limit' klines for given symbol and interval ("1m", "5m", ...)
    // https://github.com/binance-exchange/binance-official-api-docs/blob/master/rest-api.md#klinecandlestick-data
    pub fn get_klines<S1, S2, S3, S4, S5>(
        &self, symbol: S1, interval: S2, limit: S3, start_time: S4, end_time: S5,
    ) -> Result<KlineSummaries>
    where
        S1: Into<String>,
        S2: Into<String>,
        S3: Into<Option<u16>>,
        S4: Into<Option<u64>>,
        S5: Into<Option<u64>>,
    {
        let mut parameters: BTreeMap<String, String> = BTreeMap::new();

        parameters.insert("symbol".into(), symbol.into());
        parameters.insert("interval".into(), interval.into());

        // Add three optional parameters
        if let Some(lt) = limit.into() {
            parameters.insert("limit".into(), format!("{}", lt));
        }
        if let Some(st) = start_time.into() {
            parameters.insert("startTime".into(), format!("{}", st));
        }
        if let Some(et) = end_time.into() {
            parameters.insert("endTime".into(), format!("{}", et));
        }

        let request = build_request(parameters);

        let data: Vec<Vec<Value>> = self
            .client
            .get(API::Futures(Futures::Klines), Some(request))?;

        let klines = KlineSummaries::AllKlineSummaries(
            data.iter()
                .map(|row| row.try_into())
                .collect::<Result<Vec<KlineSummary>>>()?,
        );

        Ok(klines)
    }

    // 24hr ticker price change statistics
    pub fn get_24h_price_stats<S>(&self, symbol: S) -> Result<PriceStats>
    where
        S: Into<String>,
    {
        let mut parameters: BTreeMap<String, String> = BTreeMap::new();

        parameters.insert("symbol".into(), symbol.into());
        let request = build_request(parameters);

        self.client
            .get(API::Futures(Futures::Ticker24hr), Some(request))
    }

    // 24hr ticker price change statistics for all symbols
    pub fn get_all_24h_price_stats(&self) -> Result<Vec<PriceStats>> {
        self.client.get(API::Futures(Futures::Ticker24hr), None)
    }

    // Latest price for ONE symbol.
    pub fn get_price<S>(&self, symbol: S) -> Result<SymbolPrice>
    where
        S: Into<String>,
    {
        let mut parameters: BTreeMap<String, String> = BTreeMap::new();

        parameters.insert("symbol".into(), symbol.into());
        let request = build_request(parameters);

        self.client
            .get(API::Futures(Futures::TickerPrice), Some(request))
    }

    // Latest price for all symbols.
    pub fn get_all_prices(&self) -> Result<crate::model::Prices> {
        self.client.get(API::Futures(Futures::TickerPrice), None)
    }

    // Symbols order book ticker
    // -> Best price/qty on the order book for ALL symbols.
    pub fn get_all_book_tickers(&self) -> Result<BookTickers> {
        self.client.get(API::Futures(Futures::BookTicker), None)
    }

    // -> Best price/qty on the order book for ONE symbol
    pub fn get_book_ticker<S>(&self, symbol: S) -> Result<Tickers>
    where
        S: Into<String>,
    {
        let mut parameters: BTreeMap<String, String> = BTreeMap::new();
        parameters.insert("symbol".into(), symbol.into());
        let request = build_request(parameters);
        self.client
            .get(API::Futures(Futures::BookTicker), Some(request))
    }

    pub fn get_mark_prices(&self) -> Result<MarkPrices> {
        self.client.get(API::Futures(Futures::PremiumIndex), None)
    }

    /// Mark price, index price and funding rate of one symbol (`GET /fapi/v1/premiumIndex`).
    pub fn get_mark_price<S>(&self, symbol: S) -> Result<MarkPrice>
    where
        S: Into<String>,
    {
        let mut parameters: BTreeMap<String, String> = BTreeMap::new();
        parameters.insert("symbol".into(), symbol.into());
        let request = build_request(parameters);
        self.client
            .get(API::Futures(Futures::PremiumIndex), Some(request))
    }

    /// Funding rate history (`GET /fapi/v1/fundingRate`), all symbols when `None`.
    pub fn get_funding_rate_history<S, T1, T2, L>(
        &self, symbol: S, start_time: T1, end_time: T2, limit: L,
    ) -> Result<Vec<FundingRate>>
    where
        S: Into<Option<String>>,
        T1: Into<Option<u64>>,
        T2: Into<Option<u64>>,
        L: Into<Option<u16>>,
    {
        let mut parameters: BTreeMap<String, String> = BTreeMap::new();
        if let Some(symbol) = symbol.into() {
            parameters.insert("symbol".into(), symbol);
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
        let request = build_request(parameters);
        self.client
            .get(API::Futures(Futures::FundingRate), Some(request))
    }

    /// Funding rate caps, floors and intervals for symbols with adjusted funding
    /// (`GET /fapi/v1/fundingInfo`).
    pub fn get_funding_info(&self) -> Result<Vec<FundingInfo>> {
        self.client.get(API::Futures(Futures::FundingInfo), None)
    }

    /// `GET /fapi/v1/allForceOrders` was removed by Binance; this call always fails.
    /// Use [`crate::futures::account::FuturesAccount::get_force_orders`] for the account's
    /// liquidation orders or the `<symbol>@forceOrder` / `!forceOrder@arr` streams.
    #[deprecated(note = "endpoint removed by Binance, use FuturesAccount::get_force_orders")]
    pub fn get_all_liquidation_orders(&self) -> Result<LiquidationOrders> {
        self.client.get(API::Futures(Futures::AllForceOrders), None)
    }

    pub fn open_interest<S>(&self, symbol: S) -> Result<OpenInterest>
    where
        S: Into<String>,
    {
        let mut parameters: BTreeMap<String, String> = BTreeMap::new();
        parameters.insert("symbol".into(), symbol.into());
        let request = build_request(parameters);
        self.client
            .get(API::Futures(Futures::OpenInterest), Some(request))
    }

    pub fn open_interest_statistics<S1, S2, S3, S4, S5>(
        &self, symbol: S1, period: S2, limit: S3, start_time: S4, end_time: S5,
    ) -> Result<Vec<OpenInterestHist>>
    where
        S1: Into<String>,
        S2: Into<String>,
        S3: Into<Option<u16>>,
        S4: Into<Option<u64>>,
        S5: Into<Option<u64>>,
    {
        let mut parameters: BTreeMap<String, String> = BTreeMap::new();
        parameters.insert("symbol".into(), symbol.into());
        parameters.insert("period".into(), period.into());

        if let Some(lt) = limit.into() {
            parameters.insert("limit".into(), format!("{}", lt));
        }
        if let Some(st) = start_time.into() {
            parameters.insert("startTime".into(), format!("{}", st));
        }
        if let Some(et) = end_time.into() {
            parameters.insert("endTime".into(), format!("{}", et));
        }

        let request = build_request(parameters);
        self.client
            .get(API::Futures(Futures::OpenInterestHist), Some(request))
    }

    #[allow(clippy::too_many_arguments)]
    fn get_klines_from<S1, S2, S3, S4>(
        &self, endpoint: Futures, key: &str, symbol_or_pair: S1, extra: Option<(&str, String)>,
        interval: S2, limit: S3, start_time: S4, end_time: S4,
    ) -> Result<KlineSummaries>
    where
        S1: Into<String>,
        S2: Into<String>,
        S3: Into<Option<u16>>,
        S4: Into<Option<u64>>,
    {
        let mut parameters: BTreeMap<String, String> = BTreeMap::new();
        parameters.insert(key.into(), symbol_or_pair.into());
        if let Some((k, v)) = extra {
            parameters.insert(k.into(), v);
        }
        parameters.insert("interval".into(), interval.into());
        if let Some(lt) = limit.into() {
            parameters.insert("limit".into(), lt.to_string());
        }
        if let Some(st) = start_time.into() {
            parameters.insert("startTime".into(), st.to_string());
        }
        if let Some(et) = end_time.into() {
            parameters.insert("endTime".into(), et.to_string());
        }
        let request = build_request(parameters);
        let data: Vec<Vec<Value>> = self.client.get(API::Futures(endpoint), Some(request))?;
        Ok(KlineSummaries::AllKlineSummaries(
            data.iter()
                .map(|row| row.try_into())
                .collect::<Result<Vec<KlineSummary>>>()?,
        ))
    }

    /// Klines of a contract type on a pair (`GET /fapi/v1/continuousKlines`), e.g.
    /// `("BTCUSDT", ContractType::Perpetual, "1h", ...)`.
    pub fn get_continuous_klines<S1, S2, S3, S4>(
        &self, pair: S1, contract_type: crate::futures::account::ContractType, interval: S2,
        limit: S3, start_time: S4, end_time: S4,
    ) -> Result<KlineSummaries>
    where
        S1: Into<String>,
        S2: Into<String>,
        S3: Into<Option<u16>>,
        S4: Into<Option<u64>>,
    {
        self.get_klines_from(
            Futures::ContinuousKlines,
            "pair",
            pair,
            Some(("contractType", String::from(contract_type))),
            interval,
            limit,
            start_time,
            end_time,
        )
    }

    /// Index price klines of a pair (`GET /fapi/v1/indexPriceKlines`). Only the OHLC and
    /// time fields are meaningful; volume fields are zero.
    pub fn get_index_price_klines<S1, S2, S3, S4>(
        &self, pair: S1, interval: S2, limit: S3, start_time: S4, end_time: S4,
    ) -> Result<KlineSummaries>
    where
        S1: Into<String>,
        S2: Into<String>,
        S3: Into<Option<u16>>,
        S4: Into<Option<u64>>,
    {
        self.get_klines_from(
            Futures::IndexPriceKlines,
            "pair",
            pair,
            None,
            interval,
            limit,
            start_time,
            end_time,
        )
    }

    /// Mark price klines of a symbol (`GET /fapi/v1/markPriceKlines`).
    pub fn get_mark_price_klines<S1, S2, S3, S4>(
        &self, symbol: S1, interval: S2, limit: S3, start_time: S4, end_time: S4,
    ) -> Result<KlineSummaries>
    where
        S1: Into<String>,
        S2: Into<String>,
        S3: Into<Option<u16>>,
        S4: Into<Option<u64>>,
    {
        self.get_klines_from(
            Futures::MarkPriceKlines,
            "symbol",
            symbol,
            None,
            interval,
            limit,
            start_time,
            end_time,
        )
    }

    /// Premium index klines of a symbol (`GET /fapi/v1/premiumIndexKlines`).
    pub fn get_premium_index_klines<S1, S2, S3, S4>(
        &self, symbol: S1, interval: S2, limit: S3, start_time: S4, end_time: S4,
    ) -> Result<KlineSummaries>
    where
        S1: Into<String>,
        S2: Into<String>,
        S3: Into<Option<u16>>,
        S4: Into<Option<u64>>,
    {
        self.get_klines_from(
            Futures::PremiumIndexKlines,
            "symbol",
            symbol,
            None,
            interval,
            limit,
            start_time,
            end_time,
        )
    }

    #[allow(clippy::too_many_arguments)]
    fn get_futures_data<T, S1, S2, S3, S4, S5>(
        &self, endpoint: Futures, key: &str, symbol: S1, period: S2, limit: S3, start_time: S4,
        end_time: S5,
    ) -> Result<T>
    where
        T: serde::de::DeserializeOwned,
        S1: Into<String>,
        S2: Into<String>,
        S3: Into<Option<u16>>,
        S4: Into<Option<u64>>,
        S5: Into<Option<u64>>,
    {
        let mut parameters: BTreeMap<String, String> = BTreeMap::new();
        parameters.insert(key.into(), symbol.into());
        parameters.insert("period".into(), period.into());
        if let Some(lt) = limit.into() {
            parameters.insert("limit".into(), lt.to_string());
        }
        if let Some(st) = start_time.into() {
            parameters.insert("startTime".into(), st.to_string());
        }
        if let Some(et) = end_time.into() {
            parameters.insert("endTime".into(), et.to_string());
        }
        let request = build_request(parameters);
        self.client.get(API::Futures(endpoint), Some(request))
    }

    /// Top trader long/short ratio by accounts (`GET /futures/data/topLongShortAccountRatio`).
    /// `period`: `5m`, `15m`, `30m`, `1h`, `2h`, `4h`, `6h`, `12h`, `1d`; last 30 days only.
    pub fn top_long_short_account_ratio<S1, S2, S3, S4, S5>(
        &self, symbol: S1, period: S2, limit: S3, start_time: S4, end_time: S5,
    ) -> Result<Vec<LongShortRatio>>
    where
        S1: Into<String>,
        S2: Into<String>,
        S3: Into<Option<u16>>,
        S4: Into<Option<u64>>,
        S5: Into<Option<u64>>,
    {
        self.get_futures_data(
            Futures::TopLongShortAccountRatio,
            "symbol",
            symbol,
            period,
            limit,
            start_time,
            end_time,
        )
    }

    /// Top trader long/short ratio by positions (`GET /futures/data/topLongShortPositionRatio`).
    pub fn top_long_short_position_ratio<S1, S2, S3, S4, S5>(
        &self, symbol: S1, period: S2, limit: S3, start_time: S4, end_time: S5,
    ) -> Result<Vec<LongShortRatio>>
    where
        S1: Into<String>,
        S2: Into<String>,
        S3: Into<Option<u16>>,
        S4: Into<Option<u64>>,
        S5: Into<Option<u64>>,
    {
        self.get_futures_data(
            Futures::TopLongShortPositionRatio,
            "symbol",
            symbol,
            period,
            limit,
            start_time,
            end_time,
        )
    }

    /// Global long/short account ratio (`GET /futures/data/globalLongShortAccountRatio`).
    pub fn global_long_short_account_ratio<S1, S2, S3, S4, S5>(
        &self, symbol: S1, period: S2, limit: S3, start_time: S4, end_time: S5,
    ) -> Result<Vec<LongShortRatio>>
    where
        S1: Into<String>,
        S2: Into<String>,
        S3: Into<Option<u16>>,
        S4: Into<Option<u64>>,
        S5: Into<Option<u64>>,
    {
        self.get_futures_data(
            Futures::GlobalLongShortAccountRatio,
            "symbol",
            symbol,
            period,
            limit,
            start_time,
            end_time,
        )
    }

    /// Taker buy/sell volume ratio (`GET /futures/data/takerlongshortRatio`).
    pub fn taker_long_short_ratio<S1, S2, S3, S4, S5>(
        &self, symbol: S1, period: S2, limit: S3, start_time: S4, end_time: S5,
    ) -> Result<Vec<TakerLongShortRatio>>
    where
        S1: Into<String>,
        S2: Into<String>,
        S3: Into<Option<u16>>,
        S4: Into<Option<u64>>,
        S5: Into<Option<u64>>,
    {
        self.get_futures_data(
            Futures::TakerlongshortRatio,
            "symbol",
            symbol,
            period,
            limit,
            start_time,
            end_time,
        )
    }

    /// Basis between a pair's futures contract and its index (`GET /futures/data/basis`).
    pub fn basis<S1, S2, S3, S4, S5>(
        &self, pair: S1, contract_type: crate::futures::account::ContractType, period: S2,
        limit: S3, start_time: S4, end_time: S5,
    ) -> Result<Vec<Basis>>
    where
        S1: Into<String>,
        S2: Into<String>,
        S3: Into<Option<u16>>,
        S4: Into<Option<u64>>,
        S5: Into<Option<u64>>,
    {
        let mut parameters: BTreeMap<String, String> = BTreeMap::new();
        parameters.insert("pair".into(), pair.into());
        parameters.insert("contractType".into(), String::from(contract_type));
        parameters.insert("period".into(), period.into());
        if let Some(lt) = limit.into() {
            parameters.insert("limit".into(), lt.to_string());
        }
        if let Some(st) = start_time.into() {
            parameters.insert("startTime".into(), st.to_string());
        }
        if let Some(et) = end_time.into() {
            parameters.insert("endTime".into(), et.to_string());
        }
        let request = build_request(parameters);
        self.client.get(API::Futures(Futures::Basis), Some(request))
    }

    fn get_optional_symbol<T>(&self, endpoint: Futures, symbol: Option<String>) -> Result<T>
    where
        T: serde::de::DeserializeOwned,
    {
        let mut parameters: BTreeMap<String, String> = BTreeMap::new();
        if let Some(symbol) = symbol {
            parameters.insert("symbol".into(), symbol);
        }
        let request = build_request(parameters);
        self.client.get(API::Futures(endpoint), Some(request))
    }

    /// Insurance fund balance snapshot (`GET /fapi/v1/insuranceBalance`).
    pub fn get_insurance_balance<S>(&self, symbol: S) -> Result<Vec<InsuranceBalance>>
    where
        S: Into<Option<String>>,
    {
        match symbol.into() {
            Some(symbol) => self
                .get_optional_symbol::<InsuranceBalance>(Futures::InsuranceBalance, Some(symbol))
                .map(|b| vec![b]),
            None => self.get_optional_symbol(Futures::InsuranceBalance, None),
        }
    }

    /// ADL risk rating of a symbol (`GET /fapi/v1/symbolAdlRisk`), updated every 30 minutes.
    pub fn get_symbol_adl_risk<S>(&self, symbol: S) -> Result<Vec<AdlRisk>>
    where
        S: Into<Option<String>>,
    {
        match symbol.into() {
            Some(symbol) => self
                .get_optional_symbol::<AdlRisk>(Futures::SymbolAdlRisk, Some(symbol))
                .map(|r| vec![r]),
            None => self.get_optional_symbol(Futures::SymbolAdlRisk, None),
        }
    }

    /// Order book of Retail Price Improvement orders (`GET /fapi/v1/rpiDepth`).
    pub fn get_rpi_depth<S>(&self, symbol: S) -> Result<OrderBook>
    where
        S: Into<String>,
    {
        let mut parameters: BTreeMap<String, String> = BTreeMap::new();
        parameters.insert("symbol".into(), symbol.into());
        let request = build_request(parameters);
        self.client
            .get(API::Futures(Futures::RpiDepth), Some(request))
    }

    /// Composite index information (`GET /fapi/v1/indexInfo`), all indexes when `None`.
    pub fn get_index_info<S>(&self, symbol: S) -> Result<Vec<IndexInfo>>
    where
        S: Into<Option<String>>,
    {
        match symbol.into() {
            Some(symbol) => self
                .get_optional_symbol::<IndexInfo>(Futures::IndexInfo, Some(symbol))
                .map(|i| vec![i]),
            None => self.get_optional_symbol(Futures::IndexInfo, None),
        }
    }

    /// Multi-assets mode asset index (`GET /fapi/v1/assetIndex`), all assets when `None`.
    pub fn get_asset_index<S>(&self, symbol: S) -> Result<Vec<AssetIndex>>
    where
        S: Into<Option<String>>,
    {
        match symbol.into() {
            Some(symbol) => self
                .get_optional_symbol::<AssetIndex>(Futures::AssetIndex, Some(symbol))
                .map(|i| vec![i]),
            None => self.get_optional_symbol(Futures::AssetIndex, None),
        }
    }

    /// Index price constituents (`GET /fapi/v1/constituents`).
    pub fn get_index_constituents<S>(&self, symbol: S) -> Result<IndexConstituents>
    where
        S: Into<String>,
    {
        self.get_optional_symbol(Futures::Constituents, Some(symbol.into()))
    }

    /// Quarterly contract settlement prices of a pair (`GET /futures/data/delivery-price`).
    pub fn get_delivery_price<S>(&self, pair: S) -> Result<Vec<DeliveryPrice>>
    where
        S: Into<String>,
    {
        let mut parameters: BTreeMap<String, String> = BTreeMap::new();
        parameters.insert("pair".into(), pair.into());
        let request = build_request(parameters);
        self.client
            .get(API::Futures(Futures::DeliveryPrice), Some(request))
    }
}
