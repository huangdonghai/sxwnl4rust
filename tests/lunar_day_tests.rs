use chrono::{TimeZone, Utc};
use sxwnl4rust::lunar::lunar::{LunarCalendar, LunarDay};

fn day(y: i32, m: u32, d: u32) -> LunarDay {
    let cal = LunarCalendar::default();
    let at = Utc.with_ymd_and_hms(y, m, d, 12, 0, 0).unwrap();
    cal.get_day_info(&at, 120.0)
}

#[test]
fn spring_festival_is_the_first_day_of_the_first_month() {
    for (y, m, d) in [(2024, 2, 10), (2023, 1, 22), (2020, 1, 25)] {
        let got = day(y, m, d);
        assert_eq!(got.lunar_month_name, "正", "{y}-{m}-{d}");
        assert_eq!(got.lunar_day_name, "初一", "{y}-{m}-{d}");
        assert!(!got.is_leap_month, "{y}-{m}-{d}");
    }
}

#[test]
fn the_day_before_is_the_last_day_of_the_twelfth_month() {
    for (y, m, d) in [(2024, 2, 9), (2023, 1, 21)] {
        let got = day(y, m, d);
        assert_eq!(got.lunar_month_name, "十二", "{y}-{m}-{d}");
        assert_eq!(got.lunar_day_name, "三十", "{y}-{m}-{d}");
    }
}

#[test]
fn leap_months_are_marked() {
    let leap_second = day(2023, 3, 22);
    assert_eq!(leap_second.lunar_month_name, "二");
    assert_eq!(leap_second.lunar_day_name, "初一");
    assert!(leap_second.is_leap_month);

    let leap_fourth = day(2020, 5, 23);
    assert_eq!(leap_fourth.lunar_month_name, "四");
    assert!(leap_fourth.is_leap_month);

    // An ordinary month never reports itself as leap.
    assert!(!day(2024, 3, 22).is_leap_month);
}

#[test]
fn solar_terms_land_on_the_right_day() {
    assert_eq!(day(2024, 2, 4).jie_qi, "立春");
    assert_eq!(day(2024, 12, 21).jie_qi, "冬至");
    assert_eq!(day(2024, 2, 6).jie_qi, "");
    // 惊蛰 2024 was on 5 March, not 22 March.
    assert_eq!(day(2024, 3, 22).jie_qi, "");
}

#[test]
fn zodiac_and_gan_zhi_follow_their_own_new_year() {
    // 2023-01-22 is the first day of the lunar year, before 立春.
    let day_after_new_year = day(2023, 1, 22);
    assert_eq!(day_after_new_year.sheng_xiao, "兔");
    assert_eq!(day_after_new_year.gan_zhi_year, "壬寅");

    let after_lichun = day(2024, 2, 10);
    assert_eq!(after_lichun.gan_zhi_day, "甲辰");
    assert_eq!(after_lichun.sheng_xiao, "龙");
}
