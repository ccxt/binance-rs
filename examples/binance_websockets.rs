#![allow(dead_code)]

use binance::config::Config;
use binance::websockets::*;
use std::sync::atomic::{AtomicBool, Ordering};

fn main() {
    //user_stream_websocket();
    //market_websocket();
    //kline_websocket();
    //all_trades_websocket();
    //last_price_for_one_symbol();
    // multiple_streams();
}

// The listenKey based user stream (`POST /api/v3/userDataStream`) was retired by Binance on
// 2026-02-20. Spot user data is now subscribed through the WebSocket API with an API key
// signature, see `user_stream_websocket` below.

fn user_stream_websocket() {
    let keep_running = AtomicBool::new(true); // Used to control the event loop
    let api_key = "YOUR_KEY";
    let secret_key = "YOUR_SECRET";
    let config = Config::default();

    let mut web_socket: WebSockets<'_> = WebSockets::new(|event: WebsocketEvent| {
        match event {
            WebsocketEvent::OutboundAccountPosition(position) => {
                for balance in &position.balances {
                    println!(
                        "Asset: {}, free: {}, locked: {}",
                        balance.asset, balance.free, balance.locked
                    );
                }
            }
            WebsocketEvent::SpotBalanceUpdate(update) => {
                println!("Asset: {}, delta: {}", update.asset, update.delta);
            }
            WebsocketEvent::OrderTrade(trade) => {
                println!(
                    "Symbol: {}, Side: {}, Price: {}, Execution Type: {}",
                    trade.symbol, trade.side, trade.price, trade.execution_type
                );
            }
            WebsocketEvent::ListStatus(list) => {
                println!(
                    "Order list {}: {}",
                    list.order_list_id, list.list_order_status
                );
            }
            WebsocketEvent::EventStreamTerminated(_) => {
                println!("User data stream terminated, reconnect");
            }
            _ => (),
        };

        Ok(())
    });

    web_socket
        .connect_user_data_stream(api_key, secret_key, &config)
        .unwrap(); // check error
    if let Err(e) = web_socket.event_loop(&keep_running) {
        println!("Error: {}", e);
    }
    web_socket.disconnect().unwrap();
    println!("Userstrem closed and disconnected");
}

fn market_websocket() {
    let keep_running = AtomicBool::new(true); // Used to control the event loop
    let agg_trade = String::from("ethbtc@aggTrade");
    let mut web_socket: WebSockets<'_> = WebSockets::new(|event: WebsocketEvent| {
        match event {
            WebsocketEvent::Trade(trade) => {
                println!(
                    "Symbol: {}, price: {}, qty: {}",
                    trade.symbol, trade.price, trade.qty
                );
            }
            WebsocketEvent::DepthOrderBook(depth_order_book) => {
                println!(
                    "Symbol: {}, Bids: {:?}, Ask: {:?}",
                    depth_order_book.symbol, depth_order_book.bids, depth_order_book.asks
                );
            }
            WebsocketEvent::OrderBook(order_book) => {
                println!(
                    "last_update_id: {}, Bids: {:?}, Ask: {:?}",
                    order_book.last_update_id, order_book.bids, order_book.asks
                );
            }
            _ => (),
        };

        Ok(())
    });

    web_socket.connect(&agg_trade).unwrap(); // check error
    if let Err(e) = web_socket.event_loop(&keep_running) {
        println!("Error: {}", e);
    }
    web_socket.disconnect().unwrap();
    println!("disconnected");
}

fn all_trades_websocket() {
    let keep_running = AtomicBool::new(true); // Used to control the event loop
    let agg_trade = String::from("!ticker@arr");
    let mut web_socket = WebSockets::new(|event: WebsocketEvent| {
        if let WebsocketEvent::DayTickerAll(ticker_events) = event {
            for tick_event in ticker_events {
                println!(
                    "Symbol: {}, price: {}, qty: {}",
                    tick_event.symbol, tick_event.best_bid, tick_event.best_bid_qty
                );
            }
        }

        Ok(())
    });

    web_socket.connect(&agg_trade).unwrap(); // check error
    if let Err(e) = web_socket.event_loop(&keep_running) {
        println!("Error: {}", e);
    }
    web_socket.disconnect().unwrap();
    println!("disconnected");
}

fn kline_websocket() {
    let keep_running = AtomicBool::new(true);
    let kline = String::from("ethbtc@kline_1m");
    let mut web_socket = WebSockets::new(|event: WebsocketEvent| {
        if let WebsocketEvent::Kline(kline_event) = event {
            println!(
                "Symbol: {}, high: {}, low: {}",
                kline_event.kline.symbol, kline_event.kline.low, kline_event.kline.high
            );
        }

        Ok(())
    });

    web_socket.connect(&kline).unwrap(); // check error
    if let Err(e) = web_socket.event_loop(&keep_running) {
        println!("Error: {}", e);
    }
    web_socket.disconnect().unwrap();
    println!("disconnected");
}

fn last_price_for_one_symbol() {
    let keep_running = AtomicBool::new(true);
    let agg_trade = String::from("btcusdt@ticker");
    let mut btcusdt: f32 = "0".parse().unwrap();

    let mut web_socket = WebSockets::new(|event: WebsocketEvent| {
        if let WebsocketEvent::DayTicker(ticker_event) = event {
            btcusdt = ticker_event.average_price.parse().unwrap();
            let btcusdt_close: f32 = ticker_event.current_close.parse().unwrap();
            println!("{} - {}", btcusdt, btcusdt_close);

            if btcusdt_close as i32 == 7000 {
                // Break the event loop
                keep_running.store(false, Ordering::Relaxed);
            }
        }

        Ok(())
    });

    web_socket.connect(&agg_trade).unwrap(); // check error
    if let Err(e) = web_socket.event_loop(&keep_running) {
        println!("Error: {}", e);
    }
    web_socket.disconnect().unwrap();
    println!("disconnected");
}

fn multiple_streams() {
    let endpoints =
        ["ETHBTC", "BNBETH"].map(|symbol| format!("{}@depth@100ms", symbol.to_lowercase()));

    let keep_running = AtomicBool::new(true);
    let mut web_socket = WebSockets::new(|event: WebsocketEvent| {
        if let WebsocketEvent::DepthOrderBook(depth_order_book) = event {
            println!("{:?}", depth_order_book);
        }

        Ok(())
    });

    web_socket.connect_multiple_streams(&endpoints).unwrap(); // check error
    if let Err(e) = web_socket.event_loop(&keep_running) {
        println!("Error: {}", e);
    }
    web_socket.disconnect().unwrap();
}
