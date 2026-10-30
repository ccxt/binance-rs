use binance::api::*;
use binance::config::*;
use binance::account::*;
use binance::model::*;

#[cfg(test)]
mod tests {
    use super::*;
    use mockito::{Server, Matcher};
    use float_cmp::*;

    #[test]
    fn get_account() {
        let mut server = Server::new();
        let mock_get_account = server
            .mock("GET", "/api/v3/account")
            .with_header("content-type", "application/json;charset=UTF-8")
            .match_query(Matcher::Regex(
                "recvWindow=1234&timestamp=\\d+&signature=.*".into(),
            ))
            .with_body_from_file("tests/mocks/account/get_account.json")
            .create();

        let config = Config::default()
            .set_rest_api_endpoint(server.url())
            .set_recv_window(1234);
        let account: Account = Binance::new_with_config(None, None, &config);
        let _ = env_logger::try_init();
        let account = account.get_account().unwrap();

        mock_get_account.assert();

        assert!(approx_eq!(f32, account.maker_commission, 15.0, ulps = 2));
        assert!(approx_eq!(f32, account.taker_commission, 15.0, ulps = 2));
        assert!(approx_eq!(f32, account.buyer_commission, 0.0, ulps = 2));
        assert!(approx_eq!(f32, account.seller_commission, 0.0, ulps = 2));
        assert!(account.can_trade);
        assert!(account.can_withdraw);
        assert!(account.can_deposit);

        assert!(!account.balances.is_empty());

        let first_balance = &account.balances[0];
        assert_eq!(first_balance.asset, "BTC");
        assert_eq!(first_balance.free, "4723846.89208129");
        assert_eq!(first_balance.locked, "0.00000000");

        let second_balance = &account.balances[1];
        assert_eq!(second_balance.asset, "LTC");
        assert_eq!(second_balance.free, "4763368.68006011");
        assert_eq!(second_balance.locked, "0.00000000");
    }

    #[test]
    fn get_balance() {
        let mut server = Server::new();
        let mock_get_account = server
            .mock("GET", "/api/v3/account")
            .with_header("content-type", "application/json;charset=UTF-8")
            .match_query(Matcher::Regex(
                "recvWindow=1234&timestamp=\\d+&signature=.*".into(),
            ))
            .with_body_from_file("tests/mocks/account/get_account.json")
            .create();

        let config = Config::default()
            .set_rest_api_endpoint(server.url())
            .set_recv_window(1234);
        let account: Account = Binance::new_with_config(None, None, &config);
        let _ = env_logger::try_init();
        let balance = account.get_balance("BTC").unwrap();

        mock_get_account.assert();

        assert_eq!(balance.asset, "BTC");
        assert_eq!(balance.free, "4723846.89208129");
        assert_eq!(balance.locked, "0.00000000");
    }

    #[test]
    fn get_open_orders() {
        let mut server = Server::new();
        let mock_open_orders = server
            .mock("GET", "/api/v3/openOrders")
            .with_header("content-type", "application/json;charset=UTF-8")
            .match_query(Matcher::Regex(
                "recvWindow=1234&symbol=LTCBTC&timestamp=\\d+".into(),
            ))
            .with_body_from_file("tests/mocks/account/get_open_orders.json")
            .create();

        let config = Config::default()
            .set_rest_api_endpoint(server.url())
            .set_recv_window(1234);
        let account: Account = Binance::new_with_config(None, None, &config);
        let _ = env_logger::try_init();
        let open_orders = account.get_open_orders("LTCBTC").unwrap();

        mock_open_orders.assert();

        assert!(open_orders.len() == 1);
        let open_order = &open_orders[0];

        assert_eq!(open_order.symbol, "LTCBTC");
        assert_eq!(open_order.order_id, 1);
        assert_eq!(open_order.order_list_id, -1);
        assert_eq!(open_order.client_order_id, "myOrder1");
        assert!(approx_eq!(f64, open_order.price, 0.1, ulps = 2));
        assert_eq!(open_order.orig_qty, "1.0");
        assert_eq!(open_order.executed_qty, "0.0");
        assert_eq!(open_order.cummulative_quote_qty, "0.0");
        assert_eq!(open_order.status, "NEW");
        assert_eq!(open_order.time_in_force, "GTC"); //Migrate to TimeInForce enum
        assert_eq!(open_order.type_name, "LIMIT");
        assert_eq!(open_order.side, "BUY");
        assert!(approx_eq!(f64, open_order.stop_price, 0.0, ulps = 2));
        assert_eq!(open_order.iceberg_qty, "0.0");
        assert_eq!(open_order.time, 1499827319559);
        assert_eq!(open_order.update_time, 1499827319559);
        assert!(open_order.is_working);
        assert_eq!(open_order.orig_quote_order_qty, "0.000000");
    }

    #[test]
    fn get_all_open_orders() {
        let mut server = Server::new();
        let mock_open_orders = server
            .mock("GET", "/api/v3/openOrders")
            .with_header("content-type", "application/json;charset=UTF-8")
            .match_query(Matcher::Regex("recvWindow=1234&timestamp=\\d+".into()))
            .with_body_from_file("tests/mocks/account/get_open_orders.json")
            .create();

        let config = Config::default()
            .set_rest_api_endpoint(server.url())
            .set_recv_window(1234);
        let account: Account = Binance::new_with_config(None, None, &config);
        let _ = env_logger::try_init();
        let open_orders = account.get_all_open_orders().unwrap();

        mock_open_orders.assert();

        assert!(open_orders.len() == 1);
        let open_order = &open_orders[0];

        assert_eq!(open_order.symbol, "LTCBTC");
        assert_eq!(open_order.order_id, 1);
        assert_eq!(open_order.order_list_id, -1);
        assert_eq!(open_order.client_order_id, "myOrder1");
        assert!(approx_eq!(f64, open_order.price, 0.1, ulps = 2));
        assert_eq!(open_order.orig_qty, "1.0");
        assert_eq!(open_order.executed_qty, "0.0");
        assert_eq!(open_order.cummulative_quote_qty, "0.0");
        assert_eq!(open_order.status, "NEW");
        assert_eq!(open_order.time_in_force, "GTC"); //Migrate to TimeInForce enum
        assert_eq!(open_order.type_name, "LIMIT");
        assert_eq!(open_order.side, "BUY");
        assert!(approx_eq!(f64, open_order.stop_price, 0.0, ulps = 2));
        assert_eq!(open_order.iceberg_qty, "0.0");
        assert_eq!(open_order.time, 1499827319559);
        assert_eq!(open_order.update_time, 1499827319559);
        assert!(open_order.is_working);
        assert_eq!(open_order.orig_quote_order_qty, "0.000000");
    }

    #[test]
    fn cancel_all_open_orders() {
        let mut server = Server::new();
        let mock_cancel_all_open_orders = server
            .mock("DELETE", "/api/v3/openOrders")
            .with_header("content-type", "application/json;charset=UTF-8")
            .match_query(Matcher::Regex(
                "recvWindow=1234&symbol=BTCUSDT&timestamp=\\d+".into(),
            ))
            .with_body_from_file("tests/mocks/account/cancel_all_open_orders.json")
            .create();

        let config = Config::default()
            .set_rest_api_endpoint(server.url())
            .set_recv_window(1234);
        let account: Account = Binance::new_with_config(None, None, &config);
        let _ = env_logger::try_init();
        let cancel_all_open_orders = account.cancel_all_open_orders("BTCUSDT").unwrap();

        mock_cancel_all_open_orders.assert();

        assert!(cancel_all_open_orders.len() == 3);

        let first_order_cancelled: OrderCanceled = cancel_all_open_orders[0].clone();
        assert_eq!(first_order_cancelled.symbol, "BTCUSDT");
        assert_eq!(
            first_order_cancelled.orig_client_order_id.unwrap(),
            "E6APeyTJvkMvLMYMqu1KQ4"
        );
        assert_eq!(first_order_cancelled.order_id.unwrap(), 11);
        assert_eq!(
            first_order_cancelled.client_order_id.unwrap(),
            "pXLV6Hz6mprAcVYpVMTGgx"
        );

        let second_order_cancelled: OrderCanceled = cancel_all_open_orders[1].clone();
        assert_eq!(second_order_cancelled.symbol, "BTCUSDT");
        assert_eq!(
            second_order_cancelled.orig_client_order_id.unwrap(),
            "A3EF2HCwxgZPFMrfwbgrhv"
        );
        assert_eq!(second_order_cancelled.order_id.unwrap(), 13);
        assert_eq!(
            second_order_cancelled.client_order_id.unwrap(),
            "pXLV6Hz6mprAcVYpVMTGgx"
        );
    }

    #[test]
    fn order_status() {
        let mut server = Server::new();
        let mock_order_status = server
            .mock("GET", "/api/v3/order")
            .with_header("content-type", "application/json;charset=UTF-8")
            .match_query(Matcher::Regex(
                "orderId=1&recvWindow=1234&symbol=LTCBTC&timestamp=\\d+".into(),
            ))
            .with_body_from_file("tests/mocks/account/order_status.json")
            .create();

        let config = Config::default()
            .set_rest_api_endpoint(server.url())
            .set_recv_window(1234);
        let account: Account = Binance::new_with_config(None, None, &config);
        let _ = env_logger::try_init();
        let order_status: Order = account.order_status("LTCBTC", 1).unwrap();

        mock_order_status.assert();

        assert_eq!(order_status.symbol, "LTCBTC");
        assert_eq!(order_status.order_id, 1);
        assert_eq!(order_status.order_list_id, -1);
        assert_eq!(order_status.client_order_id, "myOrder1");
        assert!(approx_eq!(f64, order_status.price, 0.1, ulps = 2));
        assert_eq!(order_status.orig_qty, "1.0");
        assert_eq!(order_status.executed_qty, "0.0");
        assert_eq!(order_status.cummulative_quote_qty, "0.0");
        assert_eq!(order_status.status, "NEW");
        assert_eq!(order_status.time_in_force, "GTC"); //Migrate to TimeInForce enum
        assert_eq!(order_status.type_name, "LIMIT");
        assert_eq!(order_status.side, "BUY");
        assert!(approx_eq!(f64, order_status.stop_price, 0.0, ulps = 2));
        assert_eq!(order_status.iceberg_qty, "0.0");
        assert_eq!(order_status.time, 1499827319559);
        assert_eq!(order_status.update_time, 1499827319559);
        assert!(order_status.is_working);
        assert_eq!(order_status.orig_quote_order_qty, "0.000000");
    }

    #[test]
    fn test_order_status() {
        let mut server = Server::new();
        let mock_test_order_status = server
            .mock("GET", "/api/v3/order/test")
            .with_header("content-type", "application/json;charset=UTF-8")
            .match_query(Matcher::Regex(
                "orderId=1&recvWindow=1234&symbol=LTCBTC&timestamp=\\d+".into(),
            ))
            .with_body("{}")
            .create();

        let config = Config::default()
            .set_rest_api_endpoint(server.url())
            .set_recv_window(1234);
        let account: Account = Binance::new_with_config(None, None, &config);
        let _ = env_logger::try_init();
        account.test_order_status("LTCBTC", 1).unwrap();

        mock_test_order_status.assert();
    }

    #[test]
    fn limit_buy() {
        let mut server = Server::new();
        let mock_limit_buy = server.mock("POST", "/api/v3/order")
            .with_header("content-type", "application/json;charset=UTF-8")
            .match_query(Matcher::Regex("price=0.1&quantity=1&recvWindow=1234&side=BUY&symbol=LTCBTC&timeInForce=GTC&timestamp=\\d+&type=LIMIT".into()))
            .with_body_from_file("tests/mocks/account/limit_buy.json")
            .create();

        let config = Config::default()
            .set_rest_api_endpoint(server.url())
            .set_recv_window(1234);
        let account: Account = Binance::new_with_config(None, None, &config);
        let _ = env_logger::try_init();
        let transaction: Transaction = account.limit_buy("LTCBTC", 1, 0.1).unwrap();

        mock_limit_buy.assert();

        assert_eq!(transaction.symbol, "LTCBTC");
        assert_eq!(transaction.order_id, 1);
        assert_eq!(transaction.order_list_id.unwrap(), -1);
        assert_eq!(transaction.client_order_id, "6gCrw2kRUAF9CvJDGP16IP");
        assert_eq!(transaction.transact_time, 1507725176595);
        assert!(approx_eq!(f64, transaction.price, 0.1, ulps = 2));
        assert!(approx_eq!(f64, transaction.orig_qty, 1.0, ulps = 2));
        assert!(approx_eq!(f64, transaction.executed_qty, 1.0, ulps = 2));
        assert!(approx_eq!(
            f64,
            transaction.cummulative_quote_qty,
            0.0,
            ulps = 2
        ));
        assert_eq!(transaction.status, "NEW");
        assert_eq!(transaction.time_in_force, "GTC"); //Migrate to TimeInForce enum
        assert_eq!(transaction.type_name, "LIMIT");
        assert_eq!(transaction.side, "BUY");
    }

    #[test]
    fn test_limit_buy() {
        let mut server = Server::new();
        let mock_test_limit_buy = server.mock("POST", "/api/v3/order/test")
            .with_header("content-type", "application/json;charset=UTF-8")
            .match_query(Matcher::Regex("price=0.1&quantity=1&recvWindow=1234&side=BUY&symbol=LTCBTC&timeInForce=GTC&timestamp=\\d+&type=LIMIT".into()))
            .with_body("{}")
            .create();

        let config = Config::default()
            .set_rest_api_endpoint(server.url())
            .set_recv_window(1234);
        let account: Account = Binance::new_with_config(None, None, &config);
        let _ = env_logger::try_init();
        account.test_limit_buy("LTCBTC", 1, 0.1).unwrap();

        mock_test_limit_buy.assert();
    }

    #[test]
    fn limit_sell() {
        let mut server = Server::new();
        let mock_limit_sell = server.mock("POST", "/api/v3/order")
            .with_header("content-type", "application/json;charset=UTF-8")
            .match_query(Matcher::Regex("price=0.1&quantity=1&recvWindow=1234&side=SELL&symbol=LTCBTC&timeInForce=GTC&timestamp=\\d+&type=LIMIT".into()))
            .with_body_from_file("tests/mocks/account/limit_sell.json")
            .create();

        let config = Config::default()
            .set_rest_api_endpoint(server.url())
            .set_recv_window(1234);
        let account: Account = Binance::new_with_config(None, None, &config);
        let _ = env_logger::try_init();
        let transaction: Transaction = account.limit_sell("LTCBTC", 1, 0.1).unwrap();

        mock_limit_sell.assert();

        assert_eq!(transaction.symbol, "LTCBTC");
        assert_eq!(transaction.order_id, 1);
        assert_eq!(transaction.order_list_id.unwrap(), -1);
        assert_eq!(transaction.client_order_id, "6gCrw2kRUAF9CvJDGP16IP");
        assert_eq!(transaction.transact_time, 1507725176595);
        assert!(approx_eq!(f64, transaction.price, 0.1, ulps = 2));
        assert!(approx_eq!(f64, transaction.orig_qty, 1.0, ulps = 2));
        assert!(approx_eq!(f64, transaction.executed_qty, 1.0, ulps = 2));
        assert!(approx_eq!(
            f64,
            transaction.cummulative_quote_qty,
            0.0,
            ulps = 2
        ));
        assert_eq!(transaction.status, "NEW");
        assert_eq!(transaction.time_in_force, "GTC"); //Migrate to TimeInForce enum
        assert_eq!(transaction.type_name, "LIMIT");
        assert_eq!(transaction.side, "SELL");
    }

    #[test]
    fn test_limit_sell() {
        let mut server = Server::new();
        let mock_test_limit_sell = server.mock("POST", "/api/v3/order/test")
            .with_header("content-type", "application/json;charset=UTF-8")
            .match_query(Matcher::Regex("price=0.1&quantity=1&recvWindow=1234&side=SELL&symbol=LTCBTC&timeInForce=GTC&timestamp=\\d+&type=LIMIT".into()))
            .with_body("{}")
            .create();

        let config = Config::default()
            .set_rest_api_endpoint(server.url())
            .set_recv_window(1234);
        let account: Account = Binance::new_with_config(None, None, &config);
        let _ = env_logger::try_init();
        account.test_limit_sell("LTCBTC", 1, 0.1).unwrap();

        mock_test_limit_sell.assert();
    }

    #[test]
    fn market_buy() {
        let mut server = Server::new();
        let mock_market_buy = server
            .mock("POST", "/api/v3/order")
            .with_header("content-type", "application/json;charset=UTF-8")
            .match_query(Matcher::Regex(
                "quantity=1&recvWindow=1234&side=BUY&symbol=LTCBTC&timestamp=\\d+&type=MARKET"
                    .into(),
            ))
            .with_body_from_file("tests/mocks/account/market_buy.json")
            .create();

        let config = Config::default()
            .set_rest_api_endpoint(server.url())
            .set_recv_window(1234);
        let account: Account = Binance::new_with_config(None, None, &config);
        let _ = env_logger::try_init();
        let transaction: Transaction = account.market_buy("LTCBTC", 1).unwrap();

        mock_market_buy.assert();

        assert_eq!(transaction.symbol, "LTCBTC");
        assert_eq!(transaction.order_id, 1);
        assert_eq!(transaction.order_list_id.unwrap(), -1);
        assert_eq!(transaction.client_order_id, "6gCrw2kRUAF9CvJDGP16IP");
        assert_eq!(transaction.transact_time, 1507725176595);
        assert!(approx_eq!(f64, transaction.price, 0.1, ulps = 2));
        assert!(approx_eq!(f64, transaction.orig_qty, 1.0, ulps = 2));
        assert!(approx_eq!(f64, transaction.executed_qty, 1.0, ulps = 2));
        assert!(approx_eq!(
            f64,
            transaction.cummulative_quote_qty,
            0.0,
            ulps = 2
        ));
        assert_eq!(transaction.status, "NEW");
        assert_eq!(transaction.time_in_force, "GTC"); //Migrate to TimeInForce enum
        assert_eq!(transaction.type_name, "MARKET");
        assert_eq!(transaction.side, "BUY");
    }

    #[test]
    fn test_market_buy() {
        let mut server = Server::new();
        let mock_test_market_buy = server
            .mock("POST", "/api/v3/order/test")
            .with_header("content-type", "application/json;charset=UTF-8")
            .match_query(Matcher::Regex(
                "quantity=1&recvWindow=1234&side=BUY&symbol=LTCBTC&timestamp=\\d+&type=MARKET"
                    .into(),
            ))
            .with_body("{}")
            .create();

        let config = Config::default()
            .set_rest_api_endpoint(server.url())
            .set_recv_window(1234);
        let account: Account = Binance::new_with_config(None, None, &config);
        let _ = env_logger::try_init();
        account.test_market_buy("LTCBTC", 1).unwrap();

        mock_test_market_buy.assert();
    }

    #[test]
    fn market_buy_using_quote_quantity() {
        let mut server = Server::new();
        let mock_market_buy_using_quote_quantity = server.mock("POST", "/api/v3/order")
            .with_header("content-type", "application/json;charset=UTF-8")
            .match_query(Matcher::Regex("quoteOrderQty=0.002&recvWindow=1234&side=BUY&symbol=BNBBTC&timestamp=\\d+&type=MARKET&signature=.*".into()))
            .with_body_from_file("tests/mocks/account/market_buy_using_quote_quantity.json")
            .create();

        let config = Config::default()
            .set_rest_api_endpoint(server.url())
            .set_recv_window(1234);
        let account: Account = Binance::new_with_config(None, None, &config);
        let _ = env_logger::try_init();
        match account.market_buy_using_quote_quantity("BNBBTC", 0.002) {
            Ok(answer) => {
                assert!(answer.order_id == 1);
            }
            Err(e) => panic!("Error: {}", e),
        }

        mock_market_buy_using_quote_quantity.assert();
    }

    #[test]
    fn test_market_buy_using_quote_quantity() {
        let mut server = Server::new();
        let mock_test_market_buy_using_quote_quantity = server.mock("POST", "/api/v3/order/test")
            .with_header("content-type", "application/json;charset=UTF-8")
            .match_query(Matcher::Regex("quoteOrderQty=0.002&recvWindow=1234&side=BUY&symbol=BNBBTC&timestamp=\\d+&type=MARKET&signature=.*".into()))
            .with_body("{}")
            .create();

        let config = Config::default()
            .set_rest_api_endpoint(server.url())
            .set_recv_window(1234);
        let account: Account = Binance::new_with_config(None, None, &config);
        let _ = env_logger::try_init();
        account
            .test_market_buy_using_quote_quantity("BNBBTC", 0.002)
            .unwrap();

        mock_test_market_buy_using_quote_quantity.assert();
    }

    #[test]
    fn market_sell() {
        let mut server = Server::new();
        let mock_market_sell = server
            .mock("POST", "/api/v3/order")
            .with_header("content-type", "application/json;charset=UTF-8")
            .match_query(Matcher::Regex(
                "quantity=1&recvWindow=1234&side=SELL&symbol=LTCBTC&timestamp=\\d+&type=MARKET"
                    .into(),
            ))
            .with_body_from_file("tests/mocks/account/market_sell.json")
            .create();

        let config = Config::default()
            .set_rest_api_endpoint(server.url())
            .set_recv_window(1234);
        let account: Account = Binance::new_with_config(None, None, &config);
        let _ = env_logger::try_init();
        let transaction: Transaction = account.market_sell("LTCBTC", 1).unwrap();

        mock_market_sell.assert();

        assert_eq!(transaction.symbol, "LTCBTC");
        assert_eq!(transaction.order_id, 1);
        assert_eq!(transaction.order_list_id.unwrap(), -1);
        assert_eq!(transaction.client_order_id, "6gCrw2kRUAF9CvJDGP16IP");
        assert_eq!(transaction.transact_time, 1507725176595);
        assert!(approx_eq!(f64, transaction.price, 0.1, ulps = 2));
        assert!(approx_eq!(f64, transaction.orig_qty, 1.0, ulps = 2));
        assert!(approx_eq!(f64, transaction.executed_qty, 1.0, ulps = 2));
        assert!(approx_eq!(
            f64,
            transaction.cummulative_quote_qty,
            0.0,
            ulps = 2
        ));
        assert_eq!(transaction.status, "NEW");
        assert_eq!(transaction.time_in_force, "GTC"); //Migrate to TimeInForce enum
        assert_eq!(transaction.type_name, "MARKET");
        assert_eq!(transaction.side, "SELL");
    }

    #[test]
    fn test_market_sell() {
        let mut server = Server::new();
        let mock_test_market_sell = server
            .mock("POST", "/api/v3/order/test")
            .with_header("content-type", "application/json;charset=UTF-8")
            .match_query(Matcher::Regex(
                "quantity=1&recvWindow=1234&side=SELL&symbol=LTCBTC&timestamp=\\d+&type=MARKET"
                    .into(),
            ))
            .with_body("{}")
            .create();

        let config = Config::default()
            .set_rest_api_endpoint(server.url())
            .set_recv_window(1234);
        let account: Account = Binance::new_with_config(None, None, &config);
        let _ = env_logger::try_init();
        account.test_market_sell("LTCBTC", 1).unwrap();

        mock_test_market_sell.assert();
    }

    #[test]
    fn market_sell_using_quote_quantity() {
        let mut server = Server::new();
        let mock_market_sell_using_quote_quantity = server.mock("POST", "/api/v3/order")
            .with_header("content-type", "application/json;charset=UTF-8")
            .match_query(Matcher::Regex("quoteOrderQty=0.002&recvWindow=1234&side=SELL&symbol=BNBBTC&timestamp=\\d+&type=MARKET&signature=.*".into()))
            .with_body_from_file("tests/mocks/account/market_sell_using_quote_quantity.json")
            .create();

        let config = Config::default()
            .set_rest_api_endpoint(server.url())
            .set_recv_window(1234);
        let account: Account = Binance::new_with_config(None, None, &config);
        let _ = env_logger::try_init();
        match account.market_sell_using_quote_quantity("BNBBTC", 0.002) {
            Ok(answer) => {
                assert!(answer.order_id == 1);
            }
            Err(e) => panic!("Error: {}", e),
        }

        mock_market_sell_using_quote_quantity.assert();
    }

    #[test]
    fn test_market_sell_using_quote_quantity() {
        let mut server = Server::new();
        let mock_test_market_sell_using_quote_quantity = server.mock("POST", "/api/v3/order/test")
            .with_header("content-type", "application/json;charset=UTF-8")
            .match_query(Matcher::Regex("quoteOrderQty=0.002&recvWindow=1234&side=SELL&symbol=BNBBTC&timestamp=\\d+&type=MARKET&signature=.*".into()))
            .with_body("{}")
            .create();

        let config = Config::default()
            .set_rest_api_endpoint(server.url())
            .set_recv_window(1234);
        let account: Account = Binance::new_with_config(None, None, &config);
        let _ = env_logger::try_init();
        account
            .test_market_sell_using_quote_quantity("BNBBTC", 0.002)
            .unwrap();

        mock_test_market_sell_using_quote_quantity.assert();
    }

    #[test]
    fn stop_limit_buy_order() {
        let mut server = Server::new();
        let mock_stop_limit_buy_order = server
            .mock("POST", "/api/v3/order")
            .with_header("content-type", "application/json;charset=UTF-8")
            // .match_query(Matcher::Regex("price=0.1&quantity=1&recvWindow=1234&side=BUY&stopPrice=0.09&symbol=LTCBTC&timeInForce=GTC&timestamp=\\d+&type=STOP_LOSS_LIMIT".into()))
            .match_query(Matcher::AllOf(vec![
                Matcher::Regex(r"price=0.1".into()),
                Matcher::UrlEncoded("quantity".into(), "1".into()),
                Matcher::UrlEncoded("recvWindow".into(), "1234".into()),
                Matcher::UrlEncoded("side".into(), "BUY".into()),
                Matcher::Regex(r"stopPrice=0.09".into()),
                Matcher::UrlEncoded("symbol".into(), "LTCBTC".into()),
                Matcher::UrlEncoded("timeInForce".into(), "GTC".into()),
                Matcher::Regex(r"timestamp=\d+".into()),
                Matcher::UrlEncoded("type".into(), "STOP_LOSS_LIMIT".into()),
                Matcher::Regex(r"newClientOrderId=x-HNA2TXFJ[A-Za-z0-9]+".into()),
            ]))
            .with_body_from_file("tests/mocks/account/stop_limit_buy.json")
            .create();

        let config = Config::default()
            .set_rest_api_endpoint(server.url())
            .set_recv_window(1234);
        let account: Account = Binance::new_with_config(None, None, &config);
        let _ = env_logger::try_init();
        let transaction: Transaction = account
            .stop_limit_buy_order("LTCBTC", 1, 0.1, 0.09, TimeInForce::GTC)
            .unwrap();

        mock_stop_limit_buy_order.assert();

        assert_eq!(transaction.symbol, "LTCBTC");
        assert_eq!(transaction.order_id, 1);
        assert_eq!(transaction.order_list_id.unwrap(), -1);
        assert_eq!(transaction.client_order_id, "6gCrw2kRUAF9CvJDGP16IP");
        assert_eq!(transaction.transact_time, 1507725176595);
        assert!(approx_eq!(f64, transaction.price, 0.1, ulps = 2));
        assert!(approx_eq!(f64, transaction.orig_qty, 1.0, ulps = 2));
        assert!(approx_eq!(f64, transaction.executed_qty, 1.0, ulps = 2));
        assert!(approx_eq!(
            f64,
            transaction.cummulative_quote_qty,
            0.0,
            ulps = 2
        ));
        assert!(approx_eq!(f64, transaction.stop_price, 0.09, ulps = 2));
        assert_eq!(transaction.status, "NEW");
        assert_eq!(transaction.time_in_force, "GTC"); //Migrate to TimeInForce enum
        assert_eq!(transaction.type_name, "STOP_LOSS_LIMIT");
        assert_eq!(transaction.side, "BUY");
    }

    #[test]
    fn test_stop_limit_buy_order() {
        let mut server = Server::new();
        let mock_test_stop_limit_buy_order = server.mock("POST", "/api/v3/order/test")
            .with_header("content-type", "application/json;charset=UTF-8")
            .match_query(Matcher::Regex("price=0.1&quantity=1&recvWindow=1234&side=BUY&stopPrice=0.09&symbol=LTCBTC&timeInForce=GTC&timestamp=\\d+&type=STOP_LOSS_LIMIT".into()))
            .with_body("{}")
            .create();

        let config = Config::default()
            .set_rest_api_endpoint(server.url())
            .set_recv_window(1234);
        let account: Account = Binance::new_with_config(None, None, &config);
        let _ = env_logger::try_init();
        account
            .test_stop_limit_buy_order("LTCBTC", 1, 0.1, 0.09, TimeInForce::GTC)
            .unwrap();

        mock_test_stop_limit_buy_order.assert();
    }

    #[test]
    fn stop_limit_sell_order() {
        let mut server = Server::new();
        let mock_stop_limit_sell_order = server.mock("POST", "/api/v3/order")
            .with_header("content-type", "application/json;charset=UTF-8")
            .match_query(Matcher::Regex("price=0.1&quantity=1&recvWindow=1234&side=SELL&stopPrice=0.09&symbol=LTCBTC&timeInForce=GTC&timestamp=\\d+&type=STOP_LOSS_LIMIT".into()))
            .with_body_from_file("tests/mocks/account/stop_limit_sell.json")
            .create();

        let config = Config::default()
            .set_rest_api_endpoint(server.url())
            .set_recv_window(1234);
        let account: Account = Binance::new_with_config(None, None, &config);
        let _ = env_logger::try_init();
        let transaction: Transaction = account
            .stop_limit_sell_order("LTCBTC", 1, 0.1, 0.09, TimeInForce::GTC)
            .unwrap();

        mock_stop_limit_sell_order.assert();

        assert_eq!(transaction.symbol, "LTCBTC");
        assert_eq!(transaction.order_id, 1);
        assert_eq!(transaction.order_list_id.unwrap(), -1);
        assert_eq!(transaction.client_order_id, "6gCrw2kRUAF9CvJDGP16IP");
        assert_eq!(transaction.transact_time, 1507725176595);
        assert!(approx_eq!(f64, transaction.price, 0.1, ulps = 2));
        assert!(approx_eq!(f64, transaction.orig_qty, 1.0, ulps = 2));
        assert!(approx_eq!(f64, transaction.executed_qty, 1.0, ulps = 2));
        assert!(approx_eq!(
            f64,
            transaction.cummulative_quote_qty,
            0.0,
            ulps = 2
        ));
        assert!(approx_eq!(f64, transaction.stop_price, 0.09, ulps = 2));
        assert_eq!(transaction.status, "NEW");
        assert_eq!(transaction.time_in_force, "GTC"); //Migrate to TimeInForce enum
        assert_eq!(transaction.type_name, "STOP_LOSS_LIMIT");
        assert_eq!(transaction.side, "SELL");
    }

    #[test]
    fn test_stop_limit_sell_order() {
        let mut server = Server::new();
        let mock_test_stop_limit_sell_order = server.mock("POST", "/api/v3/order/test")
            .with_header("content-type", "application/json;charset=UTF-8")
            .match_query(Matcher::Regex("price=0.1&quantity=1&recvWindow=1234&side=SELL&stopPrice=0.09&symbol=LTCBTC&timeInForce=GTC&timestamp=\\d+&type=STOP_LOSS_LIMIT".into()))
            .with_body("{}")
            .create();

        let config = Config::default()
            .set_rest_api_endpoint(server.url())
            .set_recv_window(1234);
        let account: Account = Binance::new_with_config(None, None, &config);
        let _ = env_logger::try_init();
        account
            .test_stop_limit_sell_order("LTCBTC", 1, 0.1, 0.09, TimeInForce::GTC)
            .unwrap();

        mock_test_stop_limit_sell_order.assert();
    }

    #[test]
    fn custom_order() {
        let mut server = Server::new();
        let mock_custom_order = server.mock("POST", "/api/v3/order")
            .with_header("content-type", "application/json;charset=UTF-8")
            .match_query(Matcher::Regex("newClientOrderId=6gCrw2kRUAF9CvJDGP16IP&price=0.1&quantity=1&recvWindow=1234&side=BUY&symbol=LTCBTC&timeInForce=GTC&timestamp=\\d+&type=MARKET".into()))
            .with_body_from_file("tests/mocks/account/stop_limit_sell.json")
            .create();

        let config = Config::default()
            .set_rest_api_endpoint(server.url())
            .set_recv_window(1234);
        let account: Account = Binance::new_with_config(None, None, &config);
        let _ = env_logger::try_init();
        let transaction: Transaction = account
            .custom_order(
                "LTCBTC",
                1,
                0.1,
                None,
                OrderSide::Buy,
                OrderType::Market,
                TimeInForce::GTC,
                Some("6gCrw2kRUAF9CvJDGP16IP".into()),
            )
            .unwrap();

        mock_custom_order.assert();

        assert_eq!(transaction.symbol, "LTCBTC");
        assert_eq!(transaction.order_id, 1);
        assert_eq!(transaction.order_list_id.unwrap(), -1);
        assert_eq!(transaction.client_order_id, "6gCrw2kRUAF9CvJDGP16IP");
        assert_eq!(transaction.transact_time, 1507725176595);
        assert!(approx_eq!(f64, transaction.price, 0.1, ulps = 2));
        assert!(approx_eq!(f64, transaction.orig_qty, 1.0, ulps = 2));
        assert!(approx_eq!(f64, transaction.executed_qty, 1.0, ulps = 2));
        assert!(approx_eq!(
            f64,
            transaction.cummulative_quote_qty,
            0.0,
            ulps = 2
        ));
        assert!(approx_eq!(f64, transaction.stop_price, 0.09, ulps = 2));
        assert_eq!(transaction.status, "NEW");
        assert_eq!(transaction.time_in_force, "GTC"); //Migrate to TimeInForce enum
        assert_eq!(transaction.type_name, "STOP_LOSS_LIMIT");
        assert_eq!(transaction.side, "SELL");
    }

    #[test]
    fn test_custom_order() {
        let mut server = Server::new();
        let mock_test_custom_order = server.mock("POST", "/api/v3/order/test")
            .with_header("content-type", "application/json;charset=UTF-8")
            .match_query(Matcher::Regex("price=0.1&quantity=1&recvWindow=1234&side=BUY&symbol=LTCBTC&timeInForce=GTC&timestamp=\\d+&type=MARKET".into()))
            .with_body("{}")
            .create();

        let config = Config::default()
            .set_rest_api_endpoint(server.url())
            .set_recv_window(1234);
        let account: Account = Binance::new_with_config(None, None, &config);
        let _ = env_logger::try_init();
        account
            .test_custom_order(
                "LTCBTC",
                1,
                0.1,
                None,
                OrderSide::Buy,
                OrderType::Market,
                TimeInForce::GTC,
                None,
            )
            .unwrap();

        mock_test_custom_order.assert();
    }

    #[test]
    fn cancel_order() {
        let mut server = Server::new();
        let mock_cancel_order = server
            .mock("DELETE", "/api/v3/order")
            .with_header("content-type", "application/json;charset=UTF-8")
            .match_query(Matcher::Regex(
                "orderId=1&recvWindow=1234&symbol=BTCUSDT&timestamp=\\d+".into(),
            ))
            .with_body_from_file("tests/mocks/account/cancel_order.json")
            .create();

        let config = Config::default()
            .set_rest_api_endpoint(server.url())
            .set_recv_window(1234);
        let account: Account = Binance::new_with_config(None, None, &config);
        let _ = env_logger::try_init();
        let cancelled_order = account.cancel_order("BTCUSDT", 1).unwrap();

        mock_cancel_order.assert();

        assert_eq!(cancelled_order.symbol, "LTCBTC");
        assert_eq!(cancelled_order.orig_client_order_id.unwrap(), "myOrder1");
        assert_eq!(cancelled_order.order_id.unwrap(), 4);
        assert_eq!(cancelled_order.client_order_id.unwrap(), "cancelMyOrder1");
    }

    #[test]
    fn test_cancel_order() {
        let mut server = Server::new();
        let mock_test_cancel_order = server
            .mock("DELETE", "/api/v3/order/test")
            .with_header("content-type", "application/json;charset=UTF-8")
            .match_query(Matcher::Regex(
                "orderId=1&recvWindow=1234&symbol=BTCUSDT&timestamp=\\d+".into(),
            ))
            .with_body_from_file("tests/mocks/account/cancel_order.json")
            .create();

        let config = Config::default()
            .set_rest_api_endpoint(server.url())
            .set_recv_window(1234);
        let account: Account = Binance::new_with_config(None, None, &config);
        let _ = env_logger::try_init();
        account.test_cancel_order("BTCUSDT", 1).unwrap();

        mock_test_cancel_order.assert();
    }

    #[test]
    fn trade_history() {
        let mut server = Server::new();
        let mock_trade_history = server
            .mock("GET", "/api/v3/myTrades")
            .with_header("content-type", "application/json;charset=UTF-8")
            .match_query(Matcher::Regex(
                "recvWindow=1234&symbol=BTCUSDT&timestamp=\\d+".into(),
            ))
            .with_body_from_file("tests/mocks/account/trade_history.json")
            .create();

        let config = Config::default()
            .set_rest_api_endpoint(server.url())
            .set_recv_window(1234);
        let account: Account = Binance::new_with_config(None, None, &config);
        let _ = env_logger::try_init();
        let histories = account.trade_history("BTCUSDT").unwrap();

        mock_trade_history.assert();

        assert!(histories.len() == 1);

        let history: TradeHistory = histories[0].clone();

        assert_eq!(history.id, 28457);
        assert!(approx_eq!(f64, history.price, 4.00000100, ulps = 2));
        assert!(approx_eq!(f64, history.qty, 12.00000000, ulps = 2));
        assert_eq!(history.commission, "10.10000000");
        assert_eq!(history.commission_asset, "BNB");
        assert_eq!(history.time, 1499865549590);
        assert!(history.is_buyer);
        assert!(!history.is_maker);
        assert!(history.is_best_match);
    }

    #[test]
    fn get_account_with_params_parses_new_fields() {
        let mut server = Server::new();
        let mock = server
            .mock("GET", "/api/v3/account")
            .with_header("content-type", "application/json;charset=UTF-8")
            .match_query(Matcher::Regex(
                "omitZeroBalances=true&recvWindow=1234&timestamp=\\d+&signature=.*".into(),
            ))
            .with_body_from_file("tests/mocks/account/get_account_full.json")
            .create();

        let config = Config::default()
            .set_rest_api_endpoint(server.url())
            .set_recv_window(1234);
        let account: Account = Binance::new_with_config(None, None, &config);
        let info = account.get_account_with_params(true).unwrap();
        mock.assert();

        let rates = info.commission_rates.unwrap();
        assert!(approx_eq!(f64, rates.maker, 0.0015, ulps = 2));
        assert_eq!(info.brokered, Some(false));
        assert_eq!(info.account_type.as_deref(), Some("SPOT"));
        assert_eq!(info.permissions, vec!["SPOT"]);
        assert_eq!(info.uid, Some(354937868));
        assert_eq!(info.balances.len(), 1);
    }

    #[test]
    fn account_commission_filters_and_rate_limits() {
        let mut server = Server::new();
        let m1 = server
            .mock("GET", "/api/v3/account/commission")
            .match_query(Matcher::Regex(
                "recvWindow=1234&symbol=BTCUSDT&timestamp=\\d+".into(),
            ))
            .with_body_from_file("tests/mocks/account/account_commission.json")
            .create();
        let m2 = server
            .mock("GET", "/api/v3/myFilters")
            .match_query(Matcher::Regex(
                "recvWindow=1234&symbol=BTCUSDT&timestamp=\\d+".into(),
            ))
            .with_body_from_file("tests/mocks/account/my_filters.json")
            .create();
        let m3 = server
            .mock("GET", "/api/v3/rateLimit/order")
            .match_query(Matcher::Regex("recvWindow=1234&timestamp=\\d+".into()))
            .with_body_from_file("tests/mocks/account/rate_limit_order.json")
            .create();

        let config = Config::default()
            .set_rest_api_endpoint(server.url())
            .set_recv_window(1234);
        let account: Account = Binance::new_with_config(None, None, &config);

        let commission = account.get_account_commission("BTCUSDT").unwrap();
        assert_eq!(commission.symbol, "BTCUSDT");
        assert!(approx_eq!(
            f64,
            commission.standard_commission.taker,
            0.0000002,
            ulps = 2
        ));
        assert!(approx_eq!(
            f64,
            commission.special_commission.unwrap().maker,
            0.01,
            ulps = 2
        ));
        assert_eq!(commission.discount.discount_asset.as_deref(), Some("BNB"));
        assert!(approx_eq!(
            f64,
            commission.discount.discount,
            0.75,
            ulps = 2
        ));

        let filters = account.get_my_filters("BTCUSDT").unwrap();
        assert!(matches!(
            filters.exchange_filters[0],
            Filters::ExchangeMaxNumOrders {
                max_num_orders: Some(1000)
            }
        ));
        assert!(matches!(
            filters.symbol_filters[0],
            Filters::PriceFilter { .. }
        ));
        match &filters.asset_filters[0] {
            Filters::MaxAsset { limit, asset } => {
                assert_eq!(limit.as_deref(), Some("100.00000000"));
                assert_eq!(asset.as_deref(), Some("BNB"));
            }
            other => panic!("unexpected {:?}", other),
        }
        assert_eq!(filters.rate_limits[0].count, Some(321));

        let limits = account.get_order_rate_limits().unwrap();
        assert_eq!(limits.len(), 2);
        assert_eq!(limits[1].interval, "DAY");
        assert_eq!(limits[1].limit, 160000);

        m1.assert();
        m2.assert();
        m3.assert();
    }

    #[test]
    fn order_queries_with_new_fields() {
        let mut server = Server::new();
        let m1 = server
            .mock("GET", "/api/v3/order")
            .match_query(Matcher::Regex(
                "origClientOrderId=myOrder1&recvWindow=1234&symbol=LTCBTC&timestamp=\\d+".into(),
            ))
            .with_body_from_file("tests/mocks/account/order_full.json")
            .create();
        let m2 = server
            .mock("GET", "/api/v3/allOrders")
            .match_query(Matcher::Regex(
                "limit=10&orderId=1&recvWindow=1234&symbol=LTCBTC&timestamp=\\d+".into(),
            ))
            .with_body_from_file("tests/mocks/account/all_orders.json")
            .create();

        let config = Config::default()
            .set_rest_api_endpoint(server.url())
            .set_recv_window(1234);
        let account: Account = Binance::new_with_config(None, None, &config);

        let order = account
            .order_status_with_client_id("LTCBTC", "myOrder1".into())
            .unwrap();
        assert_eq!(order.status, "EXPIRED");
        assert_eq!(order.working_time, Some(1499827319559));
        assert_eq!(order.self_trade_prevention_mode.as_deref(), Some("NONE"));
        assert_eq!(order.prevented_quantity, Some(1.2));
        assert_eq!(order.used_sor, Some(true));
        assert_eq!(order.peg_price_type.as_deref(), Some("PRIMARY_PEG"));
        assert_eq!(order.pegged_price, Some(87523.8371));
        assert_eq!(
            order.expiry_reason.as_deref(),
            Some("INSUFFICIENT_LIQUIDITY")
        );
        // stopPrice / icebergQty are no longer always present
        assert!(approx_eq!(f64, order.stop_price, 0.0, ulps = 2));
        assert_eq!(order.iceberg_qty, "");

        let orders = account.get_all_orders("LTCBTC", 1, None, None, 10).unwrap();
        assert_eq!(orders.len(), 1);
        assert_eq!(orders[0].client_order_id, "myOrder1");

        m1.assert();
        m2.assert();
    }

    #[test]
    fn order_lists() {
        let mut server = Server::new();
        let m1 = server
            .mock("POST", "/api/v3/orderList/oco")
            .match_query(Matcher::AllOf(vec![
                Matcher::UrlEncoded("symbol".into(), "LTCBTC".into()),
                Matcher::UrlEncoded("side".into(), "SELL".into()),
                Matcher::UrlEncoded("quantity".into(), "5".into()),
                Matcher::UrlEncoded("aboveType".into(), "LIMIT_MAKER".into()),
                Matcher::UrlEncoded("abovePrice".into(), "3".into()),
                Matcher::UrlEncoded("belowType".into(), "STOP_LOSS_LIMIT".into()),
                Matcher::UrlEncoded("belowPrice".into(), "1".into()),
                Matcher::UrlEncoded("belowStopPrice".into(), "1".into()),
                Matcher::UrlEncoded("belowTimeInForce".into(), "GTC".into()),
                Matcher::UrlEncoded("belowIcebergQty".into(), "1".into()),
                Matcher::UrlEncoded("listClientOrderId".into(), "lH1YDkuQKWiXVXHPSKYEIp".into()),
                Matcher::UrlEncoded("selfTradePreventionMode".into(), "EXPIRE_TAKER".into()),
                Matcher::Regex(r"timestamp=\d+".into()),
            ]))
            .with_body_from_file("tests/mocks/account/order_list_oco.json")
            .create();
        let m2 = server
            .mock("GET", "/api/v3/orderList")
            .match_query(Matcher::Regex(
                "orderListId=27&recvWindow=1234&timestamp=\\d+".into(),
            ))
            .with_body_from_file("tests/mocks/account/order_list.json")
            .create();
        let m3 = server
            .mock("GET", "/api/v3/allOrderList")
            .match_query(Matcher::Regex(
                "limit=3&recvWindow=1234&timestamp=\\d+".into(),
            ))
            .with_body("[]")
            .create();
        let m4 = server
            .mock("GET", "/api/v3/openOrderList")
            .match_query(Matcher::Regex("recvWindow=1234&timestamp=\\d+".into()))
            .with_body("[]")
            .create();
        let m5 = server
            .mock("DELETE", "/api/v3/orderList")
            .match_query(Matcher::Regex(
                "orderListId=0&recvWindow=1234&symbol=LTCBTC&timestamp=\\d+".into(),
            ))
            .with_body_from_file("tests/mocks/account/cancel_order_list.json")
            .create();

        let config = Config::default()
            .set_rest_api_endpoint(server.url())
            .set_recv_window(1234);
        let account: Account = Binance::new_with_config(None, None, &config);

        let oco = account
            .new_oco_order_list(OcoOrderListRequest {
                symbol: "LTCBTC".into(),
                side: OrderSide::Sell,
                quantity: 5.0,
                above: OrderListLeg {
                    order_type: Some(OrderType::LimitMaker),
                    price: Some(3.0),
                    ..Default::default()
                },
                below: OrderListLeg {
                    order_type: Some(OrderType::StopLossLimit),
                    price: Some(1.0),
                    stop_price: Some(1.0),
                    time_in_force: Some(TimeInForce::GTC),
                    iceberg_qty: Some(1.0),
                    ..Default::default()
                },
                list_client_order_id: Some("lH1YDkuQKWiXVXHPSKYEIp".into()),
                new_order_resp_type: None,
                self_trade_prevention_mode: Some(SelfTradePreventionMode::ExpireTaker),
            })
            .unwrap();
        assert_eq!(oco.order_list_id, 1);
        assert_eq!(oco.contingency_type, "OCO");
        assert_eq!(oco.orders.len(), 2);
        let reports = oco.order_reports.unwrap();
        assert_eq!(reports[0].type_name.as_deref(), Some("STOP_LOSS_LIMIT"));
        assert_eq!(reports[0].stop_price, Some(1.0));
        assert_eq!(reports[1].type_name.as_deref(), Some("LIMIT_MAKER"));

        let list = account.get_order_list(27).unwrap();
        assert_eq!(list.list_client_order_id, "h2USkA5YQpaXHPIrkd96xE");
        assert!(list.order_reports.is_none());

        assert!(
            account
                .get_all_order_lists(None, None, None, 3)
                .unwrap()
                .is_empty()
        );
        assert!(account.get_open_order_lists().unwrap().is_empty());

        let canceled = account.cancel_order_list("LTCBTC", 0, None).unwrap();
        assert_eq!(canceled.list_status_type, "ALL_DONE");
        let reports = canceled.order_reports.unwrap();
        assert_eq!(reports[0].status.as_deref(), Some("CANCELED"));
        assert_eq!(
            reports[0].orig_client_order_id.as_deref(),
            Some("pO9ufTiFGg3nw2fOdgeOXa")
        );

        for m in [m1, m2, m3, m4, m5] {
            m.assert();
        }
    }

    #[test]
    fn cancel_replace_and_amend() {
        let mut server = Server::new();
        let m1 = server
            .mock("POST", "/api/v3/order/cancelReplace")
            .match_query(Matcher::AllOf(vec![
                Matcher::UrlEncoded("symbol".into(), "BTCUSDT".into()),
                Matcher::UrlEncoded("side".into(), "BUY".into()),
                Matcher::UrlEncoded("type".into(), "LIMIT".into()),
                Matcher::UrlEncoded("cancelReplaceMode".into(), "STOP_ON_FAILURE".into()),
                Matcher::UrlEncoded("cancelOrderId".into(), "9".into()),
                Matcher::UrlEncoded("timeInForce".into(), "GTC".into()),
                Matcher::UrlEncoded("quantity".into(), "0.04".into()),
                Matcher::UrlEncoded("price".into(), "0.02".into()),
                Matcher::UrlEncoded("newClientOrderId".into(), "wOceeeOzNORyLiQfw7jd8S".into()),
                Matcher::UrlEncoded("orderRateLimitExceededMode".into(), "CANCEL_ONLY".into()),
                Matcher::Regex(r"timestamp=\d+".into()),
            ]))
            .with_body_from_file("tests/mocks/account/cancel_replace.json")
            .create();
        let m2 = server
            .mock("PUT", "/api/v3/order/amend/keepPriority")
            .match_query(Matcher::Regex(
                "newQty=5&orderId=60&recvWindow=1234&symbol=BTCUSDT&timestamp=\\d+&signature=.*"
                    .into(),
            ))
            .with_body_from_file("tests/mocks/account/amend_keep_priority.json")
            .create();

        let config = Config::default()
            .set_rest_api_endpoint(server.url())
            .set_recv_window(1234);
        let account: Account = Binance::new_with_config(None, None, &config);

        let result = account
            .cancel_replace_order(CancelReplaceRequest {
                symbol: "BTCUSDT".into(),
                side: OrderSide::Buy,
                order_type: OrderType::Limit,
                cancel_replace_mode: CancelReplaceMode::StopOnFailure,
                cancel_order_id: Some(9),
                cancel_orig_client_order_id: None,
                cancel_new_client_order_id: None,
                time_in_force: Some(TimeInForce::GTC),
                quantity: Some(0.04),
                quote_order_qty: None,
                price: Some(0.02),
                new_client_order_id: Some("wOceeeOzNORyLiQfw7jd8S".into()),
                stop_price: None,
                trailing_delta: None,
                iceberg_qty: None,
                strategy_id: None,
                strategy_type: None,
                new_order_resp_type: None,
                self_trade_prevention_mode: None,
                cancel_restrictions: None,
                order_rate_limit_exceeded_mode: Some("CANCEL_ONLY".into()),
            })
            .unwrap();
        assert_eq!(result.cancel_result, "SUCCESS");
        assert_eq!(result.new_order_result, "SUCCESS");
        match result.cancel_response.unwrap() {
            CancelReplaceOutcome::Order(o) => assert_eq!(o.order_id, Some(9)),
            other => panic!("unexpected {:?}", other),
        }
        match result.new_order_response.unwrap() {
            CancelReplaceOutcome::Order(o) => {
                assert_eq!(o.order_id, Some(10));
                assert_eq!(o.status.as_deref(), Some("NEW"));
            }
            other => panic!("unexpected {:?}", other),
        }

        let amended = account
            .amend_order_keep_priority("BTCUSDT", 60, None, 5.0, None)
            .unwrap();
        assert_eq!(amended.execution_id, 75);
        assert_eq!(amended.amended_order.order_id, 60);
        assert!(approx_eq!(f64, amended.amended_order.qty, 5.0, ulps = 2));

        m1.assert();
        m2.assert();
    }

    #[test]
    fn cancel_replace_partial_failure_is_typed() {
        let json =
            std::fs::read_to_string("tests/mocks/account/cancel_replace_partial_failure.json")
                .unwrap();
        let result: CancelReplaceResponse = serde_json::from_str(&json).unwrap();
        assert_eq!(result.new_order_result, "FAILURE");
        match result.new_order_response.unwrap() {
            CancelReplaceOutcome::Error { code, msg } => {
                assert_eq!(code, -2010);
                assert!(msg.contains("immediately match"));
            }
            other => panic!("unexpected {:?}", other),
        }
    }

    #[test]
    fn trade_history_with_params() {
        let mut server = Server::new();
        let mock = server
            .mock("GET", "/api/v3/myTrades")
            .match_query(Matcher::Regex(
                "endTime=20&limit=5&orderId=3&recvWindow=1234&startTime=10&symbol=BTCUSDT&timestamp=\\d+".into(),
            ))
            .with_body_from_file("tests/mocks/account/trade_history.json")
            .create();

        let config = Config::default()
            .set_rest_api_endpoint(server.url())
            .set_recv_window(1234);
        let account: Account = Binance::new_with_config(None, None, &config);
        let trades = account
            .trade_history_with_params("BTCUSDT", 3, 10, 20, None, 5)
            .unwrap();
        mock.assert();
        assert_eq!(trades.len(), 1);
    }
}
