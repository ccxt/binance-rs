use binance::api::*;
use binance::config::*;
use binance::futures::account::*;

#[cfg(test)]
mod tests {
    use super::*;
    use mockito::{Server, Matcher};
    use float_cmp::*;
    use binance::account::OrderSide;
    use binance::futures::model::{AlgoOrder, BatchCancelResponse, BatchOrderResponse, Transaction};

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
        assert!(approx_eq!(
            f64,
            response.max_notional_value,
            9223372036854776000.0,
            ulps = 2
        ));
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
            .change_position_margin("BTCUSDT", 100., true)
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
        let mock = server
            .mock("POST", "/fapi/v1/algoOrder")
            .with_header("content-type", "application/json;charset=UTF-8")
            .match_query(Matcher::AllOf(vec![
                Matcher::UrlEncoded("algoType".into(), "CONDITIONAL".into()),
                Matcher::UrlEncoded("symbol".into(), "SRMUSDT".into()),
                Matcher::UrlEncoded("side".into(), "BUY".into()),
                Matcher::Regex(r"clientAlgoId=x-Cb7ytekJ[A-Za-z0-9]+".into()),
                Matcher::UrlEncoded("type".into(), "STOP_MARKET".into()),
                Matcher::Regex(r"timestamp=\d+".into()),
                Matcher::UrlEncoded("triggerPrice".into(), "10.5".into()),
                Matcher::UrlEncoded("closePosition".into(), "TRUE".into()),
                Matcher::UrlEncoded("recvWindow".into(), "1234".into()),
            ]))
            .with_body_from_file("tests/mocks/futures/account/new_algo_order.json")
            .create();

        let config = Config::default()
            .set_futures_rest_api_endpoint(server.url())
            .set_recv_window(1234);
        let account: FuturesAccount = Binance::new_with_config(None, None, &config);
        let _ = env_logger::try_init();
        let algo: AlgoOrder = account.stop_market_close_buy("SRMUSDT", 10.5).unwrap();

        mock.assert();

        assert_eq!(algo.algo_id, 2146760);
        assert_eq!(algo.symbol, "SRMUSDT");
        assert_eq!(algo.order_type, "STOP_MARKET");
        assert_eq!(algo.algo_status, "NEW");
        assert!(algo.close_position);
        assert!(approx_eq!(f64, algo.trigger_price, 7.4, ulps = 2));
        assert_eq!(algo.activate_price, None);
        assert_eq!(algo.iceberg_quantity, None);
    }

    #[test]
    fn stop_market_close_sell() {
        let mut server = Server::new();
        let mock = server
            .mock("POST", "/fapi/v1/algoOrder")
            .with_header("content-type", "application/json;charset=UTF-8")
            .match_query(Matcher::AllOf(vec![
                Matcher::UrlEncoded("algoType".into(), "CONDITIONAL".into()),
                Matcher::UrlEncoded("closePosition".into(), "TRUE".into()),
                Matcher::UrlEncoded("recvWindow".into(), "1234".into()),
                Matcher::UrlEncoded("side".into(), "SELL".into()),
                Matcher::UrlEncoded("triggerPrice".into(), "7.4".into()),
                Matcher::UrlEncoded("symbol".into(), "SRMUSDT".into()),
                Matcher::Regex(r"timestamp=\d+".into()),
                Matcher::UrlEncoded("type".into(), "STOP_MARKET".into()),
                Matcher::Regex(r"clientAlgoId=x-Cb7ytekJ[A-Za-z0-9]+".into()),
            ]))
            .with_body_from_file("tests/mocks/futures/account/new_algo_order.json")
            .create();

        let config = Config::default()
            .set_futures_rest_api_endpoint(server.url())
            .set_recv_window(1234);
        let account: FuturesAccount = Binance::new_with_config(None, None, &config);
        let _ = env_logger::try_init();
        let algo: AlgoOrder = account.stop_market_close_sell("SRMUSDT", 7.4).unwrap();

        mock.assert();

        assert_eq!(algo.side, "SELL");
        assert_eq!(algo.client_algo_id, "6B2I9XVcJpCjqPAJ4YoFX7");
        assert!(algo.reduce_only);
    }

    #[test]
    fn custom_algo_order() {
        let mut server = Server::new();
        let mock = server
            .mock("POST", "/fapi/v1/algoOrder")
            .with_header("content-type", "application/json;charset=UTF-8")
            .match_query(Matcher::AllOf(vec![
                Matcher::UrlEncoded("algoType".into(), "CONDITIONAL".into()),
                Matcher::UrlEncoded("clientAlgoId".into(), "myAlgoId".into()),
                Matcher::UrlEncoded("closePosition".into(), "TRUE".into()),
                Matcher::UrlEncoded("recvWindow".into(), "1234".into()),
                Matcher::UrlEncoded("side".into(), "SELL".into()),
                Matcher::UrlEncoded("triggerPrice".into(), "7.4".into()),
                Matcher::UrlEncoded("workingType".into(), "MARK_PRICE".into()),
                Matcher::UrlEncoded("priceProtect".into(), "TRUE".into()),
                Matcher::UrlEncoded("symbol".into(), "SRMUSDT".into()),
                Matcher::Regex(r"timestamp=\d+".into()),
                Matcher::UrlEncoded("type".into(), "STOP_MARKET".into()),
            ]))
            .with_body_from_file("tests/mocks/futures/account/new_algo_order.json")
            .create();

        let config = Config::default()
            .set_futures_rest_api_endpoint(server.url())
            .set_recv_window(1234);
        let account: FuturesAccount = Binance::new_with_config(None, None, &config);
        let _ = env_logger::try_init();
        let mut order = CustomOrderRequest::new("SRMUSDT", OrderSide::Sell, OrderType::StopMarket);
        order.stop_price = Some(7.4);
        order.close_position = Some(true);
        order.working_type = Some(WorkingType::MarkPrice);
        order.price_protect = Some(true);
        order.client_algo_id = Some("myAlgoId".into());
        let algo: AlgoOrder = account.custom_algo_order(order).unwrap();

        mock.assert();
        assert_eq!(algo.symbol, "SRMUSDT");
        assert_eq!(algo.algo_type, "CONDITIONAL");
    }

    #[test]
    fn custom_order_rejects_conditional_types_locally() {
        let config = Config::default().set_recv_window(1234);
        let account: FuturesAccount = Binance::new_with_config(None, None, &config);
        let mut order = CustomOrderRequest::new("BTCUSDT", OrderSide::Sell, OrderType::TakeProfit);
        order.stop_price = Some(7.4);
        let err = account.custom_order(order).unwrap_err();
        assert!(err.to_string().contains("custom_algo_order"), "{}", err);

        let limit = CustomOrderRequest::new("BTCUSDT", OrderSide::Sell, OrderType::Limit);
        let err = account.custom_algo_order(limit).unwrap_err();
        assert!(err.to_string().contains("custom_order"), "{}", err);
    }

    #[test]
    fn custom_order_limit() {
        let mut server = Server::new();
        let mock = server
            .mock("POST", "/fapi/v1/order")
            .with_header("content-type", "application/json;charset=UTF-8")
            .match_query(Matcher::AllOf(vec![
                Matcher::UrlEncoded("symbol".into(), "BTCUSDT".into()),
                Matcher::UrlEncoded("side".into(), "BUY".into()),
                Matcher::UrlEncoded("type".into(), "LIMIT".into()),
                Matcher::UrlEncoded("timeInForce".into(), "GTD".into()),
                Matcher::UrlEncoded("goodTillDate".into(), "1693207680000".into()),
                Matcher::UrlEncoded("quantity".into(), "10".into()),
                Matcher::UrlEncoded("price".into(), "30000".into()),
                Matcher::UrlEncoded("priceMatch".into(), "QUEUE_5".into()),
                Matcher::UrlEncoded("selfTradePreventionMode".into(), "EXPIRE_BOTH".into()),
                Matcher::UrlEncoded("newClientOrderId".into(), "testOrder".into()),
                Matcher::Regex(r"timestamp=\d+".into()),
            ]))
            .with_body_from_file("tests/mocks/futures/account/limit_order.json")
            .create();

        let config = Config::default()
            .set_futures_rest_api_endpoint(server.url())
            .set_recv_window(1234);
        let account: FuturesAccount = Binance::new_with_config(None, None, &config);
        let mut order = CustomOrderRequest::new("BTCUSDT", OrderSide::Buy, OrderType::Limit);
        order.time_in_force = Some(TimeInForce::GTD);
        order.good_till_date = Some(1693207680000);
        order.qty = Some(10.0);
        order.price = Some(30000.0);
        order.price_match = Some(PriceMatch::Queue5);
        order.self_trade_prevention_mode = Some(SelfTradePreventionMode::ExpireBoth);
        order.new_client_order_id = Some("testOrder".into());
        let transaction: Transaction = account.custom_order(order).unwrap();

        mock.assert();
        assert_eq!(transaction.order_id, 22542179);
        assert_eq!(transaction.price, Some(30000.0));
        assert_eq!(transaction.price_match.as_deref(), Some("NONE"));
        assert_eq!(
            transaction.self_trade_prevention_mode.as_deref(),
            Some("EXPIRE_MAKER")
        );
        assert_eq!(transaction.good_till_date, Some(0));
    }

    #[test]
    fn custom_batch_orders() {
        let mut server = Server::new();
        let mock = server
            .mock("POST", "/fapi/v1/batchOrders")
            .with_header("content-type", "application/json;charset=UTF-8")
            .match_query(Matcher::AllOf(vec![
                Matcher::UrlEncoded(
                    "batchOrders".into(),
                    r#"[{"newClientOrderId":"a","price":"30000","quantity":"1","side":"BUY","symbol":"BTCUSDT","timeInForce":"GTC","type":"LIMIT"},{"newClientOrderId":"b","quantity":"2","reduceOnly":"TRUE","side":"SELL","symbol":"ETHUSDT","type":"MARKET"}]"#.into(),
                ),
                Matcher::UrlEncoded("recvWindow".into(), "1234".into()),
                Matcher::Regex(r"timestamp=\d+".into()),
            ]))
            .with_body_from_file("tests/mocks/futures/account/batch_orders.json")
            .create();

        let config = Config::default()
            .set_futures_rest_api_endpoint(server.url())
            .set_recv_window(1234);
        let account: FuturesAccount = Binance::new_with_config(None, None, &config);
        let mut first = CustomOrderRequest::new("BTCUSDT", OrderSide::Buy, OrderType::Limit);
        first.time_in_force = Some(TimeInForce::GTC);
        first.qty = Some(1.0);
        first.price = Some(30000.0);
        first.new_client_order_id = Some("a".into());
        let mut second = CustomOrderRequest::new("ETHUSDT", OrderSide::Sell, OrderType::Market);
        second.qty = Some(2.0);
        second.reduce_only = Some(true);
        second.new_client_order_id = Some("b".into());
        let responses = account.custom_batch_orders(vec![first, second]).unwrap();

        mock.assert();
        assert_eq!(responses.len(), 2);
        match &responses[0] {
            BatchOrderResponse::Order(t) => assert_eq!(t.order_id, 22542179),
            other => panic!("unexpected {:?}", other),
        }
        match &responses[1] {
            BatchOrderResponse::Error { code, msg } => {
                assert_eq!(*code, -2022);
                assert_eq!(msg, "ReduceOnly Order is rejected.");
            }
            other => panic!("unexpected {:?}", other),
        }
    }

    #[test]
    fn custom_batch_orders_validation() {
        let config = Config::default();
        let account: FuturesAccount = Binance::new_with_config(None, None, &config);
        assert!(account.custom_batch_orders(vec![]).is_err());
        let mut stop = CustomOrderRequest::new("BTCUSDT", OrderSide::Sell, OrderType::StopMarket);
        stop.stop_price = Some(1.0);
        assert!(account.custom_batch_orders(vec![stop]).is_err());
    }

    #[test]
    fn cancel_batch_orders() {
        let mut server = Server::new();
        let mock = server
            .mock("DELETE", "/fapi/v1/batchOrders")
            .with_header("content-type", "application/json;charset=UTF-8")
            .match_query(Matcher::AllOf(vec![
                Matcher::UrlEncoded("symbol".into(), "BTCUSDT".into()),
                Matcher::UrlEncoded("orderIdList".into(), "[283194212,1]".into()),
                Matcher::Regex(r"timestamp=\d+".into()),
            ]))
            .with_body_from_file("tests/mocks/futures/account/cancel_batch_orders.json")
            .create();

        let config = Config::default()
            .set_futures_rest_api_endpoint(server.url())
            .set_recv_window(1234);
        let account: FuturesAccount = Binance::new_with_config(None, None, &config);
        let responses = account
            .cancel_batch_orders("BTCUSDT", &[283194212, 1])
            .unwrap();

        mock.assert();
        assert_eq!(responses.len(), 2);
        assert!(matches!(&responses[0], BatchCancelResponse::Order(o) if o.order_id == 283194212));
        assert!(matches!(
            &responses[1],
            BatchCancelResponse::Error { code: -2011, .. }
        ));
    }

    #[test]
    fn modify_order() {
        let mut server = Server::new();
        let mock = server
            .mock("PUT", "/fapi/v1/order")
            .with_header("content-type", "application/json;charset=UTF-8")
            .match_query(Matcher::AllOf(vec![
                Matcher::UrlEncoded("symbol".into(), "BTCUSDT".into()),
                Matcher::UrlEncoded("side".into(), "BUY".into()),
                Matcher::UrlEncoded("orderId".into(), "20072994037".into()),
                Matcher::UrlEncoded("quantity".into(), "1".into()),
                Matcher::UrlEncoded("price".into(), "30005".into()),
                Matcher::UrlEncoded("modifyId".into(), "1".into()),
                Matcher::UrlEncoded("reduceOnly".into(), "false".into()),
                Matcher::Regex(r"timestamp=\d+&signature=.*".into()),
            ]))
            .with_body_from_file("tests/mocks/futures/account/modify_order.json")
            .create();

        let config = Config::default()
            .set_futures_rest_api_endpoint(server.url())
            .set_recv_window(1234);
        let account: FuturesAccount = Binance::new_with_config(None, None, &config);
        let modified = account
            .modify_order(ModifyOrderRequest {
                symbol: "BTCUSDT".into(),
                side: OrderSide::Buy,
                order_id: Some(20072994037),
                orig_client_order_id: None,
                quantity: 1.0,
                price: Some(30005.0),
                price_match: None,
                modify_id: Some(1),
                reduce_only: Some(false),
            })
            .unwrap();

        mock.assert();
        assert_eq!(modified.order_id, 20072994037);
        assert_eq!(modified.modify_id, Some(1));
        assert!(approx_eq!(f64, modified.price, 30005.0, ulps = 2));
        assert_eq!(modified.position_side, "LONG");
    }

    #[test]
    fn order_status() {
        let mut server = Server::new();
        let mock = server
            .mock("GET", "/fapi/v1/order")
            .with_header("content-type", "application/json;charset=UTF-8")
            .match_query(Matcher::Regex(
                "orderId=1573346959&recvWindow=1234&symbol=BTCUSDT&timestamp=\\d+".into(),
            ))
            .with_body_from_file("tests/mocks/futures/account/query_order.json")
            .create();

        let config = Config::default()
            .set_futures_rest_api_endpoint(server.url())
            .set_recv_window(1234);
        let account: FuturesAccount = Binance::new_with_config(None, None, &config);
        let order = account.order_status("BTCUSDT", 1573346959).unwrap();

        mock.assert();
        assert_eq!(order.order_id, 1573346959);
        assert_eq!(order.time, Some(1579276756075));
        assert_eq!(order.price_match.as_deref(), Some("NONE"));
        assert_eq!(order.good_till_date, Some(0));
        assert!(approx_eq!(f64, order.activation_price, 9020.0, ulps = 2));
    }

    #[test]
    fn algo_order_queries() {
        let mut server = Server::new();
        let mock_get = server
            .mock("GET", "/fapi/v1/algoOrder")
            .with_header("content-type", "application/json;charset=UTF-8")
            .match_query(Matcher::Regex(
                "algoId=2146760&recvWindow=1234&timestamp=\\d+".into(),
            ))
            .with_body_from_file("tests/mocks/futures/account/query_algo_order.json")
            .create();
        let mock_open = server
            .mock("GET", "/fapi/v1/openAlgoOrders")
            .with_header("content-type", "application/json;charset=UTF-8")
            .match_query(Matcher::Regex(
                "algoType=CONDITIONAL&recvWindow=1234&symbol=BTCUSDT&timestamp=\\d+".into(),
            ))
            .with_body_from_file("tests/mocks/futures/account/open_algo_orders.json")
            .create();
        let mock_all = server
            .mock("GET", "/fapi/v1/allAlgoOrders")
            .with_header("content-type", "application/json;charset=UTF-8")
            .match_query(Matcher::Regex(
                "limit=5&recvWindow=1234&symbol=BTCUSDT&timestamp=\\d+".into(),
            ))
            .with_body_from_file("tests/mocks/futures/account/open_algo_orders.json")
            .create();
        let mock_cancel = server
            .mock("DELETE", "/fapi/v1/algoOrder")
            .with_header("content-type", "application/json;charset=UTF-8")
            .match_query(Matcher::Regex(
                "clientAlgoId=myAlgo1&recvWindow=1234&timestamp=\\d+".into(),
            ))
            .with_body_from_file("tests/mocks/futures/account/cancel_algo_order.json")
            .create();
        let mock_cancel_all = server
            .mock("DELETE", "/fapi/v1/algoOpenOrders")
            .with_header("content-type", "application/json;charset=UTF-8")
            .match_query(Matcher::Regex(
                "recvWindow=1234&symbol=BTCUSDT&timestamp=\\d+".into(),
            ))
            .with_body_from_file("tests/mocks/futures/account/cancel_all_algo_open_orders.json")
            .create();

        let config = Config::default()
            .set_futures_rest_api_endpoint(server.url())
            .set_recv_window(1234);
        let account: FuturesAccount = Binance::new_with_config(None, None, &config);

        let algo = account.get_algo_order(2146760).unwrap();
        assert_eq!(algo.algo_status, "CANCELED");
        assert_eq!(algo.actual_type.as_deref(), Some("LIMIT"));
        assert_eq!(algo.actual_qty, Some(0.01));
        assert_eq!(algo.actual_order_id.as_deref(), Some(""));

        let open = account
            .get_open_algo_orders(Some("BTCUSDT".to_string()))
            .unwrap();
        assert_eq!(open.len(), 1);
        assert_eq!(open[0].order_type, "TRAILING_STOP_MARKET");
        assert_eq!(open[0].working_type, "MARK_PRICE");
        assert_eq!(open[0].tp_trigger_price, None);

        let all = account
            .get_all_algo_orders("BTCUSDT", None, None, None, 5)
            .unwrap();
        assert_eq!(all.len(), 1);

        let canceled = account.cancel_algo_order_with_client_id("myAlgo1").unwrap();
        assert_eq!(canceled.algo_id, 2146760);
        assert_eq!(canceled.code, 200);

        let canceled_all = account.cancel_all_open_algo_orders("BTCUSDT").unwrap();
        assert_eq!(canceled_all.code, 200);

        mock_get.assert();
        mock_open.assert();
        mock_all.assert();
        mock_cancel.assert();
        mock_cancel_all.assert();
    }

    #[test]
    fn v3_account_endpoints() {
        let mut server = Server::new();
        let mock_position = server
            .mock("GET", "/fapi/v3/positionRisk")
            .with_header("content-type", "application/json;charset=UTF-8")
            .match_query(Matcher::Regex(
                "recvWindow=1234&symbol=ADAUSDT&timestamp=\\d+".into(),
            ))
            .with_body_from_file("tests/mocks/futures/account/position_risk_v3.json")
            .create();
        let mock_account = server
            .mock("GET", "/fapi/v3/account")
            .with_header("content-type", "application/json;charset=UTF-8")
            .match_query(Matcher::Regex("recvWindow=1234&timestamp=\\d+".into()))
            .with_body_from_file("tests/mocks/futures/account/account_information_v3.json")
            .create();
        let mock_balance = server
            .mock("GET", "/fapi/v3/balance")
            .with_header("content-type", "application/json;charset=UTF-8")
            .match_query(Matcher::Regex("recvWindow=1234&timestamp=\\d+".into()))
            .with_body_from_file("tests/mocks/futures/account/balance_v3.json")
            .create();

        let config = Config::default()
            .set_futures_rest_api_endpoint(server.url())
            .set_recv_window(1234);
        let account: FuturesAccount = Binance::new_with_config(None, None, &config);

        let positions = account
            .position_information_v3(Some("ADAUSDT".to_string()))
            .unwrap();
        assert_eq!(positions.len(), 1);
        assert_eq!(positions[0].symbol, "ADAUSDT");
        assert!(approx_eq!(
            f64,
            positions[0].break_even_price,
            0.385077,
            ulps = 2
        ));
        assert_eq!(positions[0].adl, 2);
        assert_eq!(positions[0].margin_asset, "USDT");

        let info = account.account_information_v3().unwrap();
        assert!(approx_eq!(
            f64,
            info.total_wallet_balance,
            126.72469206,
            ulps = 2
        ));
        assert_eq!(info.assets[0].asset, "USDT");
        assert_eq!(info.positions[0].symbol, "BTCUSDT");
        assert!(approx_eq!(
            f64,
            info.positions[0].position_amount,
            1.0,
            ulps = 2
        ));

        let balances = account.account_balance_v3().unwrap();
        assert_eq!(balances[0].asset, "USDT");
        assert!(balances[0].margin_available);

        mock_position.assert();
        mock_account.assert();
        mock_balance.assert();
    }

    #[test]
    fn config_endpoints() {
        let mut server = Server::new();
        let m1 = server
            .mock("GET", "/fapi/v1/symbolConfig")
            .match_query(Matcher::Regex(
                "recvWindow=1234&symbol=BTCUSDT&timestamp=\\d+".into(),
            ))
            .with_body_from_file("tests/mocks/futures/account/symbol_config.json")
            .create();
        let m2 = server
            .mock("GET", "/fapi/v1/accountConfig")
            .match_query(Matcher::Any)
            .with_body_from_file("tests/mocks/futures/account/account_config.json")
            .create();
        let m3 = server
            .mock("GET", "/fapi/v1/commissionRate")
            .match_query(Matcher::Regex(
                "recvWindow=1234&symbol=BTCUSDT&timestamp=\\d+".into(),
            ))
            .with_body_from_file("tests/mocks/futures/account/commission_rate.json")
            .create();
        let m4 = server
            .mock("GET", "/fapi/v1/leverageBracket")
            .match_query(Matcher::Regex(
                "recvWindow=1234&symbol=ETHUSDT&timestamp=\\d+".into(),
            ))
            .with_body_from_file("tests/mocks/futures/account/leverage_bracket.json")
            .create();
        let m5 = server
            .mock("GET", "/fapi/v1/positionSide/dual")
            .match_query(Matcher::Any)
            .with_body_from_file("tests/mocks/futures/account/position_mode.json")
            .create();
        let m6 = server
            .mock("GET", "/fapi/v1/multiAssetsMargin")
            .match_query(Matcher::Any)
            .with_body_from_file("tests/mocks/futures/account/multi_assets_margin.json")
            .create();
        let m7 = server
            .mock("POST", "/fapi/v1/multiAssetsMargin")
            .match_query(Matcher::Regex(
                "multiAssetsMargin=true&recvWindow=1234&timestamp=\\d+".into(),
            ))
            .with_body_from_file("tests/mocks/futures/account/code_msg.json")
            .create();
        let m8 = server
            .mock("GET", "/fapi/v1/feeBurn")
            .match_query(Matcher::Any)
            .with_body_from_file("tests/mocks/futures/account/fee_burn.json")
            .create();
        let m9 = server
            .mock("POST", "/fapi/v1/feeBurn")
            .match_query(Matcher::Regex(
                "feeBurn=false&recvWindow=1234&timestamp=\\d+".into(),
            ))
            .with_body_from_file("tests/mocks/futures/account/code_msg.json")
            .create();
        let m10 = server
            .mock("GET", "/fapi/v1/adlQuantile")
            .match_query(Matcher::Any)
            .with_body_from_file("tests/mocks/futures/account/adl_quantile.json")
            .create();
        let m11 = server
            .mock("GET", "/fapi/v1/apiTradingStatus")
            .match_query(Matcher::Any)
            .with_body_from_file("tests/mocks/futures/account/api_trading_status.json")
            .create();
        let m12 = server
            .mock("GET", "/fapi/v1/rateLimit/order")
            .match_query(Matcher::Any)
            .with_body_from_file("tests/mocks/futures/account/rate_limit_order.json")
            .create();

        let config = Config::default()
            .set_futures_rest_api_endpoint(server.url())
            .set_recv_window(1234);
        let account: FuturesAccount = Binance::new_with_config(None, None, &config);

        let symbol_config = account
            .get_symbol_config(Some("BTCUSDT".to_string()))
            .unwrap();
        assert_eq!(symbol_config[0].leverage, 21);
        assert_eq!(symbol_config[0].margin_type, "CROSSED");

        let account_config = account.get_account_config().unwrap();
        assert!(account_config.dual_side_position);
        assert_eq!(account_config.trade_group_id, -1);

        let commission = account.get_commission_rate("BTCUSDT").unwrap();
        assert!(approx_eq!(
            f64,
            commission.taker_commission_rate,
            0.0004,
            ulps = 2
        ));
        assert_eq!(commission.rpi_commission_rate, Some(0.0001));

        let brackets = account
            .get_leverage_brackets(Some("ETHUSDT".to_string()))
            .unwrap();
        assert_eq!(brackets[0].notional_coef, Some(1.5));
        assert_eq!(brackets[0].brackets.len(), 2);
        assert_eq!(brackets[0].brackets[1].initial_leverage, 50);

        assert!(account.get_position_mode().unwrap().dual_side_position);
        assert!(
            !account
                .get_multi_assets_margin()
                .unwrap()
                .multi_assets_margin
        );
        assert_eq!(account.change_multi_assets_margin(true).unwrap().code, 200);
        assert!(account.get_fee_burn().unwrap().fee_burn);
        assert_eq!(account.set_fee_burn(false).unwrap().msg, "success");

        let adl = account.get_adl_quantile(None).unwrap();
        assert_eq!(adl[0].adl_quantile.long, Some(3));
        assert_eq!(adl[0].adl_quantile.hedge, Some(0));
        assert_eq!(adl[1].adl_quantile.both, Some(0));

        let status = account.get_api_trading_status(None).unwrap();
        assert_eq!(status.indicators["BTCUSDT"][0].indicator, "UFR");
        assert!(status.indicators["BTCUSDT"][0].is_locked);

        let limits = account.get_order_rate_limits().unwrap();
        assert_eq!(limits[1].interval, "MINUTE");

        for m in [m1, m2, m3, m4, m5, m6, m7, m8, m9, m10, m11, m12] {
            m.assert();
        }
    }

    #[test]
    fn force_orders_countdown_and_amendments() {
        let mut server = Server::new();
        let m1 = server
            .mock("GET", "/fapi/v1/forceOrders")
            .match_query(Matcher::Regex(
                "autoCloseType=LIQUIDATION&limit=10&recvWindow=1234&symbol=BTCUSDT&timestamp=\\d+"
                    .into(),
            ))
            .with_body_from_file("tests/mocks/futures/account/force_orders.json")
            .create();
        let m2 = server
            .mock("POST", "/fapi/v1/countdownCancelAll")
            .match_query(Matcher::Regex(
                "countdownTime=100000&recvWindow=1234&symbol=BTCUSDT&timestamp=\\d+".into(),
            ))
            .with_body_from_file("tests/mocks/futures/account/countdown_cancel_all.json")
            .create();
        let m3 = server
            .mock("GET", "/fapi/v1/orderAmendment")
            .match_query(Matcher::Regex(
                "orderId=20072994037&recvWindow=1234&symbol=BTCUSDT&timestamp=\\d+".into(),
            ))
            .with_body_from_file("tests/mocks/futures/account/order_amendment.json")
            .create();
        let m4 = server
            .mock("GET", "/fapi/v1/positionMargin/history")
            .match_query(Matcher::Regex(
                "recvWindow=1234&symbol=BTCUSDT&timestamp=\\d+&type=1".into(),
            ))
            .with_body_from_file("tests/mocks/futures/account/position_margin_history.json")
            .create();

        let config = Config::default()
            .set_futures_rest_api_endpoint(server.url())
            .set_recv_window(1234);
        let account: FuturesAccount = Binance::new_with_config(None, None, &config);

        let force = account
            .get_force_orders(
                Some("BTCUSDT".to_string()),
                Some("LIQUIDATION".to_string()),
                None,
                None,
                10,
            )
            .unwrap();
        assert_eq!(force[0].client_order_id, "autoclose-1596107620040000020");
        assert!(approx_eq!(f64, force[0].avg_price, 10913.21, ulps = 2));

        let countdown = account.countdown_cancel_all("BTCUSDT", 100000).unwrap();
        assert_eq!(countdown.countdown_time, 100000);

        let amendments = account
            .get_order_amendments("BTCUSDT", 20072994037, None, None, None, None)
            .unwrap();
        assert_eq!(amendments[0].amendment.count, 3);
        assert_eq!(amendments[0].amendment.modify_id, Some(7));
        assert!(approx_eq!(
            f64,
            amendments[0].amendment.price.after,
            30003.2,
            ulps = 2
        ));

        let history = account
            .get_position_margin_history("BTCUSDT", 1, None, None, None)
            .unwrap();
        assert_eq!(history.len(), 2);
        assert_eq!(history[1].position_side, "LONG");

        for m in [m1, m2, m3, m4] {
            m.assert();
        }
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
