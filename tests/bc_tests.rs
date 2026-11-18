use chrono::{TimeZone, Utc};
use sxwnl4rust::eph::eph_base::J2000;
use sxwnl4rust::eph::jd::JD;
use sxwnl4rust::eph::msc::ec_fast;
use sxwnl4rust::eph::szj::SZJ;
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

/// Values below were produced by the original 寿星天文历 web calendar for the
/// same dates, so a regression here means a divergence from the reference.
#[test]
fn bc_dates_match_the_reference() {
    let cases = [
        (-2069, 6, 1, "三", "廿五", "", "辛卯", "癸巳", "庚子", ""),
        (-1600, 6, 1, "四", "廿一", "", "庚辰", "辛巳", "癸卯", ""),
        (-1000, 6, 1, "四", "廿五", "小满", "庚辰", "辛巳", "癸酉", ""),
        (-722, 7, 1, "五", "初九", "夏至", "戊午", "戊午", "壬戌", "上弦"),
        (-500, 6, 1, "七", "初三", "", "庚子", "辛巳", "戊午", ""),
        (-221, 6, 1, "六", "廿七", "", "己卯", "己巳", "壬午", ""),
        (-104, 6, 1, "四", "三十", "", "丙子", "癸巳", "丁酉", "朔"),
        (-1, 6, 1, "四", "廿八", "", "己未", "己巳", "丁酉", ""),
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

/// Beijing sun and moon times, again from the reference calendar.
#[test]
fn bc_sky_matches_the_reference() {
    let cases = [(-500, 6, 1, "04:45", "19:27", "06:35", "21:28")];
    for (y, m, d, sunrise, sunset, moonrise, moonset) in cases {
        let jd = JD::default().jd(y, m, d as f64) - J2000;
        let shift = 8.0 / 24.0;
        let mut site = SZJ::new();
        site.l = 116.4074f64.to_radians();
        site.fa = 39.9042f64.to_radians();
        let sun = site.st(jd + shift);
        let moon = site.mt(jd + shift);
        let clock = |v: f64| JD::default().time_str(v + shift);
        assert_eq!(clock(sun.s), sunrise);
        assert_eq!(clock(sun.j), sunset);
        assert_eq!(clock(moon.s), moonrise);
        assert_eq!(clock(moon.j), moonset);
    }
}

#[test]
fn bc_solar_eclipse_matches_the_reference() {
    // 585 BC, the eclipse of Thales.
    assert_eq!(ec_fast(JD::default().jd(-584, 5, 28.0) - J2000).lx, "T");
    // Two ordinary new moons that carry no eclipse.
    assert_eq!(ec_fast(JD::default().jd(-200, 3, 10.0) - J2000).lx, "N");
}
