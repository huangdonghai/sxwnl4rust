use chrono::{TimeZone, Utc};
use sxwnl4rust::lunar::lunar::LunarCalendar;

fn day(y: i32, m: u32, d: u32) -> sxwnl4rust::lunar::lunar::LunarDay {
    let at = Utc.with_ymd_and_hms(y, m, d, 12, 0, 0).single().unwrap();
    LunarCalendar::default().get_day_info(&at, 120.0)
}

/// Every year in the common era must produce a lunar date without panicking.
/// Before the table fixes this failed for 512 of 2100 years.
#[test]
fn every_common_era_year_resolves() {
    let mut failures = Vec::new();
    for year in 1..=2100 {
        for (month, day_of_month) in [(1u32, 1u32), (6, 1), (12, 31)] {
            let ok = std::panic::catch_unwind(|| {
                let value = day(year, month, day_of_month);
                !value.lunar_day_name.is_empty()
            });
            if !matches!(ok, Ok(true)) {
                failures.push(year);
                break;
            }
        }
    }
    assert!(failures.is_empty(), "years without a lunar date: {failures:?}");
}

/// Known civil-to-lunar dates across the range the historical tables cover.
#[test]
fn historical_dates_match_the_almanac() {
    let cases = [
        (1900, 1, 31, "正", "初一"),
        (1912, 2, 18, "正", "初一"),
        (1930, 1, 30, "正", "初一"),
        (1949, 1, 29, "正", "初一"),
        (1950, 2, 17, "正", "初一"),
        (1955, 1, 24, "正", "初一"),
        (1949, 10, 1, "八", "初十"),
        (2024, 2, 10, "正", "初一"),
        (2020, 5, 23, "四", "初一"),
        (2023, 3, 22, "二", "初一"),
        (2024, 2, 9, "十二", "三十"),
    ];
    for (year, month, date, want_month, want_day) in cases {
        let value = day(year, month, date);
        assert_eq!(value.lunar_month_name, want_month, "{year}-{month}-{date}");
        assert_eq!(value.lunar_day_name, want_day, "{year}-{month}-{date}");
    }
    assert!(day(2020, 5, 23).is_leap_month);
    assert!(day(2023, 3, 22).is_leap_month);
    assert!(!day(2024, 3, 22).is_leap_month);
}
