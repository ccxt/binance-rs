use binance::api::*;
use binance::savings::*;
use binance::config::*;
use binance::general::*;
use binance::account::*;
use binance::market::*;
use binance::model::KlineSummary;
use binance::errors::SdkError as BinanceLibErrorKind;

fn main() {
    // The general spot API endpoints; shown with
    // testnet=false and testnet=true
    general(false);
    general(true);

    // The market data API endpoint
    market_data();

    // The account data API and savings API endpoint examples need an API key. Change those lines locally
    // and uncomment the line below (and do not commit your api key :)).
    //account();
    //savings();
}

fn general(use_testnet: bool) {
    let general: General = if use_testnet {
        let config = Config::default().set_rest_api_endpoint("https://testnet.binance.vision");
        Binance::new_with_config(None, None, &config)
    } else {
        Binance::new(None, None)
    };

    let ping = general.ping();
    match ping {
        Ok(answer) => println!("{:?}", answer),
        Err(err) => {
            match err {
                BinanceLibErrorKind::BinanceError(response) => match response.code {
                    -1000_i16 => println!("An unknown error occured while processing the request"),
                    _ => println!("Non-catched code {}: {}", response.code, response.msg),
                },
                _ => println!("Other errors: {}.", err),
            };
        }
    }

    let result = general.get_server_time();
    match result {
        Ok(answer) => println!("Server Time: {}", answer.server_time),
        Err(e) => println!("Error: {}", e),
    }

    let result = general.exchange_info();
    match result {
        Ok(answer) => println!("Exchange information: {:?}", answer),
        Err(e) => println!("Error: {}", e),
    }

    let result = general.get_symbol_info("ethbtc");
    match result {
        Ok(answer) => println!("Symbol information: {:?}", answer),
        Err(e) => println!("Error: {}", e),
    }
}

#[allow(dead_code)]
fn account() {
    let api_key = Some("YOUR_API_KEY".into());
    let secret_key = Some("YOUR_SECRET_KEY".into());

    let account: Account = Binance::new(api_key, secret_key);

    match account.get_account() {
        Ok(answer) => println!("{:?}", answer.balances),
        Err(e) => println!("Error: {}", e),
    }

    match account.get_open_orders("WTCETH") {
        Ok(answer) => println!("{:?}", answer),
        Err(e) => println!("Error: {}", e),
    }

    match account.limit_buy("WTCETH", 10, 0.014000) {
        Ok(answer) => println!("{:?}", answer),
        Err(e) => println!("Error: {}", e),
    }

    match account.market_buy("WTCETH", 5) {
        Ok(answer) => println!("{:?}", answer),
        Err(e) => println!("Error: {}", e),
    }

    match account.market_buy_using_quote_quantity("WTCETH", 5) {
        Ok(answer) => println!("{:?}", answer),
        Err(e) => println!("Error: {}", e),
    }

    match account.limit_sell("WTCETH", 10, 0.035000) {
        Ok(answer) => println!("{:?}", answer),
        Err(e) => println!("Error: {}", e),
    }

    match account.market_sell("WTCETH", 5) {
        Ok(answer) => println!("{:?}", answer),
        Err(e) => println!("Error: {}", e),
    }

    match account.market_sell_using_quote_quantity("WTCETH", 5) {
        Ok(answer) => println!("{:?}", answer),
        Err(e) => println!("Error: {}", e),
    }

    let order_id = 1_957_528;
    match account.order_status("WTCETH", order_id) {
        Ok(answer) => println!("{:?}", answer),
        Err(e) => println!("Error: {}", e),
    }

    match account.cancel_order("WTCETH", order_id) {
        Ok(answer) => println!("{:?}", answer),
        Err(e) => println!("Error: {}", e),
    }

    match account.get_balance("KNC") {
        Ok(answer) => println!("{:?}", answer),
        Err(e) => println!("Error: {}", e),
    }

    match account.trade_history("WTCETH") {
        Ok(answer) => println!("{:?}", answer),
        Err(e) => println!("Error: {}", e),
    }
}

#[allow(dead_code)]
fn savings() {
    let api_key = Some("YOUR_API_KEY".into());
    let api_secret = Some("YOUR_SECRET_KEY".into());

    let savings: Savings = Binance::new(api_key, api_secret);

    match savings.get_all_coins() {
        Ok(answer) => println!("{:#?}", answer),
        Err(e) => println!("Error: {}", e),
    }

    match savings.asset_detail(None) {
        Ok(answer) => println!("{:#?}", answer),
        Err(e) => println!("Error: {}", e),
    }

    match savings.deposit_address("BTC", None) {
        Ok(answer) => println!("{:#?}", answer),
        Err(e) => println!("Error: {:?}", e),
    }
}

#[allow(dead_code)]
fn market_data() {
    let market: Market = Binance::new(None, None);

    // Order book at default depth
    match market.get_depth("BNBETH") {
        Ok(answer) => println!("{:?}", answer),
        Err(e) => println!("Error: {}", e),
    }
    // Order book at depth 500
    match market.get_custom_depth("BNBETH", 500) {
        Ok(answer) => println!("{:?}", answer),
        Err(e) => println!("Error: {}", e),
    }

    // Latest price for ALL symbols
    match market.get_all_prices() {
        Ok(answer) => println!("{:?}", answer),
        Err(e) => println!("Error: {}", e),
    }

    // Latest price for ONE symbol
    match market.get_price("KNCETH") {
        Ok(answer) => println!("{:?}", answer),
        Err(e) => println!("Error: {}", e),
    }

    // Current average price for ONE symbol
    match market.get_average_price("KNCETH") {
        Ok(answer) => println!("{:?}", answer),
        Err(e) => println!("Error: {}", e),
    }

    // Best price/qty on the order book for ALL symbols
    match market.get_all_book_tickers() {
        Ok(answer) => println!("{:?}", answer),
        Err(e) => println!("Error: {}", e),
    }

    // Best price/qty on the order book for ONE symbol
    match market.get_book_ticker("BNBETH") {
        Ok(answer) => println!(
            "Bid Price: {}, Ask Price: {}",
            answer.bid_price, answer.ask_price
        ),
        Err(e) => println!("Error: {}", e),
    }

    // 24hr ticker price change statistics
    match market.get_24h_price_stats("BNBETH") {
        Ok(answer) => println!(
            "Open Price: {}, Higher Price: {}, Lower Price: {:?}",
            answer.open_price, answer.high_price, answer.low_price
        ),
        Err(e) => println!("Error: {}", e),
    }

    // 10 latest (aggregated) trades
    match market.get_agg_trades("BNBETH", None, None, None, Some(10)) {
        Ok(trades) => {
            let trade = &trades[0]; // You need to iterate over them
            println!(
                "{} BNB Qty: {}, Price: {}",
                if trade.maker { "SELL" } else { "BUY" },
                trade.qty,
                trade.price
            );
        }
        Err(e) => println!("Error: {}", e),
    }

    // last 10 5min klines (candlesticks) for a symbol:
    match market.get_klines("BNBETH", "5m", 10, None, None) {
        Ok(klines) => {
            match klines {
                binance::model::KlineSummaries::AllKlineSummaries(klines) => {
                    let kline: KlineSummary = klines[0].clone(); // You need to iterate over the klines
                    println!(
                        "Open: {}, High: {}, Low: {}",
                        kline.open, kline.high, kline.low
                    );
                }
            }
        }
        Err(e) => println!("Error: {}", e),
    }
}
