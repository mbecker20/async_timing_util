# Async Timing Util

Convenient utilities for doing repeated tasks at precise intervals.

```rust
use async_timing_util::{Timelength, wait_until_timelength};

loop {
    let ts = wait_until_timelength(Timelength::OneHour, 0).await;
    /// Do something async every hour, on the hour.
		/// Runs at 00:00, 01:00, 02:00, etc.
}
```