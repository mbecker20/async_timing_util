use std::time::{Duration, SystemTime, UNIX_EPOCH};
use serde_derive::{Serialize, Deserialize};
use strum_macros::{Display, EnumString};

#[derive(Serialize, Deserialize, Debug, Display, EnumString, PartialEq, Hash, Eq, Clone, Copy)]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
pub enum Timelength {
    #[serde(rename = "1-sec")]
    OneSecond,
    #[serde(rename = "5-sec")]
    FiveSeconds,
    #[serde(rename = "10-sec")]
    TenSeconds,
    #[serde(rename = "30-sec")]
    ThirtySeconds,
    #[serde(rename = "1-min")]
    OneMinute,
    #[serde(rename = "2-min")]
    TwoMinutes,
    #[serde(rename = "5-min")]
    FiveMinutes,
    #[serde(rename = "10-min")]
    TenMinutes,
    #[serde(rename = "15-min")]
    FifteenMinutes,
    #[serde(rename = "30-min")]
    ThirtyMinutes,
    #[serde(rename = "1-hr")]
    OneHour,
    #[serde(rename = "2-hr")]
    TwoHours,
    #[serde(rename = "6-hr")]
    SixHours,
    #[serde(rename = "8-hr")]
    EightHours,
    #[serde(rename = "12-hr")]
    TwelveHours,
    #[serde(rename = "1-day")]
    OneDay,
    #[serde(rename = "3-day")]
    ThreeDay,
    #[serde(rename = "1-wk")]
    OneWeek,
    #[serde(rename = "2-wk")]
    TwoWeeks,
    #[serde(rename = "30-day")]
    ThirtyDays,
}

pub const ONE_SECOND_MS: u128 = 1000;
pub const FIVE_SECONDS_MS: u128 = 1000 * 5;
pub const TEN_SECONDS_MS: u128 = 1000 * 10;
pub const THIRTY_SECONDS_MS: u128 = 1000 * 30;
pub const ONE_MIN_MS: u128 = 1000 * 60;
pub const TWO_MIN_MS: u128 = 1000 * 120;
pub const FIVE_MIN_MS: u128 = ONE_MIN_MS * 5;
pub const TEN_MIN_MS: u128 = ONE_MIN_MS * 10;
pub const FIFTEEN_MIN_MS: u128 = ONE_MIN_MS * 15;
pub const THIRTY_MIN_MS: u128 = ONE_MIN_MS * 30;
pub const ONE_HOUR_MS: u128 = ONE_MIN_MS * 60;
pub const TWO_HOUR_MS: u128 = ONE_HOUR_MS * 2;
pub const SIX_HOURS_MS: u128 = ONE_HOUR_MS * 6;
pub const EIGHT_HOURS_MS: u128 = ONE_HOUR_MS * 8;
pub const TWELVE_HOURS_MS: u128 = ONE_HOUR_MS * 12;
pub const ONE_DAY_MS: u128 = ONE_HOUR_MS * 24;
pub const THREE_DAY_MS: u128 = ONE_DAY_MS * 3;
pub const ONE_WEEK_MS: u128 = ONE_DAY_MS * 7;
pub const TWO_WEEKS_MS: u128 = ONE_DAY_MS * 14;
pub const THIRTY_DAYS_MS: u128 = ONE_DAY_MS * 30;

pub fn get_timelength_in_ms(timelength: Timelength) -> u128 {
    match timelength {
        Timelength::OneSecond => ONE_SECOND_MS,
        Timelength::FiveSeconds => FIVE_SECONDS_MS,
        Timelength::TenSeconds => TEN_SECONDS_MS,
        Timelength::ThirtySeconds => THIRTY_SECONDS_MS,
        Timelength::OneMinute => ONE_MIN_MS,
        Timelength::TwoMinutes => TWO_MIN_MS,
        Timelength::FiveMinutes => FIVE_MIN_MS,
        Timelength::TenMinutes => TEN_MIN_MS,
        Timelength::FifteenMinutes => FIFTEEN_MIN_MS,
        Timelength::ThirtyMinutes => THIRTY_MIN_MS,
        Timelength::OneHour => ONE_HOUR_MS,
        Timelength::TwoHours => TWO_HOUR_MS,
        Timelength::SixHours => SIX_HOURS_MS,
        Timelength::EightHours => EIGHT_HOURS_MS,
        Timelength::TwelveHours => TWELVE_HOURS_MS,
        Timelength::OneDay => ONE_DAY_MS,
        Timelength::ThreeDay => THREE_DAY_MS,
        Timelength::OneWeek => ONE_WEEK_MS,
        Timelength::TwoWeeks => TWO_WEEKS_MS,
        Timelength::ThirtyDays => THIRTY_DAYS_MS,
    }
}

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

pub async fn wait_until_timelength(timelength: Timelength, additional_ms: u128) -> u128 {
    let ts = unix_timestamp_ms();
    let timelength = get_timelength_in_ms(timelength);
    let time_to_wait = timelength - ts % timelength + additional_ms;
    tokio::time::sleep(Duration::from_millis(time_to_wait as u64)).await;
    ts + time_to_wait
}

pub fn last_timelength_interval(timelength: Timelength) -> (u128, u128) {
    let ts = unix_timestamp_ms();
    let timelength = get_timelength_in_ms(timelength);
    let end = ts - ts % timelength;
    (end - timelength, end)
}