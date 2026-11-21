//! Local order book sync for the depth stream, behind the `orderbook` feature.
//!
//! `DepthOrderBookEvent` and `OrderBook` are plain diffs and snapshots. Keeping a
//! correct local book from them means Binance's buffer / snapshot / `U`-`u`-`pu`
//! handshake, which this module delegates to [`crypto_orderbook::BookSync`]:
//! feed it events and snapshots, it applies them or tells you to resync.
//!
//! ```
//! use binance::orderbook::{BookSync, Market, OrderBook, Outcome};
//! # let event: binance::model::DepthOrderBookEvent = serde_json::from_str(
//! #     r#"{"e":"depthUpdate","E":1,"s":"BTCUSDT","U":101,"u":102,"b":[["64250.10","0.5"]],"a":[]}"#).unwrap();
//! # let snapshot: binance::model::OrderBook = serde_json::from_str(
//! #     r#"{"lastUpdateId":100,"bids":[["64250.00","1.2"]],"asks":[["64251.00","0.8"]]}"#).unwrap();
//! let mut book = OrderBook::new();
//! let mut sync = BookSync::new(Market::Spot);
//! assert_eq!(sync.on_event(&mut book, (&event).into()), Outcome::Buffered);
//! sync.on_snapshot(&mut book, (&snapshot).into());
//! assert!(sync.is_live());
//! ```

use std::str::FromStr;

pub use crypto_orderbook::{
    BookSync, DepthSnapshot, DepthUpdate, Market, OrderBook, Outcome, PriceLevel, ResyncReason,
};
use rust_decimal::Decimal;

use crate::model::{Asks, Bids, DepthOrderBookEvent};

// f64 `Display` is the shortest string that round-trips, i.e. the decimal Binance sent.
fn level(price: f64, qty: f64) -> PriceLevel {
    let dec = |v: f64| Decimal::from_str(&v.to_string()).unwrap_or_default();
    PriceLevel {
        price: dec(price),
        quantity: dec(qty),
    }
}

fn bids(levels: &[Bids]) -> Vec<PriceLevel> {
    levels.iter().map(|l| level(l.price, l.qty)).collect()
}

fn asks(levels: &[Asks]) -> Vec<PriceLevel> {
    levels.iter().map(|l| level(l.price, l.qty)).collect()
}

impl From<&DepthOrderBookEvent> for DepthUpdate {
    fn from(e: &DepthOrderBookEvent) -> Self {
        DepthUpdate {
            first_update_id: e.first_update_id,
            last_update_id: e.final_update_id,
            prev_last_update_id: e.previous_final_update_id,
            bids: bids(&e.bids),
            asks: asks(&e.asks),
        }
    }
}

impl From<&crate::model::OrderBook> for DepthSnapshot {
    fn from(s: &crate::model::OrderBook) -> Self {
        DepthSnapshot {
            last_update_id: s.last_update_id,
            bids: bids(&s.bids),
            asks: asks(&s.asks),
        }
    }
}

impl From<&crate::futures::model::OrderBook> for DepthSnapshot {
    fn from(s: &crate::futures::model::OrderBook) -> Self {
        DepthSnapshot {
            last_update_id: s.last_update_id,
            bids: bids(&s.bids),
            asks: asks(&s.asks),
        }
    }
}
