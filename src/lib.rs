//! # Async Timing Util
//!
//! Convenient utilities for doing repeated tasks at precise intervals.
//!
//! ```
//! use async_timing_util::{Timelength, wait_until_timelength};
//!
//! loop {
//!     let ts = wait_until_timelength(Timelength::OneHour).await;
//!     /// Do something async every hour, on the hour
//!     /// Runs at 00:00, 01:00, 02:00, etc.
//! }
//! ```

use serde_derive::{Deserialize, Serialize};
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use strum_macros::{AsRefStr, Display, EnumString, IntoStaticStr};

/// Represents various standard lengths of time.
#[derive(
  Serialize,
  Deserialize,
  Debug,
  Display,
  EnumString,
  AsRefStr,
  IntoStaticStr,
  PartialEq,
  Hash,
  Eq,
  Clone,
  Copy,
)]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
pub enum Timelength {
  #[serde(rename = "1-sec")]
  #[strum(serialize = "1-sec")]
  OneSecond,

  #[serde(rename = "2-sec")]
  #[strum(serialize = "2-sec")]
  TwoSeconds,

  #[serde(rename = "3-sec")]
  #[strum(serialize = "3-sec")]
  ThreeSeconds,

  #[serde(rename = "5-sec")]
  #[strum(serialize = "5-sec")]
  FiveSeconds,

  #[serde(rename = "10-sec")]
  #[strum(serialize = "10-sec")]
  TenSeconds,

  #[serde(rename = "15-sec")]
  #[strum(serialize = "15-sec")]
  FifteenSeconds,

  #[serde(rename = "30-sec")]
  #[strum(serialize = "30-sec")]
  ThirtySeconds,

  #[serde(rename = "1-min")]
  #[strum(serialize = "1-min")]
  OneMinute,

  #[serde(rename = "2-min")]
  #[strum(serialize = "2-min")]
  TwoMinutes,

  #[serde(rename = "3-min")]
  #[strum(serialize = "3-min")]
  ThreeMinutes,

  #[serde(rename = "5-min")]
  #[strum(serialize = "5-min")]
  FiveMinutes,

  #[serde(rename = "10-min")]
  #[strum(serialize = "10-min")]
  TenMinutes,

  #[serde(rename = "15-min")]
  #[strum(serialize = "15-min")]
  FifteenMinutes,

  #[serde(rename = "30-min")]
  #[strum(serialize = "30-min")]
  ThirtyMinutes,

  #[serde(rename = "1-hr")]
  #[strum(serialize = "1-hr")]
  OneHour,

  #[serde(rename = "2-hr")]
  #[strum(serialize = "2-hr")]
  TwoHours,

  #[serde(rename = "3-hr")]
  #[strum(serialize = "3-hr")]
  ThreeHours,

  #[serde(rename = "6-hr")]
  #[strum(serialize = "6-hr")]
  SixHours,

  #[serde(rename = "8-hr")]
  #[strum(serialize = "8-hr")]
  EightHours,

  #[serde(rename = "12-hr")]
  #[strum(serialize = "12-hr")]
  TwelveHours,

  #[serde(rename = "1-day")]
  #[strum(serialize = "1-day")]
  OneDay,

  #[serde(rename = "2-day")]
  #[strum(serialize = "2-day")]
  TwoDays,

  #[serde(rename = "3-day")]
  #[strum(serialize = "3-day")]
  ThreeDays,

  #[serde(rename = "1-wk")]
  #[strum(serialize = "1-wk")]
  OneWeek,

  #[serde(rename = "2-wk")]
  #[strum(serialize = "2-wk")]
  TwoWeeks,

  #[serde(rename = "30-day")]
  #[strum(serialize = "30-day")]
  ThirtyDays,
}

pub const ONE_SECOND_MS: u128 = 1000;
pub const TWO_SECONDS_MS: u128 = 1000 * 2;
pub const THREE_SECONDS_MS: u128 = 1000 * 3;
pub const FIVE_SECONDS_MS: u128 = 1000 * 5;
pub const TEN_SECONDS_MS: u128 = 1000 * 10;
pub const FIFTEEN_SECONDS_MS: u128 = 1000 * 15;
pub const THIRTY_SECONDS_MS: u128 = 1000 * 30;
pub const ONE_MIN_MS: u128 = 1000 * 60;
pub const TWO_MINS_MS: u128 = ONE_MIN_MS * 2;
pub const THREE_MINS_MS: u128 = ONE_MIN_MS * 3;
pub const FIVE_MINS_MS: u128 = ONE_MIN_MS * 5;
pub const TEN_MINS_MS: u128 = ONE_MIN_MS * 10;
pub const FIFTEEN_MINS_MS: u128 = ONE_MIN_MS * 15;
pub const THIRTY_MINS_MS: u128 = ONE_MIN_MS * 30;
pub const ONE_HOUR_MS: u128 = ONE_MIN_MS * 60;
pub const TWO_HOURS_MS: u128 = ONE_HOUR_MS * 2;
pub const THREE_HOURS_MS: u128 = ONE_HOUR_MS * 3;
pub const SIX_HOURS_MS: u128 = ONE_HOUR_MS * 6;
pub const EIGHT_HOURS_MS: u128 = ONE_HOUR_MS * 8;
pub const TWELVE_HOURS_MS: u128 = ONE_HOUR_MS * 12;
pub const ONE_DAY_MS: u128 = ONE_HOUR_MS * 24;
pub const TWO_DAYS_MS: u128 = ONE_DAY_MS * 2;
pub const THREE_DAYS_MS: u128 = ONE_DAY_MS * 3;
pub const ONE_WEEK_MS: u128 = ONE_DAY_MS * 7;
pub const TWO_WEEKS_MS: u128 = ONE_DAY_MS * 14;
pub const THIRTY_DAYS_MS: u128 = ONE_DAY_MS * 30;

/// Converts a timelength into milliseconds.
pub fn get_timelength_in_ms(timelength: Timelength) -> u128 {
  match timelength {
    Timelength::OneSecond => ONE_SECOND_MS,
    Timelength::TwoSeconds => TWO_SECONDS_MS,
    Timelength::ThreeSeconds => THREE_SECONDS_MS,
    Timelength::FiveSeconds => FIVE_SECONDS_MS,
    Timelength::TenSeconds => TEN_SECONDS_MS,
    Timelength::FifteenSeconds => FIFTEEN_SECONDS_MS,
    Timelength::ThirtySeconds => THIRTY_SECONDS_MS,
    Timelength::OneMinute => ONE_MIN_MS,
    Timelength::TwoMinutes => TWO_MINS_MS,
    Timelength::ThreeMinutes => THREE_MINS_MS,
    Timelength::FiveMinutes => FIVE_MINS_MS,
    Timelength::TenMinutes => TEN_MINS_MS,
    Timelength::FifteenMinutes => FIFTEEN_MINS_MS,
    Timelength::ThirtyMinutes => THIRTY_MINS_MS,
    Timelength::OneHour => ONE_HOUR_MS,
    Timelength::TwoHours => TWO_HOURS_MS,
    Timelength::ThreeHours => THREE_HOURS_MS,
    Timelength::SixHours => SIX_HOURS_MS,
    Timelength::EightHours => EIGHT_HOURS_MS,
    Timelength::TwelveHours => TWELVE_HOURS_MS,
    Timelength::OneDay => ONE_DAY_MS,
    Timelength::TwoDays => TWO_DAYS_MS,
    Timelength::ThreeDays => THREE_DAYS_MS,
    Timelength::OneWeek => ONE_WEEK_MS,
    Timelength::TwoWeeks => TWO_WEEKS_MS,
    Timelength::ThirtyDays => THIRTY_DAYS_MS,
  }
}

/// Gets the system time in unix milliseconds.
/// Panics if [SystemTime::duration_since] returns an error.
pub fn unix_timestamp_ms() -> u128 {
  SystemTime::now()
    .duration_since(UNIX_EPOCH)
    .unwrap()
    .as_millis()
}

/// Sleeps until the target unix time. Returns current time if the target time is before the current time.
pub async fn wait_until(target_ts_ms: u128) -> u128 {
  let ts = unix_timestamp_ms();
  if target_ts_ms < ts {
    return ts;
  }
  let time_to_wait = target_ts_ms - ts;
  tokio::time::sleep(Duration::from_millis(time_to_wait as u64)).await;
  ts + time_to_wait
}

/// Waits until the next even instance of the timelength. For example, Timelength::OneHour will wait
/// until the next hour, eg 01:00, 02:00, plus the "additional_ms".
pub async fn wait_until_timelength(timelength: Timelength, additional_ms: u128) -> u128 {
  let ts = unix_timestamp_ms();
  let timelength = get_timelength_in_ms(timelength);
  let time_after = ts % timelength;
  let time_to_wait = if time_after < additional_ms {
    additional_ms - time_after
  } else {
    timelength - time_after + additional_ms
  };
  tokio::time::sleep(Duration::from_millis(time_to_wait as u64)).await;
  ts + time_to_wait
}

/// Gets the most recent complete interval of a given timelength.
///
/// For example, passing Timelength::OneHour and calling this at 03:12 will return
/// the period from (02:00, 03:00).
pub fn last_timelength_interval(timelength: Timelength) -> (u128, u128) {
  let ts = unix_timestamp_ms();
  let timelength = get_timelength_in_ms(timelength);
  let end = ts - ts % timelength;
  (end - timelength, end)
}
