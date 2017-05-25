extern crate chrono;

use chrono::{DateTime, Duration, UTC};


const GIGASECOND: i64 = 1_000_000_000;


pub fn after(start_date: DateTime<UTC>) -> DateTime<UTC> {
    let duration = Duration::seconds(GIGASECOND);
    start_date + (duration)
}
