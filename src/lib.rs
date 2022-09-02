use std::time::{Duration, SystemTime, UNIX_EPOCH};

pub const ONE_MIN_MS: i64 = 1000 * 60;
pub const FIVE_MIN_MS: i64 = ONE_MIN_MS * 5;
pub const TEN_MIN_MS: i64 = FIVE_MIN_MS * 2;
pub const FIFTEEN_MIN_MS: i64 = FIVE_MIN_MS * 3;
pub const THIRTY_MIN_MS: i64 = FIFTEEN_MIN_MS * 2;
pub const ONE_HOUR_MS: i64 = THIRTY_MIN_MS * 3;
pub const DAY_MS: i64 = ONE_HOUR_MS * 24;

pub fn unix_timestamp_ms() -> i64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis() as i64
}

pub async fn wait_until(target_ts_ms: i64) {
    let ts = unix_timestamp_ms();
    if target_ts_ms < ts {
        panic!("provided target ts is before the current time");
    }
    let time_to_wait = target_ts_ms - ts;
    tokio::time::sleep(Duration::from_millis(time_to_wait as u64)).await;
}

pub async fn wait_until_next_1_min() {
    let ts = unix_timestamp_ms();
    let time_to_wait = ONE_MIN_MS - ts % ONE_MIN_MS + 1000;
    tokio::time::sleep(Duration::from_millis(time_to_wait as u64)).await;
}

pub fn last_1_min_interval() -> (i64, i64) {
    let ts = unix_timestamp_ms();
    let end = ts - ts % ONE_MIN_MS;
    (end - ONE_MIN_MS, end)
}

pub async fn wait_until_next_5_min() {
    let ts = unix_timestamp_ms();
    let time_to_wait = FIVE_MIN_MS - ts % FIVE_MIN_MS + 1000;
    tokio::time::sleep(Duration::from_millis(time_to_wait as u64)).await;
}

pub fn last_five_min_interval() -> (i64, i64) {
    let ts = unix_timestamp_ms();
    let end = ts - ts % FIVE_MIN_MS;
    (end - FIVE_MIN_MS, end)
}

pub async fn wait_until_next_10_min() {
    let ts = unix_timestamp_ms();
    let time_to_wait = TEN_MIN_MS - ts % TEN_MIN_MS + 1000;
    tokio::time::sleep(Duration::from_millis(time_to_wait as u64)).await;
}

pub fn last_ten_min_interval() -> (i64, i64) {
    let ts = unix_timestamp_ms();
    let end = ts - ts % TEN_MIN_MS;
    (end - TEN_MIN_MS, end)
}

pub async fn wait_until_next_15_min() {
    let ts = unix_timestamp_ms();
    let time_to_wait = FIFTEEN_MIN_MS - ts % FIFTEEN_MIN_MS + 1000;
    tokio::time::sleep(Duration::from_millis(time_to_wait as u64)).await;
}

pub fn last_fifteen_min_interval() -> (i64, i64) {
    let ts = unix_timestamp_ms();
    let end = ts - ts % FIFTEEN_MIN_MS;
    (end - FIFTEEN_MIN_MS, end)
}

pub async fn wait_until_next_30_min() {
    let ts = unix_timestamp_ms();
    let time_to_wait = THIRTY_MIN_MS - ts % THIRTY_MIN_MS + 1000;
    tokio::time::sleep(Duration::from_millis(time_to_wait as u64)).await;
}

pub fn last_thirty_min_interval() -> (i64, i64) {
    let ts = unix_timestamp_ms();
    let end = ts - ts % THIRTY_MIN_MS;
    (end - THIRTY_MIN_MS, end)
}

pub async fn wait_until_next_hour() {
    let ts = unix_timestamp_ms();
    let time_to_wait = ONE_HOUR_MS - ts % ONE_HOUR_MS + 1000;
    tokio::time::sleep(Duration::from_millis(time_to_wait as u64)).await;
}

pub fn last_hour_interval() -> (i64, i64) {
    let ts = unix_timestamp_ms();
    let end = ts - ts % ONE_HOUR_MS;
    (end - ONE_HOUR_MS, end)
}