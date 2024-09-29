use binance::api::*;
use binance::config::*;
use binance::general::*;
use binance::model::*;
use rust_decimal::prelude::*;

#[cfg(test)]
mod tests {
    use super::*;
    use mockito::Server;

    #[test]
    fn ping() {
        let mut server = Server::new();
        let mock_ping = server
            .mock("GET", "/api/v3/ping")
            .with_header("content-type", "application/json;charset=UTF-8")
            .with_body("{}")
            .create();

        let config = Config::default().set_rest_api_endpoint(server.url());
        let general: General = Binance::new_with_config(None, None, &config);

        let pong = general.ping().unwrap();
        mock_ping.assert();

        assert_eq!(pong, "pong");
    }

    #[test]
    fn get_server_time() {
        let mut server = Server::new();
        let mock_server_time = server
            .mock("GET", "/api/v3/time")
            .with_header("content-type", "application/json;charset=UTF-8")
            .with_body_from_file("tests/mocks/general/server_time.json")
            .create();

        let config = Config::default().set_rest_api_endpoint(server.url());
        let general: General = Binance::new_with_config(None, None, &config);

        let server_time = general.get_server_time().unwrap();
        mock_server_time.assert();

        assert_eq!(server_time.server_time, 1499827319559);
    }

    #[test]
    fn exchange_info() {
        let mut server = Server::new();
        let mock_exchange_info = server
            .mock("GET", "/api/v3/exchangeInfo")
            .with_header("content-type", "application/json;charset=UTF-8")
            .with_body_from_file("tests/mocks/general/exchange_info.json")
            .create();

        let config = Config::default().set_rest_api_endpoint(server.url());
        let general: General = Binance::new_with_config(None, None, &config);

        let exchange_info = general.exchange_info().unwrap();
        mock_exchange_info.assert();

        assert!(exchange_info.symbols.len() > 1);
    }

    #[test]
    fn get_symbol_info() {
        let mut server = Server::new();
        let mock_exchange_info = server
            .mock("GET", "/api/v3/exchangeInfo")
            .with_header("content-type", "application/json;charset=UTF-8")
            .with_body_from_file("tests/mocks/general/exchange_info.json")
            .create();

        let config = Config::default().set_rest_api_endpoint(server.url());
        let general: General = Binance::new_with_config(None, None, &config);

        let symbol = general.get_symbol_info("BNBBTC").unwrap();
        mock_exchange_info.assert();

        assert_eq!(symbol.symbol, "BNBBTC");
        assert_eq!(symbol.status, "TRADING");
        assert_eq!(symbol.base_asset, "BNB");
        assert_eq!(symbol.base_asset_precision, 8);
        assert_eq!(symbol.quote_asset, "BTC");
        assert_eq!(symbol.quote_precision, 8);

        assert!(!symbol.order_types.is_empty());
        assert_eq!(symbol.order_types[0], "LIMIT");
        assert_eq!(symbol.order_types[1], "LIMIT_MAKER");
        assert_eq!(symbol.order_types[2], "MARKET");
        assert_eq!(symbol.order_types[3], "STOP_LOSS_LIMIT");
        assert_eq!(symbol.order_types[4], "TAKE_PROFIT_LIMIT");

        assert!(symbol.iceberg_allowed);
        assert!(symbol.is_spot_trading_allowed);
        assert!(symbol.is_margin_trading_allowed);

        assert!(!symbol.filters.is_empty());

        for filter in symbol.filters.into_iter() {
            match filter {
                Filters::PriceFilter {
                    min_price,
                    max_price,
                    tick_size,
                } => {
                    assert_eq!(min_price, "0.00000010");
                    assert_eq!(max_price, "100000.00000000");
                    assert_eq!(tick_size, "0.00000010");
                }
                Filters::PercentPrice {
                    multiplier_up,
                    multiplier_down,
                    avg_price_mins,
                } => {
                    assert_eq!(multiplier_up, "5");
                    assert_eq!(multiplier_down, "0.2");
                    assert_eq!(avg_price_mins.unwrap(), Decimal::new(5, 0));
                }
                Filters::LotSize {
                    min_qty,
                    max_qty,
                    step_size,
                } => {
                    assert_eq!(min_qty, "0.01000000");
                    assert_eq!(max_qty, "100000.00000000");
                    assert_eq!(step_size, "0.01000000");
                }
                Filters::MinNotional {
                    notional,
                    min_notional,
                    apply_to_market,
                    avg_price_mins,
                } => {
                    assert!(notional.is_none());
                    assert_eq!(min_notional.unwrap(), "0.00010000");
                    assert!(apply_to_market.unwrap());
                    assert_eq!(avg_price_mins.unwrap(), Decimal::new(5, 0));
                }
                Filters::IcebergParts { limit } => {
                    assert_eq!(limit.unwrap(), 10);
                }
                Filters::MarketLotSize {
                    min_qty,
                    max_qty,
                    step_size,
                } => {
                    assert_eq!(min_qty, "0.00000000");
                    assert_eq!(max_qty, "8528.32329395");
                    assert_eq!(step_size, "0.00000000");
                }
                Filters::MaxNumOrders { max_num_orders } => {
                    assert_eq!(max_num_orders.unwrap(), 200);
                }
                Filters::MaxNumAlgoOrders {
                    max_num_algo_orders,
                } => {
                    assert_eq!(max_num_algo_orders.unwrap(), 5);
                }
                _ => panic!(),
            }
        }
    }
}
