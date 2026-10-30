use binance::config::Config;
use binance::futures::websockets::{
    futures_stream_url, FuturesMarket, FuturesStreamRoute, FuturesWebSockets, FuturesWebsocketEvent,
};
use binance::util::sign_hmac;
use binance::websockets::{user_data_stream_subscribe_request, WebSockets, WebsocketEvent};
use std::cell::RefCell;
use std::rc::Rc;

fn spot_events(messages: &[&str]) -> Vec<WebsocketEvent> {
    let events = Rc::new(RefCell::new(Vec::new()));
    let sink = events.clone();
    let mut ws = WebSockets::new(move |event| {
        sink.borrow_mut().push(event);
        Ok(())
    });
    for msg in messages {
        ws.test_handle_msg(msg).unwrap();
    }
    let out = events.borrow().clone();
    drop(ws);
    out
}

fn futures_events(messages: &[&str]) -> Vec<FuturesWebsocketEvent> {
    let events = Rc::new(RefCell::new(Vec::new()));
    let sink = events.clone();
    let mut ws = FuturesWebSockets::new(move |event| {
        sink.borrow_mut().push(event);
        Ok(())
    });
    for msg in messages {
        ws.test_handle_msg(msg).unwrap();
    }
    let out = events.borrow().clone();
    drop(ws);
    out
}

#[test]
fn spot_execution_report_through_ws_api_wrapper() {
    let msg = r#"{"subscriptionId":0,"event":{"e":"executionReport","E":1499405658658,"s":"ETHBTC","c":"mUvoqJxFIILMdfAW5iGSOW","S":"BUY","o":"LIMIT","f":"GTC","q":"1.00000000","p":"0.10264410","P":"0.00000000","F":"0.00000000","g":-1,"C":"","x":"NEW","X":"NEW","r":"NONE","i":4293153,"l":"0.00000000","z":"0.00000000","L":"0.00000000","n":"0","N":null,"T":1499405658657,"t":-1,"v":3,"I":8641984,"w":true,"m":false,"M":false,"O":1499405658657,"Z":"0.00000000","Y":"0.00000000","Q":"0.00000000","W":1499405658657,"V":"NONE","eR":"INSUFFICIENT_LIQUIDITY"}}"#;
    let events = spot_events(&[msg]);
    assert_eq!(events.len(), 1);
    match &events[0] {
        WebsocketEvent::OrderTrade(e) => {
            assert_eq!(e.symbol, "ETHBTC");
            assert_eq!(e.order_id, 4293153);
            assert_eq!(e.working_time, Some(1499405658657));
            assert_eq!(e.self_trade_prevention_mode.as_deref(), Some("NONE"));
            assert_eq!(e.prevented_match_id, Some(3));
            assert_eq!(e.expiry_reason.as_deref(), Some("INSUFFICIENT_LIQUIDITY"));
        }
        other => panic!("unexpected {:?}", other),
    }
}

#[test]
fn spot_account_events_are_dispatched_by_name() {
    let position = r#"{"e":"outboundAccountPosition","E":1564034571105,"u":1564034571073,"B":[{"a":"ETH","f":"10000.000000","l":"0.000000"}]}"#;
    let balance =
        r#"{"e":"balanceUpdate","E":1573200697110,"a":"BTC","d":"100.00000000","T":1573200697068}"#;
    let list = r#"{"e":"listStatus","E":1564035303637,"s":"ETHBTC","g":2,"c":"OCO","l":"EXEC_STARTED","L":"EXECUTING","r":"NONE","C":"F4QN4G8DlFATFlIUQ0cjdD","T":1564035303625,"O":[{"s":"ETHBTC","i":17,"c":"AJYsMjErWJesZvqlJCTUgL"},{"s":"ETHBTC","i":18,"c":"bfYPSQdLoqAJeNrOr9adzq"}]}"#;
    let lock = r#"{"e":"externalLockUpdate","E":1581557507324,"a":"NEO","d":"10.00000000","T":1581557507268}"#;
    let terminated =
        r#"{"subscriptionId":0,"event":{"e":"eventStreamTerminated","E":1728973001334}}"#;
    let shutdown = r#"{"e":"serverShutdown","E":1728973001334}"#;
    let events = spot_events(&[position, balance, list, lock, terminated, shutdown]);
    assert_eq!(events.len(), 6);
    assert!(
        matches!(&events[0], WebsocketEvent::OutboundAccountPosition(e) if e.balances[0].asset == "ETH")
    );
    assert!(
        matches!(&events[1], WebsocketEvent::SpotBalanceUpdate(e) if e.delta == "100.00000000")
    );
    assert!(
        matches!(&events[2], WebsocketEvent::ListStatus(e) if e.orders.len() == 2 && e.order_list_id == 2)
    );
    assert!(matches!(&events[3], WebsocketEvent::ExternalLockUpdate(e) if e.asset == "NEO"));
    assert!(matches!(
        &events[4],
        WebsocketEvent::EventStreamTerminated(_)
    ));
    assert!(matches!(&events[5], WebsocketEvent::ServerShutdown(_)));
}

#[test]
fn spot_ws_api_responses_are_checked() {
    let ok = r#"{"id":"5494febb","status":200,"result":{"subscriptionId":0}}"#;
    assert!(spot_events(&[ok]).is_empty());

    let mut ws = WebSockets::new(|_| Ok(()));
    let err = ws
        .test_handle_msg(r#"{"id":"5494febb","status":400,"error":{"code":-1022,"msg":"Signature for this request is not valid."}}"#)
        .unwrap_err();
    assert!(err.to_string().contains("-1022"), "{}", err);
}

#[test]
fn spot_market_streams_still_parse() {
    let trade = r#"{"e":"trade","E":1672515782136,"s":"BNBBTC","t":12345,"p":"0.001","q":"100","T":1672515782136,"m":true,"M":true}"#;
    let events = spot_events(&[trade]);
    assert!(matches!(&events[0], WebsocketEvent::Trade(e) if e.trade_id == 12345));
}

#[test]
fn user_data_stream_subscribe_request_is_signed_over_sorted_params() {
    let request =
        user_data_stream_subscribe_request("my-api-key", "my-secret", 5000, 1770736694138, "req-1");
    let value: serde_json::Value = serde_json::from_str(&request).unwrap();
    assert_eq!(value["id"], "req-1");
    assert_eq!(value["method"], "userDataStream.subscribe.signature");
    assert_eq!(value["params"]["apiKey"], "my-api-key");
    assert_eq!(value["params"]["timestamp"], 1770736694138u64);
    assert_eq!(value["params"]["recvWindow"], 5000);
    let expected = sign_hmac(
        "my-secret",
        "apiKey=my-api-key&recvWindow=5000&timestamp=1770736694138",
    );
    assert_eq!(value["params"]["signature"], expected);

    let no_window = user_data_stream_subscribe_request("k", "s", 0, 1, "id");
    let value: serde_json::Value = serde_json::from_str(&no_window).unwrap();
    assert!(value["params"].get("recvWindow").is_none());
    assert_eq!(
        value["params"]["signature"],
        sign_hmac("s", "apiKey=k&timestamp=1")
    );
}

#[test]
fn config_has_ws_api_endpoints() {
    assert_eq!(
        Config::default().ws_api_endpoint,
        "wss://ws-api.binance.com:443/ws-api/v3"
    );
    assert_eq!(
        Config::testnet().ws_api_endpoint,
        "wss://ws-api.testnet.binance.vision/ws-api/v3"
    );
}

#[test]
fn futures_user_data_events() {
    let algo = r#"{"e":"ALGO_UPDATE","T":1568879465650,"E":1568879465651,"o":{"caid":"myAlgo","aid":2146760,"at":"CONDITIONAL","o":"TRAILING_STOP_MARKET","s":"BTCUSDT","S":"SELL","ps":"BOTH","f":"GTC","q":"0.5","X":"TRIGGERED","ai":"8886774","ap":"0","aq":"0","act":"MARKET","tp":"0","p":"0","V":"NONE","wt":"MARK_PRICE","pm":"NONE","cp":false,"pP":true,"R":true,"tt":1568879465650,"gtd":0,"rm":"","ia":true}}"#;
    let lite = r#"{"e":"TRADE_LITE","E":1721895408092,"T":1721895408214,"s":"BTCUSDT","q":"0.001","p":"0","m":false,"c":"z8hcUoOsqEdKMeprq6","S":"BUY","L":"64089.20","l":"0.040","t":109100866,"i":8886774}"#;
    let order = r#"{"e":"ORDER_TRADE_UPDATE","E":1568879465651,"T":1568879465650,"o":{"s":"BTCUSDT","c":"TEST","S":"SELL","o":"LIMIT","f":"GTC","q":"0.001","p":"9910","ap":"0","sp":"0","x":"EXPIRED","X":"EXPIRED","i":8886774,"l":"0","z":"0","L":"0","N":"USDT","n":"0","T":1568879465650,"t":0,"b":"0","a":"9.91","m":false,"R":false,"wt":"CONTRACT_PRICE","ot":"LIMIT","ps":"LONG","cp":false,"pP":false,"si":0,"ss":0,"rp":"0","V":"EXPIRE_TAKER","pm":"OPPONENT","gtd":0,"er":"8","M":"7"}}"#;
    let account = r#"{"e":"ACCOUNT_UPDATE","E":1564745798939,"T":1564745798938,"a":{"m":"FUNDING_FEE","B":[{"a":"USDT","wb":"122624.12345678","cw":"100.12345678","bc":"50.12345678"}],"P":[{"s":"BTCUSDT","pa":"20","ep":"6563.66500","bep":"6563.7","cr":"0","up":"2850.21200","mt":"isolated","iw":"13200.70726908","ps":"LONG"}],"S":"BTCUSDT"}}"#;
    let margin_call = r#"{"e":"MARGIN_CALL","E":1587727187525,"cw":"3.16812045","p":[{"s":"ETHUSDT","ps":"LONG","pa":"1.327","mt":"CROSSED","iw":"0","mp":"187.17127","up":"-1.166074","mm":"1.614445"}]}"#;
    let config_update = r#"{"e":"ACCOUNT_CONFIG_UPDATE","E":1611646737479,"T":1611646737476,"ac":{"s":"BTCUSDT","l":25}}"#;
    let expired = r#"{"e":"listenKeyExpired","E":1576653824250,"listenKey":"abc"}"#;
    let events = futures_events(&[
        algo,
        lite,
        order,
        account,
        margin_call,
        config_update,
        expired,
    ]);
    assert_eq!(events.len(), 7);
    match &events[0] {
        FuturesWebsocketEvent::AlgoUpdate(e) => {
            assert_eq!(e.algo_order.algo_id, 2146760);
            assert_eq!(e.algo_order.algo_status, "TRIGGERED");
            assert_eq!(e.algo_order.order_id.as_deref(), Some("8886774"));
            assert_eq!(e.algo_order.is_activated, Some(true));
        }
        other => panic!("unexpected {:?}", other),
    }
    assert!(
        matches!(&events[1], FuturesWebsocketEvent::TradeLite(e) if e.order_id == 8886774 && e.last_filled_price == "64089.20")
    );
    match &events[2] {
        FuturesWebsocketEvent::OrderTrade(e) => {
            assert_eq!(e.order.expiry_reason.as_deref(), Some("8"));
            assert_eq!(e.order.price_match.as_deref(), Some("OPPONENT"));
            assert_eq!(
                e.order.self_trade_prevention_mode.as_deref(),
                Some("EXPIRE_TAKER")
            );
            assert_eq!(e.order.modify_id.as_deref(), Some("7"));
        }
        other => panic!("unexpected {:?}", other),
    }
    match &events[3] {
        FuturesWebsocketEvent::AccountUpdate(e) => {
            assert_eq!(e.data.symbol.as_deref(), Some("BTCUSDT"));
            assert_eq!(
                e.data.positions[0].breakeven_price.as_deref(),
                Some("6563.7")
            );
        }
        other => panic!("unexpected {:?}", other),
    }
    assert!(
        matches!(&events[4], FuturesWebsocketEvent::MarginCall(e) if e.positions[0].symbol == "ETHUSDT")
    );
    assert!(
        matches!(&events[5], FuturesWebsocketEvent::AccountConfigUpdate(e) if e.leverage_config.as_ref().unwrap().leverage == 25)
    );
    assert!(matches!(
        &events[6],
        FuturesWebsocketEvent::UserDataStreamExpiredEvent(_)
    ));
}

#[test]
fn futures_mark_price_stream_with_moving_average() {
    let msg = r#"{"e":"markPriceUpdate","E":1562305380000,"s":"BTCUSDT","p":"11794.15000000","i":"11784.62659091","P":"11784.25641265","r":"0.00038167","ap":"11790.1","T":1562306400000}"#;
    let events = futures_events(&[msg]);
    assert!(
        matches!(&events[0], FuturesWebsocketEvent::MarkPrice(e) if e.mark_price_moving_average.as_deref() == Some("11790.1"))
    );
}

#[test]
fn futures_stream_routes() {
    assert_eq!(
        FuturesStreamRoute::for_stream("btcusdt@depth"),
        FuturesStreamRoute::Public
    );
    assert_eq!(
        FuturesStreamRoute::for_stream("btcusdt@depth20@100ms"),
        FuturesStreamRoute::Public
    );
    assert_eq!(
        FuturesStreamRoute::for_stream("btcusdt@bookTicker"),
        FuturesStreamRoute::Public
    );
    assert_eq!(
        FuturesStreamRoute::for_stream("!bookTicker"),
        FuturesStreamRoute::Public
    );
    assert_eq!(
        FuturesStreamRoute::for_stream("btcusdt@aggTrade"),
        FuturesStreamRoute::Market
    );
    assert_eq!(
        FuturesStreamRoute::for_stream("btcusdt@markPrice@1s"),
        FuturesStreamRoute::Market
    );
    assert_eq!(
        FuturesStreamRoute::for_stream("!markPrice@arr"),
        FuturesStreamRoute::Market
    );
    assert_eq!(
        FuturesStreamRoute::for_stream("btcusdt@kline_1m"),
        FuturesStreamRoute::Market
    );
    assert_eq!(
        FuturesStreamRoute::for_stream(
            "XaEAKTsQSRLZAGH9tuIu37plSRsdjmlAVBoNYPUITlTAko1WI22PgmBMpI1rS8Yh"
        ),
        FuturesStreamRoute::Private
    );
    assert_eq!(
        futures_stream_url(
            &FuturesMarket::USDM,
            FuturesStreamRoute::Market,
            "btcusdt@aggTrade"
        ),
        "wss://fstream.binance.com/market/ws/btcusdt@aggTrade"
    );
    assert_eq!(
        futures_stream_url(
            &FuturesMarket::USDMTestnet,
            FuturesStreamRoute::Private,
            "key"
        ),
        "wss://fstream.binancefuture.com/private/ws/key"
    );
    assert_eq!(
        futures_stream_url(
            &FuturesMarket::COINM,
            FuturesStreamRoute::Root,
            "btcusd_perp@aggTrade"
        ),
        "wss://dstream.binance.com/ws/btcusd_perp@aggTrade"
    );
}
