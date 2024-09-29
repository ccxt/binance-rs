use binance::api::*;
use binance::config::*;
use binance::futures::account::*;
use rust_decimal::prelude::*;

#[cfg(test)]
mod tests {
    use super::*;
    use mockito::{Server, Matcher};
    use binance::account::OrderSide;
    use binance::futures::model::Transaction;

    #[test]
    fn change_initial_leverage() {
        let mut server = Server::new();
        let mock_change_leverage = server
            .mock("POST", "/fapi/v1/leverage")
            .with_header("content-type", "application/json;charset=UTF-8")
            .match_query(Matcher::Regex(
                "leverage=2&recvWindow=1234&symbol=LTCUSDT&timestamp=\\d+&signature=.*".into(),
            ))
            .with_body_from_file("tests/mocks/futures/account/change_initial_leverage.json")
            .create();

        let config = Config::default()
            .set_futures_rest_api_endpoint(server.url())
            .set_recv_window(1234);
        let account: FuturesAccount = Binance::new_with_config(None, None, &config);
        let _ = env_logger::try_init();
        let response = account.change_initial_leverage("LTCUSDT", 2).unwrap();

        mock_change_leverage.assert();

        assert_eq!(response.leverage, 2);
        assert_eq!(response.symbol, "LTCUSDT");
        assert_eq!(response.max_notional_value, Decimal::from_str("9223372036854776000.0").unwrap());
    }

    #[test]
    fn change_margin_type() {
        let mut server = Server::new();
        let mock = server
            .mock("POST", "/fapi/v1/marginType")
            .with_header("content-type", "application/json;charset=UTF-8")
            .match_query(Matcher::Regex(
                "marginType=ISOLATED&recvWindow=1234&symbol=BTCUSDT&timestamp=\\d+&signature=.*"
                    .into(),
            ))
            .with_body_from_file("tests/mocks/futures/account/change_margin_type.json")
            .create();

        let config = Config::default()
            .set_futures_rest_api_endpoint(server.url())
            .set_recv_window(1234);
        let account: FuturesAccount = Binance::new_with_config(None, None, &config);
        let _ = env_logger::try_init();
        account.change_margin_type("BTCUSDT", true).unwrap();

        mock.assert();
    }

    #[test]
    fn change_position_margin() {
        let mut server = Server::new();
        let mock = server
            .mock("POST", "/fapi/v1/positionMargin")
            .with_header("content-type", "application/json;charset=UTF-8")
            .match_query(Matcher::Regex(
                "amount=100&recvWindow=1234&symbol=BTCUSDT&timestamp=\\d+&type=1&signature=.*"
                    .into(),
            ))
            .with_body_from_file("tests/mocks/futures/account/change_position_margin.json")
            .create();

        let config = Config::default()
            .set_futures_rest_api_endpoint(server.url())
            .set_recv_window(1234);
        let account: FuturesAccount = Binance::new_with_config(None, None, &config);
        let _ = env_logger::try_init();
        account
        .change_position_margin("BTCUSDT", Decimal::new(100, 0), true)
            .unwrap();

        mock.assert();
    }

    #[test]
    fn cancel_all_open_orders() {
        let mut server = Server::new();
        let mock = server
            .mock("DELETE", "/fapi/v1/allOpenOrders")
            .with_header("content-type", "application/json;charset=UTF-8")
            .match_query(Matcher::Regex(
                "recvWindow=1234&symbol=BTCUSDT&timestamp=\\d+&signature=.*".into(),
            ))
            .with_body_from_file("tests/mocks/futures/account/cancel_all_open_orders.json")
            .create();

        let config = Config::default()
            .set_futures_rest_api_endpoint(server.url())
            .set_recv_window(1234);
        let account: FuturesAccount = Binance::new_with_config(None, None, &config);
        let _ = env_logger::try_init();
        account.cancel_all_open_orders("BTCUSDT").unwrap();

        mock.assert();
    }

    #[test]
    fn change_position_mode() {
        let mut server = Server::new();
        let mock = server
            .mock("POST", "/fapi/v1/positionSide/dual")
            .with_header("content-type", "application/json;charset=UTF-8")
            .match_query(Matcher::Regex(
                "dualSidePosition=true&recvWindow=1234&timestamp=\\d+&signature=.*".into(),
            ))
            .with_body_from_file("tests/mocks/futures/account/change_position_mode.json")
            .create();

        let config = Config::default()
            .set_futures_rest_api_endpoint(server.url())
            .set_recv_window(1234);
        let account: FuturesAccount = Binance::new_with_config(None, None, &config);
        let _ = env_logger::try_init();
        account.change_position_mode(true).unwrap();

        mock.assert();
    }

    #[test]
    fn stop_market_close_buy() {
        let mut server = Server::new();
        let mock_stop_market_close_sell = server.mock("POST", "/fapi/v1/order")
            .with_header("content-type", "application/json;charset=UTF-8")
            .match_query(Matcher::Regex("closePosition=TRUE&recvWindow=1234&side=BUY&stopPrice=10.5&symbol=SRMUSDT&timestamp=\\d+&type=STOP_MARKET".into()))
            .with_body_from_file("tests/mocks/futures/account/stop_market_close_position_buy.json")
            .create();

        let config = Config::default()
            .set_futures_rest_api_endpoint(server.url())
            .set_recv_window(1234);
        let account: FuturesAccount = Binance::new_with_config(None, None, &config);
        let _ = env_logger::try_init();
        let transaction: Transaction = account.stop_market_close_buy("SRMUSDT", Decimal::new(105, 1)).unwrap();

        mock_stop_market_close_sell.assert();

        assert_eq!(transaction.symbol, "SRMUSDT");
        assert_eq!(transaction.side, "BUY");
        assert_eq!(transaction.orig_type, "STOP_MARKET");
        assert!(transaction.close_position);
        assert_eq!(transaction.stop_price, Decimal::new(105, 1));
    }

    #[test]
    fn stop_market_close_sell() {
        let mut server = Server::new();
        let mock_stop_market_close_sell = server.mock("POST", "/fapi/v1/order")
            .with_header("content-type", "application/json;charset=UTF-8")
            .match_query(Matcher::Regex("closePosition=TRUE&recvWindow=1234&side=SELL&stopPrice=7.4&symbol=SRMUSDT&timestamp=\\d+&type=STOP_MARKET".into()))
            .with_body_from_file("tests/mocks/futures/account/stop_market_close_position_sell.json")
            .create();

        let config = Config::default()
            .set_futures_rest_api_endpoint(server.url())
            .set_recv_window(1234);
        let account: FuturesAccount = Binance::new_with_config(None, None, &config);
        let _ = env_logger::try_init();
        let transaction: Transaction = account.stop_market_close_sell("SRMUSDT", Decimal::new(74, 1)).unwrap();

        mock_stop_market_close_sell.assert();

        assert_eq!(transaction.symbol, "SRMUSDT");
        assert_eq!(transaction.side, "SELL");
        assert_eq!(transaction.orig_type, "STOP_MARKET");
        assert!(transaction.close_position);
        assert_eq!(transaction.stop_price, Decimal::new(74, 1));
    }

    #[test]
    fn custom_order() {
        let mut server = Server::new();
        let mock_custom_order = server.mock("POST", "/fapi/v1/order")
            .with_header("content-type", "application/json;charset=UTF-8")
            .match_query(Matcher::Regex("closePosition=TRUE&recvWindow=1234&side=SELL&stopPrice=7.4&symbol=SRMUSDT&timestamp=\\d+&type=STOP_MARKET".into()))
            .with_body_from_file("tests/mocks/futures/account/stop_market_close_position_sell.json")
            .create();

        let config = Config::default()
            .set_futures_rest_api_endpoint(server.url())
            .set_recv_window(1234);
        let account: FuturesAccount = Binance::new_with_config(None, None, &config);
        let _ = env_logger::try_init();
        let custom_order = CustomOrderRequest {
            symbol: "SRMUSDT".into(),
            side: OrderSide::Sell,
            position_side: None,
            order_type: OrderType::StopMarket,
            time_in_force: None,
            qty: None,
            reduce_only: None,
            price: None,
            stop_price: Some(Decimal::new(74, 1)),
            close_position: Some(true),
            activation_price: None,
            callback_rate: None,
            working_type: None,
            price_protect: None,
        };
        let transaction: Transaction = account.custom_order(custom_order).unwrap();

        mock_custom_order.assert();

        assert_eq!(transaction.symbol, "SRMUSDT");
        assert_eq!(transaction.side, "SELL");
        assert_eq!(transaction.orig_type, "STOP_MARKET");
        assert!(transaction.close_position);
        assert_eq!(transaction.stop_price, Decimal::new(74, 1));
    }

    #[test]
    fn get_income() {
        let mut server = Server::new();
        let mock = server
            .mock("GET", "/fapi/v1/income")
            .with_header("content-type", "application/json;charset=UTF-8")
            .match_query(Matcher::Regex(
                "endTime=12345678910&incomeType=TRANSFER&limit=10\
                &recvWindow=1234&startTime=12345678910&symbol=BTCUSDT&timestamp=\\d+"
                    .into(),
            ))
            .with_body_from_file("tests/mocks/futures/account/get_income_history.json")
            .create();

        let config = Config::default()
            .set_futures_rest_api_endpoint(server.url())
            .set_recv_window(1234);
        let account: FuturesAccount = Binance::new_with_config(None, None, &config);
        let _ = env_logger::try_init();
        let income_request = IncomeRequest {
            symbol: Some("BTCUSDT".into()),
            income_type: Some(IncomeType::TRANSFER),
            start_time: Some(12345678910),
            end_time: Some(12345678910),
            limit: Some(10),
        };
        account.get_income(income_request).unwrap();

        mock.assert();
    }
}
