use sxwnl4rust::eph::eph_base::J2000;
use sxwnl4rust::eph::jd::JD;
use sxwnl4rust::eph::msc::{ec_fast, YsPL};

fn near(y: i32, m: i32, d: i32) -> f64 {
    JD::default().jd(y, m, d as f64) - J2000 + 0.5
}

#[test]
fn total_lunar_eclipses_match_the_almanac() {
    for (y, m, d, min_magnitude) in [(2025, 9, 7, 1.30), (2026, 3, 3, 1.10), (2022, 11, 8, 1.30)] {
        let mut eclipse = YsPL::new();
        eclipse.lec_max(near(y, m, d));
        assert_eq!(eclipse.lx, "全", "{y}-{m}-{d}");
        assert!(
            eclipse.sf >= min_magnitude,
            "{y}-{m}-{d} magnitude {}",
            eclipse.sf
        );
        // 全食顺序: 初亏 < 食既 < 食甚 < 生光 < 复圆
        let (first, max, last, enter, exit) = (
            eclipse.lt[0],
            eclipse.lt[1],
            eclipse.lt[2],
            eclipse.lt[5],
            eclipse.lt[6],
        );
        assert!(first < enter && enter < max && max < exit && exit < last);
    }
}

#[test]
fn solar_eclipse_types_match_the_almanac() {
    let cases = [
        (2024, 4, 8, "T"),  // North America
        (2026, 8, 12, "T"), // Spain and Iceland
        (2020, 6, 21, "A"), // annular, visible in China
        (2025, 3, 29, "P"), // partial
        (2026, 1, 18, "N"), // no eclipse at this new moon
        (-310, 8, 15, "H"), // hybrid, from the reference calendar
        (-200, 3, 10, "N"),
    ];
    for (y, m, d, want) in cases {
        let result = ec_fast(near(y, m, d));
        assert_eq!(result.lx, want, "{y}-{m}-{d}");
    }
}
