use binance::api::*;
use binance::config::*;
use binance::market::*;
use binance::model::*;
use rust_decimal::prelude::*;

#[cfg(test)]
mod tests {
    use super::*;
    use mockito::{Server, Matcher};

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
        assert_eq!(order_book.bids[0], Bids::new(Decimal::new(4, 0), Decimal::new(431, 0)));
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
        assert_eq!(order_book.bids[0], Bids::new(Decimal::new(4, 0), Decimal::new(431, 0)));
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
                assert_eq!(first_symbol.price, Decimal::from_str("4.000002").unwrap());
                let second_symbol = symbols[1].clone();
                assert_eq!(second_symbol.symbol, "ETHBTC");
                assert_eq!(second_symbol.price, Decimal::from_str("0.07946600").unwrap());
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
        assert_eq!(symbol.price, Decimal::from_str("4.000002").unwrap());
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
        assert_eq!(symbol.price, Decimal::from_str("9.35751834").unwrap());
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
                assert_eq!(first_ticker.bid_price, Decimal::new(4, 0));
                assert_eq!(first_ticker.bid_qty, Decimal::new(431, 0));
                assert_eq!(first_ticker.ask_price, Decimal::from_str("4.000002").unwrap());
                assert_eq!(first_ticker.ask_qty, Decimal::new(9, 0));
                let second_ticker = tickers[1].clone();
                assert_eq!(second_ticker.symbol, "ETHBTC");
                assert_eq!(second_ticker.bid_price, Decimal::from_str("0.07946700").unwrap());
                assert_eq!(second_ticker.bid_qty, Decimal::new(9, 0));
                assert_eq!(second_ticker.ask_price, Decimal::from_str("100000.00").unwrap());
                assert_eq!(second_ticker.ask_qty, Decimal::new(1000, 0));
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
        assert_eq!(book_ticker.bid_price, Decimal::new(4, 0));
        assert_eq!(book_ticker.bid_qty, Decimal::new(431, 0));
        assert_eq!(book_ticker.ask_price, Decimal::from_str("4.000002").unwrap());
        assert_eq!(book_ticker.ask_qty, Decimal::new(9, 0));
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
        assert_eq!(price_stats.prev_close_price, Decimal::from_str("0.10002").unwrap());
        assert_eq!(price_stats.last_price, Decimal::from_str("4.000002").unwrap());
        assert_eq!(price_stats.bid_price, Decimal::from_str("4.0").unwrap());
        assert_eq!(price_stats.ask_price, Decimal::from_str("4.000002").unwrap());
        assert_eq!(price_stats.open_price, Decimal::from_str("99.000").unwrap());
        assert_eq!(price_stats.high_price, Decimal::from_str("100.0000").unwrap());
        assert_eq!(price_stats.low_price, Decimal::from_str("0.1").unwrap());
        assert_eq!(price_stats.volume, Decimal::from_str("8913.30").unwrap());
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
        assert_eq!(price_stats.prev_close_price, Decimal::from_str("0.10002").unwrap());
        assert_eq!(price_stats.last_price, Decimal::from_str("4.00000200").unwrap());
        assert_eq!(price_stats.bid_price, Decimal::from_str("4.0").unwrap());
        assert_eq!(price_stats.ask_price, Decimal::from_str("4.000002").unwrap());
        assert_eq!(price_stats.open_price, Decimal::from_str("99.000").unwrap());
        assert_eq!(price_stats.high_price, Decimal::from_str("100.0000").unwrap());
        assert_eq!(price_stats.low_price, Decimal::from_str("0.1").unwrap());
        assert_eq!(price_stats.volume, Decimal::from_str("8913.30").unwrap());
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
}
