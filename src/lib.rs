use std::time::{Duration, SystemTime, UNIX_EPOCH};

pub const THIRTY_SECONDS_MS: u128 = 1000 * 30;
pub const ONE_MIN_MS: u128 = 1000 * 60;
pub const FIVE_MIN_MS: u128 = ONE_MIN_MS * 5;
pub const TEN_MIN_MS: u128 = FIVE_MIN_MS * 2;
pub const FIFTEEN_MIN_MS: u128 = FIVE_MIN_MS * 3;
pub const THIRTY_MIN_MS: u128 = FIFTEEN_MIN_MS * 2;
pub const ONE_HOUR_MS: u128 = THIRTY_MIN_MS * 3;
pub const DAY_MS: u128 = ONE_HOUR_MS * 24;

pub fn unix_timestamp_ms() -> u128 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis()
}

pub async fn wait_until(target_ts_ms: u128) -> u128 {
    let ts = unix_timestamp_ms();
    if target_ts_ms < ts {
        panic!("provided target ts is before the current time");
    }
    let time_to_wait = target_ts_ms - ts;
    tokio::time::sleep(Duration::from_millis(time_to_wait as u64)).await;
    ts + time_to_wait
}

pub async fn wait_until_next_30_sec(additional_ms: u128) -> u128 {
    let ts = unix_timestamp_ms();
    let time_to_wait = THIRTY_SECONDS_MS - ts % THIRTY_SECONDS_MS + additional_ms;
    tokio::time::sleep(Duration::from_millis(time_to_wait as u64)).await;
    ts + time_to_wait
}

pub fn last_30_sec_interval() -> (u128, u128) {
    let ts = unix_timestamp_ms();
    let end = ts - ts % THIRTY_SECONDS_MS;
    (end - THIRTY_SECONDS_MS, end)
}

pub async fn wait_until_next_1_min(additional_ms: u128) -> u128 {
    let ts = unix_timestamp_ms();
    let time_to_wait = ONE_MIN_MS - ts % ONE_MIN_MS + additional_ms;
    tokio::time::sleep(Duration::from_millis(time_to_wait as u64)).await;
    ts + time_to_wait
}

pub fn last_1_min_interval() -> (u128, u128) {
    let ts = unix_timestamp_ms();
    let end = ts - ts % ONE_MIN_MS;
    (end - ONE_MIN_MS, end)
}

pub async fn wait_until_next_5_min(additional_ms: u128) -> u128 {
    let ts = unix_timestamp_ms();
    let time_to_wait = FIVE_MIN_MS - ts % FIVE_MIN_MS + additional_ms;
    tokio::time::sleep(Duration::from_millis(time_to_wait as u64)).await;
    ts + time_to_wait
}

pub fn last_5_min_interval() -> (u128, u128) {
    let ts = unix_timestamp_ms();
    let end = ts - ts % FIVE_MIN_MS;
    (end - FIVE_MIN_MS, end)
}

pub async fn wait_until_next_10_min(additional_ms: u128) -> u128 {
    let ts = unix_timestamp_ms();
    let time_to_wait = TEN_MIN_MS - ts % TEN_MIN_MS + additional_ms;
    tokio::time::sleep(Duration::from_millis(time_to_wait as u64)).await;
    ts + time_to_wait
}

pub fn last_10_min_interval() -> (u128, u128) {
    let ts = unix_timestamp_ms();
    let end = ts - ts % TEN_MIN_MS;
    (end - TEN_MIN_MS, end)
}

pub async fn wait_until_next_15_min(additional_ms: u128) -> u128 {
    let ts = unix_timestamp_ms();
    let time_to_wait = FIFTEEN_MIN_MS - ts % FIFTEEN_MIN_MS + additional_ms;
    tokio::time::sleep(Duration::from_millis(time_to_wait as u64)).await;
    ts + time_to_wait
}

pub fn last_15_min_interval() -> (u128, u128) {
    let ts = unix_timestamp_ms();
    let end = ts - ts % FIFTEEN_MIN_MS;
    (end - FIFTEEN_MIN_MS, end)
}

pub async fn wait_until_next_30_min(additional_ms: u128) -> u128 {
    let ts = unix_timestamp_ms();
    let time_to_wait = THIRTY_MIN_MS - ts % THIRTY_MIN_MS + additional_ms;
    tokio::time::sleep(Duration::from_millis(time_to_wait as u64)).await;
    ts + time_to_wait
}

pub fn last_30_min_interval() -> (u128, u128) {
    let ts = unix_timestamp_ms();
    let end = ts - ts % THIRTY_MIN_MS;
    (end - THIRTY_MIN_MS, end)
}

pub async fn wait_until_next_hour(additional_ms: u128) -> u128 {
    let ts = unix_timestamp_ms();
    let time_to_wait = ONE_HOUR_MS - ts % ONE_HOUR_MS + additional_ms;
    tokio::time::sleep(Duration::from_millis(time_to_wait as u64)).await;
    ts + time_to_wait
}

pub fn last_hour_interval() -> (u128, u128) {
    let ts = unix_timestamp_ms();
    let end = ts - ts % ONE_HOUR_MS;
    (end - ONE_HOUR_MS, end)
}