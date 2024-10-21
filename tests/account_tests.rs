use binance::api::*;
use binance::config::*;
use binance::account::*;
use binance::model::*;
use rust_decimal::prelude::*;

#[cfg(test)]
mod tests {
    use super::*;
    use mockito::{Server, Matcher};

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

        assert_eq!(account.maker_commission, Decimal::new(15, 0));
        assert_eq!(account.taker_commission, Decimal::new(15, 0));
        assert_eq!(account.buyer_commission, Decimal::ZERO);
        assert_eq!(account.seller_commission, Decimal::ZERO);
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
        assert_eq!(open_order.price, Decimal::new(1, 1));
        assert_eq!(open_order.orig_qty, "1.0");
        assert_eq!(open_order.executed_qty, "0.0");
        assert_eq!(open_order.cummulative_quote_qty, "0.0");
        assert_eq!(open_order.status, "NEW");
        assert_eq!(open_order.time_in_force, "GTC"); //Migrate to TimeInForce enum
        assert_eq!(open_order.type_name, "LIMIT");
        assert_eq!(open_order.side, "BUY");
        assert_eq!(open_order.stop_price, Decimal::ZERO);
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
        assert_eq!(open_order.price, Decimal::new(1, 1));
        assert_eq!(open_order.orig_qty, "1.0");
        assert_eq!(open_order.executed_qty, "0.0");
        assert_eq!(open_order.cummulative_quote_qty, "0.0");
        assert_eq!(open_order.status, "NEW");
        assert_eq!(open_order.time_in_force, "GTC"); //Migrate to TimeInForce enum
        assert_eq!(open_order.type_name, "LIMIT");
        assert_eq!(open_order.side, "BUY");
        assert_eq!(open_order.stop_price, Decimal::ZERO);
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
        assert_eq!(order_status.price, Decimal::new(1, 1));
        assert_eq!(order_status.orig_qty, "1.0");
        assert_eq!(order_status.executed_qty, "0.0");
        assert_eq!(order_status.cummulative_quote_qty, "0.0");
        assert_eq!(order_status.status, "NEW");
        assert_eq!(order_status.time_in_force, "GTC"); //Migrate to TimeInForce enum
        assert_eq!(order_status.type_name, "LIMIT");
        assert_eq!(order_status.side, "BUY");
        assert_eq!(order_status.stop_price, Decimal::ZERO);
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
        let transaction: Transaction = account.limit_buy("LTCBTC", 1, Decimal::new(1, 1)).unwrap();

        mock_limit_buy.assert();

        assert_eq!(transaction.symbol, "LTCBTC");
        assert_eq!(transaction.order_id, 1);
        assert_eq!(transaction.order_list_id.unwrap(), -1);
        assert_eq!(transaction.client_order_id, "6gCrw2kRUAF9CvJDGP16IP");
        assert_eq!(transaction.transact_time, 1507725176595);
        assert_eq!(transaction.price, Decimal::new(1, 1));
        assert_eq!(transaction.orig_qty, Decimal::new(1, 0));
        assert_eq!(transaction.executed_qty, Decimal::new(1, 0));
        assert_eq!(transaction.cummulative_quote_qty, Decimal::ZERO);
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
        account.test_limit_buy("LTCBTC", 1, Decimal::new(1, 1)).unwrap();

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
        let transaction: Transaction = account.limit_sell("LTCBTC", 1, Decimal::new(1, 1)).unwrap();

        mock_limit_sell.assert();

        assert_eq!(transaction.symbol, "LTCBTC");
        assert_eq!(transaction.order_id, 1);
        assert_eq!(transaction.order_list_id.unwrap(), -1);
        assert_eq!(transaction.client_order_id, "6gCrw2kRUAF9CvJDGP16IP");
        assert_eq!(transaction.transact_time, 1507725176595);
        assert_eq!(transaction.price, Decimal::new(1, 1));
        assert_eq!(transaction.orig_qty, Decimal::new(1, 0));
        assert_eq!(transaction.executed_qty, Decimal::new(1, 0));
        assert_eq!(transaction.cummulative_quote_qty, Decimal::ZERO);
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
        account.test_limit_sell("LTCBTC", 1, Decimal::new(1, 1)).unwrap();

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
        assert_eq!(transaction.price, Decimal::new(1, 1));
        assert_eq!(transaction.orig_qty, Decimal::new(1, 0));
        assert_eq!(transaction.executed_qty, Decimal::new(1, 0));
        assert_eq!(transaction.cummulative_quote_qty, Decimal::ZERO);
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
        match account.market_buy_using_quote_quantity("BNBBTC", Decimal::new(2, 3)) {
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
        .test_market_buy_using_quote_quantity("BNBBTC", Decimal::new(2, 3))
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
        assert_eq!(transaction.price, Decimal::new(1, 1));
        assert_eq!(transaction.orig_qty, Decimal::new(1, 0));
        assert_eq!(transaction.executed_qty, Decimal::new(1, 0));
        assert_eq!(transaction.cummulative_quote_qty, Decimal::ZERO);
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
        match account.market_sell_using_quote_quantity("BNBBTC", Decimal::new(2, 3)) {
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
        .test_market_sell_using_quote_quantity("BNBBTC", Decimal::new(2, 3))
            .unwrap();

        mock_test_market_sell_using_quote_quantity.assert();
    }

    #[test]
    fn stop_limit_buy_order() {
        let mut server = Server::new();
        let mock_stop_limit_buy_order = server.mock("POST", "/api/v3/order")
            .with_header("content-type", "application/json;charset=UTF-8")
            .match_query(Matcher::Regex("price=0.1&quantity=1&recvWindow=1234&side=BUY&stopPrice=0.09&symbol=LTCBTC&timeInForce=GTC&timestamp=\\d+&type=STOP_LOSS_LIMIT".into()))
            .with_body_from_file("tests/mocks/account/stop_limit_buy.json")
            .create();

        let config = Config::default()
            .set_rest_api_endpoint(server.url())
            .set_recv_window(1234);
        let account: Account = Binance::new_with_config(None, None, &config);
        let _ = env_logger::try_init();
        let transaction: Transaction = account
        .stop_limit_buy_order("LTCBTC", 1, Decimal::new(1, 1), Decimal::new(9, 2), TimeInForce::GTC)
            .unwrap();

        mock_stop_limit_buy_order.assert();

        assert_eq!(transaction.symbol, "LTCBTC");
        assert_eq!(transaction.order_id, 1);
        assert_eq!(transaction.order_list_id.unwrap(), -1);
        assert_eq!(transaction.client_order_id, "6gCrw2kRUAF9CvJDGP16IP");
        assert_eq!(transaction.transact_time, 1507725176595);
        assert_eq!(transaction.price, Decimal::new(1, 1));
        assert_eq!(transaction.orig_qty, Decimal::new(1, 0));
        assert_eq!(transaction.executed_qty, Decimal::new(1, 0));
        assert_eq!(transaction.cummulative_quote_qty, Decimal::ZERO);
        assert_eq!(transaction.stop_price, Decimal::new(9, 2));
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
        .test_stop_limit_buy_order("LTCBTC", 1, Decimal::new(1, 1), Decimal::new(9, 2), TimeInForce::GTC)
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
            .stop_limit_sell_order("LTCBTC", 1, Decimal::new(1, 1), Decimal::new(9, 2), TimeInForce::GTC)
            .unwrap();

        mock_stop_limit_sell_order.assert();

        assert_eq!(transaction.symbol, "LTCBTC");
        assert_eq!(transaction.order_id, 1);
        assert_eq!(transaction.order_list_id.unwrap(), -1);
        assert_eq!(transaction.client_order_id, "6gCrw2kRUAF9CvJDGP16IP");
        assert_eq!(transaction.transact_time, 1507725176595);
        assert_eq!(transaction.price, Decimal::new(1, 1));
        assert_eq!(transaction.orig_qty, Decimal::new(1, 0));
        assert_eq!(transaction.executed_qty, Decimal::new(1, 0));
        assert_eq!(transaction.cummulative_quote_qty, Decimal::ZERO);
        assert_eq!(transaction.stop_price, Decimal::new(9, 2));
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
            .test_stop_limit_sell_order("LTCBTC", 1, Decimal::new(1, 1), Decimal::new(9, 2), TimeInForce::GTC)
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
                Decimal::new(1, 1),
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
        assert_eq!(transaction.price, Decimal::new(1, 1));
        assert_eq!(transaction.orig_qty, Decimal::new(1, 0));
        assert_eq!(transaction.executed_qty, Decimal::new(1, 0));
        assert_eq!(transaction.cummulative_quote_qty, Decimal::ZERO);
        assert_eq!(transaction.stop_price, Decimal::new(9, 2));
        assert_eq!(transaction.price, Decimal::new(1, 1));
        assert_eq!(transaction.orig_qty, Decimal::new(1, 0));
        assert_eq!(transaction.executed_qty, Decimal::new(1, 0));
        assert_eq!(transaction.cummulative_quote_qty, Decimal::ZERO);
        assert_eq!(transaction.stop_price, Decimal::new(9, 2));
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
                Decimal::new(1, 1),
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
        assert_eq!(history.price, Decimal::from_str("4.000001").unwrap());
        assert_eq!(history.qty, Decimal::new(12, 0));
        assert_eq!(history.commission, "10.10000000");
        assert_eq!(history.commission_asset, "BNB");
        assert_eq!(history.time, 1499865549590);
        assert!(history.is_buyer);
        assert!(!history.is_maker);
        assert!(history.is_best_match);
    }
}
