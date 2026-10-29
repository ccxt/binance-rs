use crate::util::{build_request, json_string_array};
use crate::model::{
    AggTrade, AveragePrice, BookTickers, KlineSummaries, KlineSummary, OrderBook, PriceStats,
    Prices, SymbolPrice, TickerStats, Tickers, Trade,
};
use crate::client::Client;
use crate::errors::Result;
use std::collections::BTreeMap;
use serde_json::Value;
use crate::api::API;
use crate::api::Spot;

#[derive(Clone)]
pub struct Market {
    pub client: Client,
    pub recv_window: u64,
}

// Market Data endpoints
impl Market {
    // Order book at the default depth of 100
    pub fn get_depth<S>(&self, symbol: S) -> Result<OrderBook>
    where
        S: Into<String>,
    {
        let mut parameters: BTreeMap<String, String> = BTreeMap::new();
        parameters.insert("symbol".into(), symbol.into());
        let request = build_request(parameters);
        self.client.get(API::Spot(Spot::Depth), Some(request))
    }

    // Order book at a custom depth. Currently supported values
    // are 5, 10, 20, 50, 100, 500, 1000 and 5000
    pub fn get_custom_depth<S>(&self, symbol: S, depth: u64) -> Result<OrderBook>
    where
        S: Into<String>,
    {
        let mut parameters: BTreeMap<String, String> = BTreeMap::new();
        parameters.insert("symbol".into(), symbol.into());
        parameters.insert("limit".into(), depth.to_string());
        let request = build_request(parameters);
        self.client.get(API::Spot(Spot::Depth), Some(request))
    }

    // Latest price for ALL symbols.
    pub fn get_all_prices(&self) -> Result<Prices> {
        self.client.get(API::Spot(Spot::Price), None)
    }

    // Latest price for ONE symbol.
    pub fn get_price<S>(&self, symbol: S) -> Result<SymbolPrice>
    where
        S: Into<String>,
    {
        let mut parameters: BTreeMap<String, String> = BTreeMap::new();
        parameters.insert("symbol".into(), symbol.into());
        let request = build_request(parameters);
        self.client.get(API::Spot(Spot::Price), Some(request))
    }

    // Average price for ONE symbol.
    pub fn get_average_price<S>(&self, symbol: S) -> Result<AveragePrice>
    where
        S: Into<String>,
    {
        let mut parameters: BTreeMap<String, String> = BTreeMap::new();
        parameters.insert("symbol".into(), symbol.into());
        let request = build_request(parameters);
        self.client.get(API::Spot(Spot::AvgPrice), Some(request))
    }

    // Symbols order book ticker
    // -> Best price/qty on the order book for ALL symbols.
    pub fn get_all_book_tickers(&self) -> Result<BookTickers> {
        self.client.get(API::Spot(Spot::BookTicker), None)
    }

    // -> Best price/qty on the order book for ONE symbol
    pub fn get_book_ticker<S>(&self, symbol: S) -> Result<Tickers>
    where
        S: Into<String>,
    {
        let mut parameters: BTreeMap<String, String> = BTreeMap::new();
        parameters.insert("symbol".into(), symbol.into());
        let request = build_request(parameters);
        self.client.get(API::Spot(Spot::BookTicker), Some(request))
    }

    // 24hr ticker price change statistics
    pub fn get_24h_price_stats<S>(&self, symbol: S) -> Result<PriceStats>
    where
        S: Into<String>,
    {
        let mut parameters: BTreeMap<String, String> = BTreeMap::new();
        parameters.insert("symbol".into(), symbol.into());
        let request = build_request(parameters);
        self.client.get(API::Spot(Spot::Ticker24hr), Some(request))
    }

    // 24hr ticker price change statistics for all symbols
    pub fn get_all_24h_price_stats(&self) -> Result<Vec<PriceStats>> {
        self.client.get(API::Spot(Spot::Ticker24hr), None)
    }

    /// Get aggregated historical trades.
    ///
    /// If you provide start_time, you also need to provide end_time.
    /// If from_id, start_time and end_time are omitted, the most recent trades are fetched.
    pub fn get_agg_trades<S1, S2, S3, S4, S5>(
        &self, symbol: S1, from_id: S2, start_time: S3, end_time: S4, limit: S5,
    ) -> Result<Vec<AggTrade>>
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

        self.client.get(API::Spot(Spot::AggTrades), Some(request))
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
        let data: Vec<Vec<Value>> = self.client.get(API::Spot(Spot::Klines), Some(request))?;

        let klines = KlineSummaries::AllKlineSummaries(
            data.iter()
                .map(|row| row.try_into())
                .collect::<Result<Vec<KlineSummary>>>()?,
        );

        Ok(klines)
    }

    /// Recent trades for a symbol (`GET /api/v3/trades`). `limit` defaults to 500, max 1000.
    pub fn get_trades<S, L>(&self, symbol: S, limit: L) -> Result<Vec<Trade>>
    where
        S: Into<String>,
        L: Into<Option<u16>>,
    {
        let mut parameters: BTreeMap<String, String> = BTreeMap::new();
        parameters.insert("symbol".into(), symbol.into());
        if let Some(lt) = limit.into() {
            parameters.insert("limit".into(), lt.to_string());
        }
        let request = build_request(parameters);
        self.client.get(API::Spot(Spot::Trades), Some(request))
    }

    /// Older trades (`GET /api/v3/historicalTrades`). Requires an API key.
    ///
    /// `from_id` is the trade id to fetch from; the most recent trades are returned when omitted.
    pub fn get_historical_trades<S, F, L>(
        &self, symbol: S, from_id: F, limit: L,
    ) -> Result<Vec<Trade>>
    where
        S: Into<String>,
        F: Into<Option<u64>>,
        L: Into<Option<u16>>,
    {
        let mut parameters: BTreeMap<String, String> = BTreeMap::new();
        parameters.insert("symbol".into(), symbol.into());
        if let Some(fi) = from_id.into() {
            parameters.insert("fromId".into(), fi.to_string());
        }
        if let Some(lt) = limit.into() {
            parameters.insert("limit".into(), lt.to_string());
        }
        let request = build_request(parameters);
        self.client
            .get_with_api_key(API::Spot(Spot::HistoricalTrades), Some(request))
    }

    /// Klines optimised for presentation (`GET /api/v3/uiKlines`).
    ///
    /// Same parameters and response as [`Market::get_klines`]; `time_zone` (e.g. `"8"` or
    /// `"-1:00"`) shifts the kline boundaries, default `0` (UTC).
    pub fn get_ui_klines<S1, S2, S3, S4, S5, S6>(
        &self, symbol: S1, interval: S2, limit: S3, start_time: S4, end_time: S5, time_zone: S6,
    ) -> Result<KlineSummaries>
    where
        S1: Into<String>,
        S2: Into<String>,
        S3: Into<Option<u16>>,
        S4: Into<Option<u64>>,
        S5: Into<Option<u64>>,
        S6: Into<Option<String>>,
    {
        let mut parameters: BTreeMap<String, String> = BTreeMap::new();
        parameters.insert("symbol".into(), symbol.into());
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
        if let Some(tz) = time_zone.into() {
            parameters.insert("timeZone".into(), tz);
        }
        let request = build_request(parameters);
        let data: Vec<Vec<Value>> = self.client.get(API::Spot(Spot::UiKlines), Some(request))?;
        Ok(KlineSummaries::AllKlineSummaries(
            data.iter()
                .map(|row| row.try_into())
                .collect::<Result<Vec<KlineSummary>>>()?,
        ))
    }

    /// Price change statistics for a trading day (`GET /api/v3/ticker/tradingDay`).
    ///
    /// `time_zone` defaults to `0` (UTC); `mini` selects the `MINI` response type.
    pub fn get_trading_day_ticker<S, T>(
        &self, symbol: S, time_zone: T, mini: bool,
    ) -> Result<TickerStats>
    where
        S: Into<String>,
        T: Into<Option<String>>,
    {
        let mut parameters: BTreeMap<String, String> = BTreeMap::new();
        parameters.insert("symbol".into(), symbol.into());
        if let Some(tz) = time_zone.into() {
            parameters.insert("timeZone".into(), tz);
        }
        if mini {
            parameters.insert("type".into(), "MINI".into());
        }
        let request = build_request(parameters);
        self.client
            .get(API::Spot(Spot::TickerTradingDay), Some(request))
    }

    /// Trading day statistics for several symbols at once (max 100).
    pub fn get_trading_day_tickers<S, T>(
        &self, symbols: &[S], time_zone: T, mini: bool,
    ) -> Result<Vec<TickerStats>>
    where
        S: AsRef<str>,
        T: Into<Option<String>>,
    {
        let mut parameters: BTreeMap<String, String> = BTreeMap::new();
        parameters.insert("symbols".into(), json_string_array(symbols));
        if let Some(tz) = time_zone.into() {
            parameters.insert("timeZone".into(), tz);
        }
        if mini {
            parameters.insert("type".into(), "MINI".into());
        }
        let request = build_request(parameters);
        self.client
            .get(API::Spot(Spot::TickerTradingDay), Some(request))
    }

    /// Rolling window price change statistics (`GET /api/v3/ticker`).
    ///
    /// `window_size` examples: `1m`..`59m`, `1h`..`23h`, `1d`..`7d` (default `1d`).
    pub fn get_rolling_window_ticker<S, W>(
        &self, symbol: S, window_size: W, mini: bool,
    ) -> Result<TickerStats>
    where
        S: Into<String>,
        W: Into<Option<String>>,
    {
        let mut parameters: BTreeMap<String, String> = BTreeMap::new();
        parameters.insert("symbol".into(), symbol.into());
        if let Some(ws) = window_size.into() {
            parameters.insert("windowSize".into(), ws);
        }
        if mini {
            parameters.insert("type".into(), "MINI".into());
        }
        let request = build_request(parameters);
        self.client.get(API::Spot(Spot::Ticker), Some(request))
    }

    /// Rolling window statistics for several symbols at once (max 100).
    pub fn get_rolling_window_tickers<S, W>(
        &self, symbols: &[S], window_size: W, mini: bool,
    ) -> Result<Vec<TickerStats>>
    where
        S: AsRef<str>,
        W: Into<Option<String>>,
    {
        let mut parameters: BTreeMap<String, String> = BTreeMap::new();
        parameters.insert("symbols".into(), json_string_array(symbols));
        if let Some(ws) = window_size.into() {
            parameters.insert("windowSize".into(), ws);
        }
        if mini {
            parameters.insert("type".into(), "MINI".into());
        }
        let request = build_request(parameters);
        self.client.get(API::Spot(Spot::Ticker), Some(request))
    }

    /// Latest prices for a list of symbols.
    pub fn get_prices_for_symbols<S: AsRef<str>>(&self, symbols: &[S]) -> Result<Vec<SymbolPrice>> {
        let mut parameters: BTreeMap<String, String> = BTreeMap::new();
        parameters.insert("symbols".into(), json_string_array(symbols));
        let request = build_request(parameters);
        self.client.get(API::Spot(Spot::Price), Some(request))
    }

    /// 24hr statistics for a list of symbols.
    pub fn get_24h_price_stats_for_symbols<S: AsRef<str>>(
        &self, symbols: &[S],
    ) -> Result<Vec<PriceStats>> {
        let mut parameters: BTreeMap<String, String> = BTreeMap::new();
        parameters.insert("symbols".into(), json_string_array(symbols));
        let request = build_request(parameters);
        self.client.get(API::Spot(Spot::Ticker24hr), Some(request))
    }
}
