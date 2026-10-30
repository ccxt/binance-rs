use crate::errors::Result;
use std::collections::BTreeMap;
use std::time::{SystemTime, UNIX_EPOCH};
use error_chain::bail;
use hex::encode as hex_encode;
use hmac::{Hmac, Mac};
use serde_json::Value;
use sha2::Sha256;

/// Percent-encode a query string value following RFC 3986.
///
/// Only unreserved characters (`A-Z a-z 0-9 - _ . ~`) are kept as-is; everything else is
/// encoded as `%XX` (UTF-8 bytes). Binance requires that signed payloads are percent-encoded
/// before signing, and this is also what makes JSON array parameters (`symbols`, `batchOrders`,
/// `orderIdList`) and non-ASCII symbols round-trip correctly.
pub fn percent_encode(value: &str) -> String {
    let mut out = String::with_capacity(value.len());
    for byte in value.bytes() {
        match byte {
            b'A'..=b'Z' | b'a'..=b'z' | b'0'..=b'9' | b'-' | b'_' | b'.' | b'~' => {
                out.push(byte as char);
            }
            _ => out.push_str(&format!("%{:02X}", byte)),
        }
    }
    out
}

/// Build a `key=value&key=value` query string from the given parameters.
///
/// Keys are emitted in alphabetical order (BTreeMap iteration order) which is the order
/// Binance uses for signature payloads; values are percent-encoded.
pub fn build_request(parameters: BTreeMap<String, String>) -> String {
    let mut request = String::new();
    for (key, value) in parameters {
        let param = format!("{}={}&", key, percent_encode(&value));
        request.push_str(param.as_ref());
    }
    request.pop();
    request
}

pub fn build_signed_request(
    parameters: BTreeMap<String, String>, recv_window: u64,
) -> Result<String> {
    build_signed_request_custom(parameters, recv_window, SystemTime::now())
}

pub fn build_signed_request_custom(
    mut parameters: BTreeMap<String, String>, recv_window: u64, start: SystemTime,
) -> Result<String> {
    if recv_window > 0 {
        parameters.insert("recvWindow".into(), recv_window.to_string());
    }
    if let Ok(timestamp) = get_timestamp(start) {
        parameters.insert("timestamp".into(), timestamp.to_string());
        return Ok(build_request(parameters));
    }
    bail!("Failed to get timestamp")
}

/// Compute the hex-encoded HMAC-SHA256 signature of `payload` using `secret_key`.
pub fn sign_hmac(secret_key: &str, payload: &str) -> String {
    let mut mac = Hmac::<Sha256>::new_from_slice(secret_key.as_bytes())
        .expect("HMAC accepts keys of any length");
    mac.update(payload.as_bytes());
    hex_encode(mac.finalize().into_bytes())
}

/// Encode a list of strings as the JSON array format Binance expects for
/// parameters such as `symbols` or `origClientOrderIdList`: `["A","B"]`.
pub fn json_string_array<S: AsRef<str>>(items: &[S]) -> String {
    let values: Vec<Value> = items
        .iter()
        .map(|s| Value::String(s.as_ref().to_string()))
        .collect();
    Value::Array(values).to_string()
}

pub fn to_i64(v: &Value) -> i64 {
    v.as_i64().unwrap()
}

pub fn to_f64(v: &Value) -> f64 {
    v.as_str().unwrap().parse().unwrap()
}

/// Current Unix time in milliseconds.
pub fn get_timestamp_now() -> Result<u64> {
    get_timestamp(SystemTime::now())
}

fn get_timestamp(start: SystemTime) -> Result<u64> {
    let since_epoch = start.duration_since(UNIX_EPOCH)?;
    Ok(since_epoch.as_secs() * 1000 + u64::from(since_epoch.subsec_nanos()) / 1_000_000)
}

pub fn is_start_time_valid(start_time: &u64) -> bool {
    let current_time = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs();

    start_time <= &current_time
}

pub fn generate_uuid22() -> String {
    use uuid::Uuid;
    let uuid = Uuid::new_v4();
    let uuid_str = uuid.to_string().replace('-', "");
    uuid_str[..22].to_string()
}

pub fn uuid_spot() -> String {
    format!("x-HNA2TXFJ{}", generate_uuid22())
}

pub fn uuid_futures() -> String {
    format!("x-Cb7ytekJ{}", generate_uuid22())
}
