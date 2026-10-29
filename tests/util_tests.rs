use binance::util::*;

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::BTreeMap;
    use std::time::{SystemTime, UNIX_EPOCH};
    use float_cmp::*;

    #[test]
    fn build_request_empty() {
        let parameters: BTreeMap<String, String> = BTreeMap::new();
        let result = build_request(parameters);
        assert!(result.is_empty());
    }

    #[test]
    fn build_request_not_empty() {
        let mut parameters: BTreeMap<String, String> = BTreeMap::new();
        parameters.insert("recvWindow".into(), "1234".to_string());
        let result = build_request(parameters);
        assert_eq!(result, format!("recvWindow={}", 1234));
    }

    #[test]
    fn build_signed_request() {
        let now = SystemTime::now();
        let recv_window = 1234;

        let since_epoch = now.duration_since(UNIX_EPOCH).unwrap();
        let timestamp =
            since_epoch.as_secs() * 1000 + u64::from(since_epoch.subsec_nanos()) / 1_000_000;

        let parameters: BTreeMap<String, String> = BTreeMap::new();
        let result =
            binance::util::build_signed_request_custom(parameters, recv_window, now).unwrap();

        assert_eq!(
            result,
            format!("recvWindow={}&timestamp={}", recv_window, timestamp)
        );
    }

    #[test]
    fn to_i64() {
        let value_max = serde_json::json!(i64::MAX);
        let value_min = serde_json::json!(i64::MIN);
        assert_eq!(binance::util::to_i64(&value_max), i64::MAX);
        assert_eq!(binance::util::to_i64(&value_min), i64::MIN);
    }

    #[test]
    fn to_f64() {
        let value = serde_json::json!("123.3");
        assert!(approx_eq!(
            f64,
            binance::util::to_f64(&value),
            123.3,
            ulps = 2
        ));
    }

    #[test]
    fn percent_encode_keeps_unreserved_and_encodes_the_rest() {
        assert_eq!(percent_encode("BTCUSDT"), "BTCUSDT");
        assert_eq!(percent_encode("x-HNA2TXFJabc_1.2~"), "x-HNA2TXFJabc_1.2~");
        assert_eq!(
            percent_encode(r#"["BTCUSDT","BNBUSDT"]"#),
            "%5B%22BTCUSDT%22%2C%22BNBUSDT%22%5D"
        );
        assert_eq!(percent_encode("a b&c=d"), "a%20b%26c%3Dd");
        assert_eq!(percent_encode("１２"), "%EF%BC%91%EF%BC%92");
    }

    #[test]
    fn build_request_encodes_values_in_alphabetical_order() {
        let mut parameters: BTreeMap<String, String> = BTreeMap::new();
        parameters.insert("symbols".into(), json_string_array(&["BTCUSDT", "BNBUSDT"]));
        parameters.insert("limit".into(), "5".into());
        assert_eq!(
            build_request(parameters),
            "limit=5&symbols=%5B%22BTCUSDT%22%2C%22BNBUSDT%22%5D"
        );
    }

    #[test]
    fn sign_hmac_matches_binance_documentation_example() {
        // Example from the Binance Spot REST API docs, "SIGNED Endpoint Examples for POST /api/v3/order"
        let secret = "NhqPtmdSJYdKjVHjA7PZj4Mge3R5YNiP1e3UZjInClVN65XAbvqqM6A7H5fATj0j";
        let payload = "symbol=LTCBTC&side=BUY&type=LIMIT&timeInForce=GTC&quantity=1&price=0.1&recvWindow=5000&timestamp=1499827319559";
        assert_eq!(
            sign_hmac(secret, payload),
            "c8db56825ae71d6d79447849e617115f4a920fa2acdcab2b053c4b2838bd6b71"
        );
    }

    #[test]
    fn json_string_array_format() {
        assert_eq!(json_string_array(&["A"]), r#"["A"]"#);
        assert_eq!(json_string_array::<&str>(&[]), "[]");
    }
}
