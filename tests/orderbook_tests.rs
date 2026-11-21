#![cfg(feature = "orderbook")]

use binance::model::{DepthOrderBookEvent, OrderBook as Snapshot};
use binance::orderbook::{BookSync, Market, OrderBook, Outcome, ResyncReason};

fn event(json: &str) -> DepthOrderBookEvent {
    serde_json::from_str(json).unwrap()
}

#[test]
fn dropped_diff_forces_resync_instead_of_a_stale_bid() {
    let mut book = OrderBook::new();
    let mut sync = BookSync::new(Market::Spot);

    let first = event(
        r#"{"e":"depthUpdate","E":1,"s":"BTCUSDT","U":101,"u":102,"b":[["64250.10","0.5"]],"a":[]}"#,
    );
    assert_eq!(sync.on_event(&mut book, (&first).into()), Outcome::Buffered);

    let snapshot: Snapshot = serde_json::from_str(
        r#"{"lastUpdateId":100,"bids":[["64250.00","1.2"]],"asks":[["64251.00","0.8"]]}"#,
    )
    .unwrap();
    sync.on_snapshot(&mut book, (&snapshot).into());
    assert!(sync.is_live());
    assert_eq!(book.best_bid().unwrap().price.to_string(), "64250.1");

    // 103..=104 (which cancels the 64250.10 bid) never arrives.
    let next = event(
        r#"{"e":"depthUpdate","E":3,"s":"BTCUSDT","U":105,"u":105,"b":[],"a":[["64251.00","0.7"]]}"#,
    );
    assert_eq!(
        sync.on_event(&mut book, (&next).into()),
        Outcome::Resync(ResyncReason::Gap {
            expected: 103,
            got: 105
        })
    );
    assert!(book.best_bid().is_none());
    assert!(sync.needs_snapshot());
}

#[test]
fn futures_events_are_sequenced_by_pu() {
    let mut book = OrderBook::new();
    let mut sync = BookSync::new(Market::UsdM);
    let snapshot: Snapshot =
        serde_json::from_str(r#"{"lastUpdateId":100,"bids":[["1.5","2"]],"asks":[["1.6","3"]]}"#)
            .unwrap();

    let a = event(
        r#"{"e":"depthUpdate","E":1,"s":"BTCUSDT","U":95,"u":105,"pu":90,"b":[["1.5","0"]],"a":[]}"#,
    );
    sync.on_event(&mut book, (&a).into());
    sync.on_snapshot(&mut book, (&snapshot).into());
    assert!(sync.is_live());
    assert!(book.best_bid().is_none());

    let b =
        event(r#"{"e":"depthUpdate","E":2,"s":"BTCUSDT","U":110,"u":112,"pu":104,"b":[],"a":[]}"#);
    assert!(matches!(
        sync.on_event(&mut book, (&b).into()),
        Outcome::Resync(_)
    ));
}
