use chrono::{TimeZone, Utc};
use sxwnl4rust::lunar::lunar::LunarCalendar;

fn day(y: i32, m: u32, d: u32) -> (String, String, bool, String, String, String, String, String) {
    let at = Utc.with_ymd_and_hms(y, m, d, 12, 0, 0).single().unwrap();
    let value = LunarCalendar::default().get_day_info(&at, 120.0);
    (
        value.lunar_month_name,
        value.lunar_day_name,
        value.is_leap_month,
        value.jie_qi,
        value.gan_zhi_year,
        value.gan_zhi_month,
        value.gan_zhi_day,
        value.moon_phase,
    )
}

/// Rows from the original 寿星天文历 web calendar, covering the fitted historical
/// periods (620 onward uses the correction tables), the medieval range that
/// used to panic, and modern dates.
#[test]
fn common_era_dates_match_the_reference() {
    let cases = [
        (620, 8, 20, "七", "十七", "", "庚辰", "甲申", "戊寅", ""),
        (629, 2, 10, "正", "十二", "", "己丑", "丙寅", "甲寅", ""),
        (1279, 8, 20, "七", "十二", "", "己卯", "壬申", "丁巳", ""),
        (1500, 8, 20, "七", "廿六", "", "庚申", "甲申", "戊寅", ""),
        (1900, 2, 10, "正", "十一", "", "庚子", "戊寅", "甲寅", ""),
        (1949, 8, 20, "七", "廿六", "", "己丑", "壬申", "壬午", ""),
        (2024, 2, 10, "正", "初一", "", "甲辰", "丙寅", "甲辰", "朔"),
        (2033, 8, 20, "七", "廿六", "", "癸丑", "庚申", "癸卯", ""),
        (2099, 2, 10, "正", "廿一", "", "己未", "丙寅", "戊寅", ""),
    ];
    for (y, m, d, month, day_name, term, gz_year, gz_month, gz_day, phase) in cases {
        let got = day(y, m, d);
        assert_eq!(got.0, month, "{y}-{m}-{d} month");
        assert_eq!(got.1, day_name, "{y}-{m}-{d} day");
        assert!(!got.2, "{y}-{m}-{d} leap");
        assert_eq!(got.3, term, "{y}-{m}-{d} term");
        assert_eq!(got.4, gz_year, "{y}-{m}-{d} ganzhi year");
        assert_eq!(got.5, gz_month, "{y}-{m}-{d} ganzhi month");
        assert_eq!(got.6, gz_day, "{y}-{m}-{d} ganzhi day");
        assert_eq!(got.7, phase, "{y}-{m}-{d} phase");
    }
}
