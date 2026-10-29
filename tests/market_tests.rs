use binance::api::*;
use binance::config::*;
use binance::market::*;
use binance::model::*;

#[cfg(test)]
mod tests {
    use super::*;
    use mockito::{Server, Matcher};
    use float_cmp::*;

    #[test]
    fn get_depth() {
        let mut server = Server::new();
        let mock_get_depth = server
            .mock("GET", "/api/v3/depth")
            .with_header("content-type", "application/json;charset=UTF-8")
            .match_query(Matcher::Regex("symbol=LTCBTC".into()))
            .with_body_from_file("tests/mocks/market/get_depth.json")
            .create();

        let config = Config::default().set_rest_api_endpoint(server.url());
        let market: Market = Binance::new_with_config(None, None, &config);

        let order_book = market.get_depth("LTCBTC").unwrap();
        mock_get_depth.assert();

        assert_eq!(order_book.last_update_id, 1027024);
        assert_eq!(order_book.bids[0], Bids::new(4.00000000, 431.00000000));
    }

    #[test]
    fn get_custom_depth() {
        let mut server = Server::new();
        let mock_get_custom_depth = server
            .mock("GET", "/api/v3/depth")
            .with_header("content-type", "application/json;charset=UTF-8")
            .match_query(Matcher::Regex("limit=10&symbol=LTCBTC".into()))
            .with_body_from_file("tests/mocks/market/get_depth.json")
            .create();

        let config = Config::default().set_rest_api_endpoint(server.url());
        let market: Market = Binance::new_with_config(None, None, &config);

        let order_book = market.get_custom_depth("LTCBTC", 10).unwrap();
        mock_get_custom_depth.assert();

        assert_eq!(order_book.last_update_id, 1027024);
        assert_eq!(order_book.bids[0], Bids::new(4.00000000, 431.00000000));
    }

    #[test]
    fn get_all_prices() {
        let mut server = Server::new();
        let mock_get_all_prices = server
            .mock("GET", "/api/v3/ticker/price")
            .with_header("content-type", "application/json;charset=UTF-8")
            .with_body_from_file("tests/mocks/market/get_all_prices.json")
            .create();

        let config = Config::default().set_rest_api_endpoint(server.url());
        let market: Market = Binance::new_with_config(None, None, &config);

        let prices: Prices = market.get_all_prices().unwrap();
        mock_get_all_prices.assert();

        match prices {
            binance::model::Prices::AllPrices(symbols) => {
                assert!(!symbols.is_empty());
                let first_symbol = symbols[0].clone();
                assert_eq!(first_symbol.symbol, "LTCBTC");
                assert!(approx_eq!(f64, first_symbol.price, 4.00000200, ulps = 2));
                let second_symbol = symbols[1].clone();
                assert_eq!(second_symbol.symbol, "ETHBTC");
                assert!(approx_eq!(f64, second_symbol.price, 0.07946600, ulps = 2));
            }
        }
    }

    #[test]
    fn get_price() {
        let mut server = Server::new();
        let mock_get_price = server
            .mock("GET", "/api/v3/ticker/price")
            .with_header("content-type", "application/json;charset=UTF-8")
            .match_query(Matcher::Regex("symbol=LTCBTC".into()))
            .with_body_from_file("tests/mocks/market/get_price.json")
            .create();

        let config = Config::default().set_rest_api_endpoint(server.url());
        let market: Market = Binance::new_with_config(None, None, &config);

        let symbol = market.get_price("LTCBTC").unwrap();
        mock_get_price.assert();

        assert_eq!(symbol.symbol, "LTCBTC");
        assert!(approx_eq!(f64, symbol.price, 4.00000200, ulps = 2));
    }

    #[test]
    fn get_average_price() {
        let mut server = Server::new();
        let mock_get_average_price = server
            .mock("GET", "/api/v3/avgPrice")
            .with_header("content-type", "application/json;charset=UTF-8")
            .match_query(Matcher::Regex("symbol=LTCBTC".into()))
            .with_body_from_file("tests/mocks/market/get_average_price.json")
            .create();

        let config = Config::default().set_rest_api_endpoint(server.url());
        let market: Market = Binance::new_with_config(None, None, &config);

        let symbol = market.get_average_price("LTCBTC").unwrap();
        mock_get_average_price.assert();

        assert_eq!(symbol.mins, 5);
        assert!(approx_eq!(f64, symbol.price, 9.35751834, ulps = 2));
    }

    #[test]
    fn get_all_book_tickers() {
        let mut server = Server::new();
        let mock_get_all_book_tickers = server
            .mock("GET", "/api/v3/ticker/bookTicker")
            .with_header("content-type", "application/json;charset=UTF-8")
            .with_body_from_file("tests/mocks/market/get_all_book_tickers.json")
            .create();

        let config = Config::default().set_rest_api_endpoint(server.url());
        let market: Market = Binance::new_with_config(None, None, &config);

        let book_tickers = market.get_all_book_tickers().unwrap();
        mock_get_all_book_tickers.assert();

        match book_tickers {
            binance::model::BookTickers::AllBookTickers(tickers) => {
                assert!(!tickers.is_empty());
                let first_ticker = tickers[0].clone();
                assert_eq!(first_ticker.symbol, "LTCBTC");
                assert!(approx_eq!(
                    f64,
                    first_ticker.bid_price,
                    4.00000000,
                    ulps = 2
                ));
                assert!(approx_eq!(
                    f64,
                    first_ticker.bid_qty,
                    431.00000000,
                    ulps = 2
                ));
                assert!(approx_eq!(
                    f64,
                    first_ticker.ask_price,
                    4.00000200,
                    ulps = 2
                ));
                assert!(approx_eq!(f64, first_ticker.ask_qty, 9.00000000, ulps = 2));
                let second_ticker = tickers[1].clone();
                assert_eq!(second_ticker.symbol, "ETHBTC");
                assert!(approx_eq!(
                    f64,
                    second_ticker.bid_price,
                    0.07946700,
                    ulps = 2
                ));
                assert!(approx_eq!(f64, second_ticker.bid_qty, 9.00000000, ulps = 2));
                assert!(approx_eq!(
                    f64,
                    second_ticker.ask_price,
                    100000.00000000,
                    ulps = 2
                ));
                assert!(approx_eq!(
                    f64,
                    second_ticker.ask_qty,
                    1000.00000000,
                    ulps = 2
                ));
            }
        }
    }

    #[test]
    fn get_book_ticker() {
        let mut server = Server::new();
        let mock_get_book_ticker = server
            .mock("GET", "/api/v3/ticker/bookTicker")
            .with_header("content-type", "application/json;charset=UTF-8")
            .match_query(Matcher::Regex("symbol=LTCBTC".into()))
            .with_body_from_file("tests/mocks/market/get_book_ticker.json")
            .create();

        let config = Config::default().set_rest_api_endpoint(server.url());
        let market: Market = Binance::new_with_config(None, None, &config);

        let book_ticker = market.get_book_ticker("LTCBTC").unwrap();
        mock_get_book_ticker.assert();

        assert_eq!(book_ticker.symbol, "LTCBTC");
        assert!(approx_eq!(f64, book_ticker.bid_price, 4.00000000, ulps = 2));
        assert!(approx_eq!(f64, book_ticker.bid_qty, 431.00000000, ulps = 2));
        assert!(approx_eq!(f64, book_ticker.ask_price, 4.00000200, ulps = 2));
        assert!(approx_eq!(f64, book_ticker.ask_qty, 9.00000000, ulps = 2));
    }

    #[test]
    fn get_24h_price_stats() {
        let mut server = Server::new();
        let mock_get_24h_price_stats = server
            .mock("GET", "/api/v3/ticker/24hr")
            .with_header("content-type", "application/json;charset=UTF-8")
            .match_query(Matcher::Regex("symbol=BNBBTC".into()))
            .with_body_from_file("tests/mocks/market/get_24h_price_stats.json")
            .create();

        let config = Config::default().set_rest_api_endpoint(server.url());
        let market: Market = Binance::new_with_config(None, None, &config);

        let price_stats = market.get_24h_price_stats("BNBBTC").unwrap();
        mock_get_24h_price_stats.assert();

        assert_eq!(price_stats.symbol, "BNBBTC");
        assert_eq!(price_stats.price_change, "-94.99999800");
        assert_eq!(price_stats.price_change_percent, "-95.960");
        assert_eq!(price_stats.weighted_avg_price, "0.29628482");
        assert!(approx_eq!(
            f64,
            price_stats.prev_close_price,
            0.10002000,
            ulps = 2
        ));
        assert!(approx_eq!(
            f64,
            price_stats.last_price,
            4.00000200,
            ulps = 2
        ));
        assert!(approx_eq!(f64, price_stats.bid_price, 4.00000000, ulps = 2));
        assert!(approx_eq!(f64, price_stats.ask_price, 4.00000200, ulps = 2));
        assert!(approx_eq!(
            f64,
            price_stats.open_price,
            99.00000000,
            ulps = 2
        ));
        assert!(approx_eq!(
            f64,
            price_stats.high_price,
            100.00000000,
            ulps = 2
        ));
        assert!(approx_eq!(f64, price_stats.low_price, 0.10000000, ulps = 2));
        assert!(approx_eq!(f64, price_stats.volume, 8913.30000000, ulps = 2));
        assert_eq!(price_stats.open_time, 1499783499040);
        assert_eq!(price_stats.close_time, 1499869899040);
        assert_eq!(price_stats.first_id, 28385);
        assert_eq!(price_stats.last_id, 28460);
        assert_eq!(price_stats.count, 76);
    }

    #[test]
    fn get_all_24h_price_stats() {
        let mut server = Server::new();
        let mock_get_all_24h_price_stats = server
            .mock("GET", "/api/v3/ticker/24hr")
            .with_header("content-type", "application/json;charset=UTF-8")
            .with_body_from_file("tests/mocks/market/get_all_24h_price_stats.json")
            .create();

        let config = Config::default().set_rest_api_endpoint(server.url());
        let market: Market = Binance::new_with_config(None, None, &config);

        let prices_stats = market.get_all_24h_price_stats().unwrap();
        mock_get_all_24h_price_stats.assert();

        assert!(!prices_stats.is_empty());

        let price_stats = prices_stats[0].clone();

        assert_eq!(price_stats.symbol, "BNBBTC");
        assert_eq!(price_stats.price_change, "-94.99999800");
        assert_eq!(price_stats.price_change_percent, "-95.960");
        assert_eq!(price_stats.weighted_avg_price, "0.29628482");
        assert!(approx_eq!(
            f64,
            price_stats.prev_close_price,
            0.10002000,
            ulps = 2
        ));
        assert!(approx_eq!(
            f64,
            price_stats.last_price,
            4.00000200,
            ulps = 2
        ));
        assert!(approx_eq!(f64, price_stats.bid_price, 4.00000000, ulps = 2));
        assert!(approx_eq!(f64, price_stats.ask_price, 4.00000200, ulps = 2));
        assert!(approx_eq!(
            f64,
            price_stats.open_price,
            99.00000000,
            ulps = 2
        ));
        assert!(approx_eq!(
            f64,
            price_stats.high_price,
            100.00000000,
            ulps = 2
        ));
        assert!(approx_eq!(f64, price_stats.low_price, 0.10000000, ulps = 2));
        assert!(approx_eq!(f64, price_stats.volume, 8913.30000000, ulps = 2));
        assert_eq!(price_stats.open_time, 1499783499040);
        assert_eq!(price_stats.close_time, 1499869899040);
        assert_eq!(price_stats.first_id, 28385);
        assert_eq!(price_stats.last_id, 28460);
        assert_eq!(price_stats.count, 76);
    }

    #[test]
    fn get_klines() {
        let mut server = Server::new();
        let mock_get_klines = server
            .mock("GET", "/api/v3/klines")
            .with_header("content-type", "application/json;charset=UTF-8")
            .match_query(Matcher::Regex("interval=5m&limit=10&symbol=LTCBTC".into()))
            .with_body_from_file("tests/mocks/market/get_klines.json")
            .create();

        let config = Config::default().set_rest_api_endpoint(server.url());
        let market: Market = Binance::new_with_config(None, None, &config);

        let klines = market.get_klines("LTCBTC", "5m", 10, None, None).unwrap();
        mock_get_klines.assert();

        match klines {
            binance::model::KlineSummaries::AllKlineSummaries(klines) => {
                assert!(!klines.is_empty());
                let kline: KlineSummary = klines[0].clone();

                assert_eq!(kline.open_time, 1499040000000);
                assert_eq!(kline.open, "0.01634790");
                assert_eq!(kline.high, "0.80000000");
                assert_eq!(kline.low, "0.01575800");
                assert_eq!(kline.close, "0.01577100");
                assert_eq!(kline.volume, "148976.11427815");
                assert_eq!(kline.close_time, 1499644799999);
                assert_eq!(kline.quote_asset_volume, "2434.19055334");
                assert_eq!(kline.number_of_trades, 308);
                assert_eq!(kline.taker_buy_base_asset_volume, "1756.87402397");
                assert_eq!(kline.taker_buy_quote_asset_volume, "28.46694368");
            }
        }
    }

    #[test]
    fn get_trades_and_historical_trades() {
        let mut server = Server::new();
        let m1 = server
            .mock("GET", "/api/v3/trades")
            .match_query(Matcher::Regex("limit=1&symbol=LTCBTC".into()))
            .with_body_from_file("tests/mocks/market/trades.json")
            .create();
        let m2 = server
            .mock("GET", "/api/v3/historicalTrades")
            .match_header("x-mbx-apikey", "api-key")
            .match_query(Matcher::Regex("fromId=28457&limit=1&symbol=LTCBTC".into()))
            .with_body_from_file("tests/mocks/market/trades.json")
            .create();

        let config = Config::default().set_rest_api_endpoint(server.url());
        let market: Market = Binance::new_with_config(Some("api-key".into()), None, &config);

        let trades = market.get_trades("LTCBTC", 1).unwrap();
        assert_eq!(trades[0].id, 28457);
        assert!(approx_eq!(f64, trades[0].quote_qty, 48.000012, ulps = 2));
        assert!(trades[0].is_buyer_maker);

        let historical = market.get_historical_trades("LTCBTC", 28457, 1).unwrap();
        assert_eq!(historical.len(), 1);

        m1.assert();
        m2.assert();
    }

    #[test]
    fn get_ui_klines() {
        let mut server = Server::new();
        let mock = server
            .mock("GET", "/api/v3/uiKlines")
            .match_query(Matcher::AllOf(vec![
                Matcher::UrlEncoded("symbol".into(), "LTCBTC".into()),
                Matcher::UrlEncoded("interval".into(), "5m".into()),
                Matcher::UrlEncoded("limit".into(), "10".into()),
                Matcher::UrlEncoded("timeZone".into(), "-1:00".into()),
            ]))
            .with_body_from_file("tests/mocks/market/get_klines.json")
            .create();

        let config = Config::default().set_rest_api_endpoint(server.url());
        let market: Market = Binance::new_with_config(None, None, &config);
        let KlineSummaries::AllKlineSummaries(klines) = market
            .get_ui_klines("LTCBTC", "5m", 10, None, None, Some("-1:00".to_string()))
            .unwrap();
        mock.assert();
        assert!(!klines.is_empty());
    }

    #[test]
    fn trading_day_and_rolling_window_tickers() {
        let mut server = Server::new();
        let m1 = server
            .mock("GET", "/api/v3/ticker/tradingDay")
            .match_query(Matcher::Regex("symbol=BTCUSDT&timeZone=8".into()))
            .with_body_from_file("tests/mocks/market/trading_day_ticker.json")
            .create();
        let m2 = server
            .mock("GET", "/api/v3/ticker")
            .match_query(Matcher::AllOf(vec![
                Matcher::UrlEncoded("symbols".into(), r#"["BTCUSDT","BNBBTC"]"#.into()),
                Matcher::UrlEncoded("windowSize".into(), "4h".into()),
                Matcher::UrlEncoded("type".into(), "MINI".into()),
            ]))
            .with_body_from_file("tests/mocks/market/rolling_window_tickers_mini.json")
            .create();

        let config = Config::default().set_rest_api_endpoint(server.url());
        let market: Market = Binance::new_with_config(None, None, &config);

        let day = market
            .get_trading_day_ticker("BTCUSDT", Some("8".to_string()), false)
            .unwrap();
        assert_eq!(day.price_change.as_deref(), Some("-83.13000000"));
        assert!(approx_eq!(f64, day.last_price, 26221.67, ulps = 2));
        assert_eq!(day.count, 697727);

        let tickers = market
            .get_rolling_window_tickers(&["BTCUSDT", "BNBBTC"], Some("4h".to_string()), true)
            .unwrap();
        assert_eq!(tickers.len(), 2);
        assert_eq!(tickers[1].symbol, "BNBBTC");
        assert!(tickers[1].price_change.is_none());
        assert!(approx_eq!(f64, tickers[1].volume, 166.67, ulps = 2));

        m1.assert();
        m2.assert();
    }

    #[test]
    fn prices_for_symbols_encodes_json_array() {
        let mut server = Server::new();
        let mock = server
            .mock("GET", "/api/v3/ticker/price")
            .match_query(Matcher::UrlEncoded(
                "symbols".into(),
                r#"["LTCBTC","ETHBTC"]"#.into(),
            ))
            .with_body(r#"[{"symbol":"LTCBTC","price":"4.00000200"},{"symbol":"ETHBTC","price":"0.07946600"}]"#)
            .create();

        let config = Config::default().set_rest_api_endpoint(server.url());
        let market: Market = Binance::new_with_config(None, None, &config);
        let prices = market
            .get_prices_for_symbols(&["LTCBTC", "ETHBTC"])
            .unwrap();
        mock.assert();
        assert_eq!(prices.len(), 2);
        assert_eq!(prices[1].symbol, "ETHBTC");
    }
}
