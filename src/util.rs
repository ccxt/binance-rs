use crate::errors::{Result, SdkError};
use std::collections::BTreeMap;
use std::time::{SystemTime, UNIX_EPOCH};
use serde_json::Value;

pub fn build_request(parameters: BTreeMap<String, String>) -> String {
    let mut request = String::new();
    for (key, value) in parameters {
        let param = format!("{}={}&", key, value);
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
    Err(SdkError::Other("Failed to get timestamp".into()))
}

pub fn to_i64(v: &Value) -> i64 {
    v.as_i64().unwrap()
}

pub fn to_f64(v: &Value) -> f64 {
    v.as_str().unwrap().parse().unwrap()
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
