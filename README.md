# binance-rs

Unofficial Rust Library for the [Binance Spot API](https://developers.binance.com/docs/binance-spot-api-docs) and the [Binance USDⓈ-M Futures API](https://developers.binance.com/docs/derivatives/usds-margined-futures/general-info).

Endpoints and models track the official changelogs as of October 2026, including:

- Spot: order lists (OCO/OTO/OTOCO), cancel-replace, amend-keep-priority, commission rates, `myFilters`, order rate limits, trading-day and rolling-window tickers, `uiKlines`, and the WebSocket API user data stream that replaced the retired `listenKey` endpoints.
- Futures: conditional orders through the Algo service (`/fapi/v1/algoOrder`), batch place/cancel, modify order, V3 account/balance/position endpoints, symbol and account config, leverage brackets, commission rate, funding rate history, long/short ratios, basis, insurance fund, routed `/public`, `/market` and `/private` websocket streams, and the `ALGO_UPDATE` / `TRADE_LITE` events.

[![Crates.io](https://img.shields.io/crates/v/binance.svg)](https://crates.io/crates/binance)
[![Build Status](https://travis-ci.org/wisespace-io/binance-rs.png?branch=master)](https://travis-ci.org/wisespace-io/binance-rs)
[![CI](https://github.com/wisespace-io/binance-rs/workflows/Rust/badge.svg)](https://github.com/wisespace-io/binance-rs/actions?query=workflow%3ARust)
[![MIT licensed](https://img.shields.io/badge/License-MIT-blue.svg)](./LICENSE-MIT)
[![Apache-2.0 licensed](https://img.shields.io/badge/License-Apache%202.0-blue.svg)](./LICENSE-APACHE)

[Documentation on docs.rs](https://docs.rs/crate/binance/)

## Binance API Telegram

  <https://t.me/binance_api_english>

## Usage

Add this to your Cargo.toml

```toml
[dependencies]
binance = { git = "https://github.com/ccxt/binance-rs.git" }
```

## Rust >= 1.89.0

```rust
rustup install stable
```

### Table of Contents  
- [MARKET DATA](#market-data)
- [ACCOUNT DATA](#account-data)
- [ERROR HANDLING](#error-handling)
- [TESTNET AND API CLUSTERS](#testnet-and-api-clusters)
- [USER STREAM CONFIGURATION](#user-stream-configuration)
- [WEBSOCKETS](#websockets)
  - [USER STREAM](#user-stream)
  - [TRADES](#trades)
  - [KLINE](#kline)
  - [MULTIPLE STREAMS](#multiple-streams)

### MARKET DATA

```rust
use binance::api::*;
use binance::model::*;
use binance::market::*;

fn main() {
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
        Err(e) => println!("Error: {:?}", e),
    }

    // Latest price for ONE symbol
    match market.get_price("BNBETH") {
        Ok(answer) => println!("{:?}", answer),
        Err(e) => println!("Error: {:?}", e),
    }

    // Current average price for ONE symbol
    match market.get_average_price("BNBETH") {
        Ok(answer) => println!("{:?}", answer),
        Err(e) => println!("Error: {:?}", e),
    }

    // Best price/qty on the order book for ALL symbols
    match market.get_all_book_tickers() {
        Ok(answer) => println!("{:?}", answer),
        Err(e) => println!("Error: {:?}", e),
    }

    // Best price/qty on the order book for ONE symbol
    match market.get_book_ticker("BNBETH") {
        Ok(answer) => println!(
            "Bid Price: {}, Ask Price: {}",
            answer.bid_price, answer.ask_price
        ),
        Err(e) => println!("Error: {:?}", e),
    }

    // 24hr ticker price change statistics
    match market.get_24h_price_stats("BNBETH") {
        Ok(answer) => println!(
            "Open Price: {}, Higher Price: {}, Lower Price: {:?}",
            answer.open_price, answer.high_price, answer.low_price
        ),
        Err(e) => println!("Error: {:?}", e),
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
                    )
                }
            }
        },
        Err(e) => println!("Error: {}", e),
    }
}
```

### ACCOUNT DATA

```rust
use binance::api::*;
use binance::account::*;

fn main() {
    let api_key = Some("YOUR_API_KEY".into());
    let secret_key = Some("YOUR_SECRET_KEY".into());

    let account: Account = Binance::new(api_key, secret_key);

    match account.get_account() {
        Ok(answer) => println!("{:?}", answer.balances),
        Err(e) => println!("Error: {:?}", e),
    }

    match account.get_open_orders("WTCETH") {
        Ok(answer) => println!("{:?}", answer),
        Err(e) => println!("Error: {:?}", e),
    }

    match account.limit_buy("WTCETH", 10, 0.014000) {
        Ok(answer) => println!("{:?}", answer),
        Err(e) => println!("Error: {:?}", e),
    }

    match account.market_buy("WTCETH", 5) {
        Ok(answer) => println!("{:?}", answer),
        Err(e) => println!("Error: {:?}", e),
    }

    match account.limit_sell("WTCETH", 10, 0.035000) {
        Ok(answer) => println!("{:?}", answer),
        Err(e) => println!("Error: {:?}", e),
    }

    match account.market_sell("WTCETH", 5) {
        Ok(answer) => println!("{:?}", answer),
        Err(e) => println!("Error: {:?}", e),
    }

    match account.custom_order("WTCETH", 9999, 0.0123, "SELL", "LIMIT", "IOC") {
        Ok(answer) => println!("{:?}", answer),
        Err(e) => println!("Error: {:?}", e),
    }

    let order_id = 1_957_528;
    match account.order_status("WTCETH", order_id) {
        Ok(answer) => println!("{:?}", answer),
        Err(e) => println!("Error: {:?}", e),
    }

    match account.cancel_order("WTCETH", order_id) {
        Ok(answer) => println!("{:?}", answer),
        Err(e) => println!("Error: {:?}", e),
    }

    match account.cancel_all_open_orders("WTCETH") {
        Ok(answer) => println!("{:?}", answer),
        Err(e) => println!("Error: {:?}", e),
    }

    match account.get_balance("KNC") {
        Ok(answer) => println!("{:?}", answer),
        Err(e) => println!("Error: {:?}", e),
    }

    match account.trade_history("WTCETH") {
        Ok(answer) => println!("{:?}", answer),
        Err(e) => println!("Error: {:?}", e),
    }
}
```

### ERROR HANDLING

Provides more detailed error information

You can check out the [Binance Error Codes](https://github.com/binance-exchange/binance-official-api-docs/blob/master/errors.md)

```rust
use binance::errors::ErrorKind as BinanceLibErrorKind;

[...]

Err(err) => {
    println!("Can't put an order!");

    match err.0 {
        BinanceLibErrorKind::BinanceError(response) => match response.code {
            -1013_i16 => println!("Filter failure: LOT_SIZE!"),
            -2010_i16 => println!("Funds insufficient! {}", response.msg),
            _ => println!("Non-catched code {}: {}", response.code, response.msg),
        },
        BinanceLibErrorKind::Msg(msg) => {
            println!("Binancelib error msg: {}", msg)
        }
        _ => println!("Other errors: {}.", err.0),
    };
}
```

### TESTNET AND API CLUSTERS

You can overwrite the default binance api urls if there are performance issues with the endpoints.

You can check out the [Binance API Clusters](https://github.com/binance/binance-spot-api-docs/blob/master/rest-api.md#general-api-information).

The same is applicable for Testnet and Binance.US support. See example below:

```rust
let general: General = if use_testnet {
    let config = Config::default().set_rest_api_endpoint("https://testnet.binance.vision");
                                  // .set_ws_endpoint("wss://testnet.binance.vision/ws")
                                  // .set_futures_rest_api_endpoint("https://testnet.binancefuture.com/api")
                                  // .set_futures_ws_endpoint("https://testnet.binancefuture.com/ws")
    Binance::new_with_config(None, None, &config)
} else {
    Binance::new(None, None)
};
```

### USER STREAM CONFIGURATION

Binance retired the `listenKey` based spot user data stream (`POST /api/v3/userDataStream`)
on 2026-02-20. Spot account events are now delivered through the WebSocket API
(`wss://ws-api.binance.com:443/ws-api/v3`) after a signed `userDataStream.subscribe.signature`
request, which `WebSockets::connect_user_data_stream` sends for you. HMAC API keys work; no
`session.logon` is needed. Events arrive wrapped in `{"subscriptionId": n, "event": {...}}`
and are unwrapped before reaching your handler.

USDⓈ-M futures still use a `listenKey` (`FuturesUserStream::start`), connected through the
routed `/private` endpoint (`wss://fstream.binance.com/private/ws/<listenKey>`), which
`FuturesWebSockets::connect` picks automatically.

#### USER STREAM

```rust
use binance::config::Config;
use binance::websockets::*;
use std::sync::atomic::{AtomicBool};

fn main() {
    let keep_running = AtomicBool::new(true); // Used to control the event loop
    let config = Config::default(); // Config::testnet() for the Spot testnet

    let mut web_socket = WebSockets::new(|event: WebsocketEvent| {
        match event {
            WebsocketEvent::OutboundAccountPosition(position) => {
                for balance in &position.balances {
                    println!("Asset: {}, free: {}, locked: {}", balance.asset, balance.free, balance.locked);
                }
            },
            WebsocketEvent::OrderTrade(trade) => {
                println!("Symbol: {}, Side: {}, Price: {}, Execution Type: {}", trade.symbol, trade.side, trade.price, trade.execution_type);
            },
            WebsocketEvent::EventStreamTerminated(_) => {
                println!("Subscription closed, reconnect");
            },
            _ => (),
        };
        Ok(())
    });

    web_socket.connect_user_data_stream("YOUR_KEY", "YOUR_SECRET", &config).unwrap(); // check error
    if let Err(e) = web_socket.event_loop(&keep_running) {
        println!("Error: {:?}", e);
    }
}
```

#### TRADES

```rust
use binance::websockets::*;
use std::sync::atomic::{AtomicBool};

fn main() {
    let keep_running = AtomicBool::new(true); // Used to control the event loop
    let agg_trade = format!("!ticker@arr"); // All Symbols
    let mut web_socket = WebSockets::new(|event: WebsocketEvent| {
	match event {
        // 24hr rolling window ticker statistics for all symbols that changed in an array.
	    WebsocketEvent::DayTickerAll(ticker_events) => {
	        for tick_event in ticker_events {
		    if tick_event.symbol == "BTCUSDT" {
			let btcusdt: f32 = tick_event.average_price.parse().unwrap();
			let btcusdt_close: f32 = tick_event.current_close.parse().unwrap();
			println!("{} - {}", btcusdt, btcusdt_close);
		    }
		}
	    },
	    _ => (),
        };

        Ok(())
    });

    web_socket.connect(&agg_trade).unwrap(); // check error
    if let Err(e) = web_socket.event_loop(&keep_running) {
	match e {
	    err => {
	        println!("Error: {:?}", err);
	    }
	}
     }
}
```

#### KLINE

```rust
use binance::websockets::*;
use std::sync::atomic::{AtomicBool};

fn main() {
    let keep_running = AtomicBool::new(true); // Used to control the event loop
    let kline = format!("{}", "ethbtc@kline_1m");
    let mut web_socket = WebSockets::new(|event: WebsocketEvent| {
        match event {
            WebsocketEvent::Kline(kline_event) => {
                println!("Symbol: {}, high: {}, low: {}", kline_event.kline.symbol, kline_event.kline.low, kline_event.kline.high);
            },
            _ => (),
        };
        Ok(())
    });
 
    web_socket.connect(&kline).unwrap(); // check error
    if let Err(e) = web_socket.event_loop(&keep_running) {
        match e {
          err => {
             println!("Error: {:?}", err);
          }
        }
     }
     web_socket.disconnect().unwrap();
}

```

#### MULTIPLE STREAMS

```rust
use binance::websockets::*;
use std::sync::atomic::{AtomicBool};

fn main() {
    let endpoints = ["ETHBTC", "BNBETH"]
        .map(|symbol| format!("{}@depth@100ms", symbol.to_lowercase()));

    let keep_running = AtomicBool::new(true);
    let mut web_socket = WebSockets::new(|event: WebsocketEvent| {
        if let WebsocketEvent::DepthOrderBook(depth_order_book) = event {
            println!("{:?}", depth_order_book);
        }

        Ok(())
    });

    web_socket.connect_multiple_streams(&endpoints).unwrap(); // check error
    if let Err(e) = web_socket.event_loop(&keep_running) {
        println!("Error: {:?}", e);
    }
    web_socket.disconnect().unwrap();
}

```

### Other Exchanges

If you use [Bitfinex](https://www.bitfinex.com/) check out my [Rust library for bitfinex API](https://github.com/wisespace-io/bitfinex-rs)
