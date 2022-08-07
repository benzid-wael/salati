use std::f64::consts::PI;
use std::ops::{Add, Div, Mul, Sub};

use crate::astronomy::ops;
use chrono::{DateTime, Datelike, Duration, TimeZone, Timelike};

pub trait Normalize {
    fn normalized_to_scale(&self, max: f64) -> f64;
}

impl Normalize for f64 {
    fn normalized_to_scale(&self, max: f64) -> f64 {
        self - (max * (self / max).floor())
    }
}

/// Convenience methods for the DateTime type.
pub trait Stride {
    fn tomorrow(&self) -> Self;
    fn yesterday(&self) -> Self;
    fn julian_day(&self) -> f64;
    fn nearest_minute(&self) -> Self;
    fn adjust_time(&self, minutes: i64) -> Self;
    fn next_date(&self, fwd: bool) -> Self;
}

impl<Tz: TimeZone> Stride for DateTime<Tz> {
    /// Returns the date/time for tomorrow.
    fn tomorrow(&self) -> Self {
        self.next_date(true)
    }

    /// Returns the date/time for yesterday.
    fn yesterday(&self) -> Self {
        self.next_date(false)
    }

    /// Returns the Julian day.
    fn julian_day(&self) -> f64 {
        ops::julian_day(
            self.year() as i32,
            self.month() as i32,
            self.day() as i32,
            0.0,
        )
    }

    fn nearest_minute(&self) -> Self {
        let adjusted = self.clone();
        let seconds = adjusted.second() as i64;

        if adjusted.second() >= 30 {
            adjusted + Duration::seconds(60 - seconds)
        } else {
            adjusted + Duration::seconds(-seconds)
        }
    }

    fn adjust_time(&self, minutes: i64) -> Self {
        let some_date = self.clone();
        some_date
            .checked_add_signed(Duration::seconds(minutes * 60))
            .unwrap()
    }

    fn next_date(&self, fwd: bool) -> Self {
        let ordinal = if fwd {
            self.ordinal() + 1
        } else {
            self.ordinal() - 1
        };

        match self.with_ordinal(ordinal) {
            Some(dt) => dt,
            None => {
                if fwd {
                    self.with_year(self.year() + 1)
                        .unwrap()
                        .with_ordinal(1) // 1st january
                        .unwrap()
                } else {
                    self.with_year(self.year() - 1)
                        .unwrap()
                        .with_month(12)
                        .unwrap()
                        .with_day(31)
                        .unwrap()
                }
            }
        }
    }
}

#[derive(PartialEq, Debug, Copy, Clone)]
pub struct Angle {
    pub degrees: f64,
}

impl Angle {
    pub fn new(value: f64) -> Self {
        Angle { degrees: value }
    }

    pub fn from_radians(value: f64) -> Self {
        Angle {
            degrees: (value * 180.0) / PI,
        }
    }

    pub fn radians(&self) -> f64 {
        (self.degrees * PI) / 180.0
    }

    pub fn unwound(&self) -> Angle {
        Angle {
            degrees: self.degrees.normalized_to_scale(360.0),
        }
    }

    pub fn quadrant_shifted(&self) -> Angle {
        if self.degrees >= -180.0 && self.degrees <= 180.0 {
            // Nothing to do. Already initialized
            // to the default value.
            *self
        } else {
            let value = self.degrees - (360.0 * (self.degrees / 360.0).round());
            Angle { degrees: value }
        }
    }
}

impl Add for Angle {
    type Output = Angle;

    fn add(self, rhs: Angle) -> Angle {
        Angle {
            degrees: self.degrees + rhs.degrees,
        }
    }
}

impl Sub for Angle {
    type Output = Angle;

    fn sub(self, rhs: Angle) -> Angle {
        Angle {
            degrees: self.degrees - rhs.degrees,
        }
    }
}

impl Mul for Angle {
    type Output = Angle;

    fn mul(self, rhs: Angle) -> Angle {
        Angle {
            degrees: self.degrees * rhs.degrees,
        }
    }
}

impl Div for Angle {
    type Output = Angle;

    fn div(self, rhs: Angle) -> Angle {
        if rhs.degrees == 0.0 {
            panic!("Cannot divide by zero.");
        }

        Angle {
            degrees: self.degrees / rhs.degrees,
        }
    }
}

/// The latitude and longitude associated with a location.
/// Both latiude and longitude values are specified in degrees.
#[derive(PartialEq, Debug, Copy, Clone)]
pub struct Coordinates {
    pub latitude: f64,
    pub longitude: f64,
}

impl Coordinates {
    pub fn new(latitude: f64, longitude: f64) -> Self {
        Coordinates {
            latitude,
            longitude,
        }
    }
}

impl Coordinates {
    pub fn latitude_angle(&self) -> Angle {
        Angle::new(self.latitude)
    }

    pub fn longitude_angle(&self) -> Angle {
        Angle::new(self.longitude)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Utc;
    use std::f64::consts::PI;

    #[test]
    fn angle_conversion_from_radians() {
        assert_eq!(Angle::from_radians(PI).degrees, 180.0);
        assert_eq!(Angle::from_radians(PI / 2.0).degrees, 90.0);
    }

    #[test]
    fn angle_conversion_degrees_to_radians() {
        assert_eq!(Angle::new(180.0).radians(), PI);
        assert_eq!(Angle::new(90.0).radians(), PI / 2.0);
    }

    #[test]
    fn normalize_value() {
        assert_eq!(2.0_f64.normalized_to_scale(-5.0), -3.0);
        assert_eq!((-4.0_f64).normalized_to_scale(-5.0), -4.0);
        assert_eq!((-6.0_f64).normalized_to_scale(-5.0), -1.0);

        assert_eq!((-1.0_f64).normalized_to_scale(24.0), 23.0);
        assert_eq!(1.0_f64.normalized_to_scale(24.0), 1.0);
        assert_eq!(49.0_f64.normalized_to_scale(24.0), 1.0);

        assert_eq!(361.0_f64.normalized_to_scale(360.0), 1.0);
        assert_eq!(360.0_f64.normalized_to_scale(360.0), 0.0);
        assert_eq!(259.0_f64.normalized_to_scale(360.0), 259.0);
        assert_eq!(2592.0_f64.normalized_to_scale(360.0), 72.0);
    }

    #[test]
    fn angle_unwound() {
        assert_eq!(Angle::new(-45.0).unwound().degrees, 315.0);
        assert_eq!(Angle::new(361.0).unwound().degrees, 1.0);
        assert_eq!(Angle::new(360.0).unwound().degrees, 0.0);
        assert_eq!(Angle::new(259.0).unwound().degrees, 259.0);
        assert_eq!(Angle::new(2592.0).unwound().degrees, 72.0);
    }

    #[test]
    fn closest_angle() {
        assert_eq!(Angle::new(360.0).quadrant_shifted().degrees, 0.0);
        assert_eq!(Angle::new(361.0).quadrant_shifted().degrees, 1.0);
        assert_eq!(Angle::new(1.0).quadrant_shifted().degrees, 1.0);
        assert_eq!(Angle::new(-1.0).quadrant_shifted().degrees, -1.0);
        assert_eq!(Angle::new(-181.0).quadrant_shifted().degrees, 179.0);
        assert_eq!(Angle::new(180.0).quadrant_shifted().degrees, 180.0);
        assert_eq!(Angle::new(359.0).quadrant_shifted().degrees, -1.0);
        assert_eq!(Angle::new(-359.0).quadrant_shifted().degrees, 1.0);
        assert_eq!(Angle::new(1261.0).quadrant_shifted().degrees, -179.0);
    }

    #[test]
    fn adding_angles() {
        let angle_a = Angle::new(45.0);
        let angle_b = Angle::new(45.0);

        assert_eq!((angle_a + angle_b).degrees, 90.0)
    }

    #[test]
    fn calculate_nearest_minute() {
        let time_1 = Utc.ymd(2015, 7, 13).and_hms(4, 37, 30);
        let time_2 = Utc.ymd(2015, 7, 13).and_hms(5, 59, 20);

        assert_eq!(
            time_1.nearest_minute(),
            Utc.ymd(2015, 7, 13).and_hms(4, 38, 00)
        );
        assert_eq!(
            time_2.nearest_minute(),
            Utc.ymd(2015, 7, 13).and_hms(5, 59, 00)
        );
    }

    macro_rules! tomorrow_tests {
        ($($name:ident: $value:expr,)*) => {
        $(
            #[test]
            fn $name() {
                let (input, expected) = $value;
                assert_eq!(expected.and_hms(0, 0, 0), input.and_hms(0, 0, 0).tomorrow());
            }
        )*
        }
    }

    tomorrow_tests! {
        test_tomorrow_1: (Utc.ymd(2015, 1, 15), Utc.ymd(2015, 1, 16)),
        test_tomorrow_2: (Utc.ymd(1995, 3, 3), Utc.ymd(1995, 3, 4)),
        test_tomorrow_last_day_month: (Utc.ymd(2015, 1, 31), Utc.ymd(2015, 2, 1)),
        test_tomorrow_last_day_feb_1: (Utc.ymd(2015, 2, 28), Utc.ymd(2015, 3, 1)),
        test_tomorrow_last_day_feb_2: (Utc.ymd(1500, 2, 28), Utc.ymd(1500, 3, 1)),
        test_tomorrow_last_last_day_feb_leap: (Utc.ymd(2000, 2, 28), Utc.ymd(2000, 2, 29)),
        test_tomorrow_last_day_feb_leap: (Utc.ymd(2000, 2, 29), Utc.ymd(2000, 3, 1)),
        test_tomorrow_last_day_in_year: (Utc.ymd(2006, 12, 31), Utc.ymd(2007, 1, 1)),
    }

    macro_rules! yesterday_tests {
        ($($name:ident: $value:expr,)*) => {
        $(
            #[test]
            fn $name() {
                let (input, expected) = $value;
                assert_eq!(expected.and_hms(0, 0, 0), input.and_hms(0, 0, 0).yesterday());
            }
        )*
        }
    }

    yesterday_tests! {
        test_yesterday_regular: (Utc.ymd(2015, 1, 15), Utc.ymd(2015, 1, 14)),
        test_yesterday_last_day_month: (Utc.ymd(2015, 1, 31), Utc.ymd(2015, 1, 30)),
        test_yesterday_first_day_month: (Utc.ymd(2015, 5, 1), Utc.ymd(2015, 4, 30)),
        test_yesterday_feb: (Utc.ymd(2015, 2, 28), Utc.ymd(2015, 2, 27)),
        test_yesterday_march: (Utc.ymd(2015, 3, 1), Utc.ymd(2015, 2, 28)),
        test_yesterday_march_leap_year: (Utc.ymd(2000, 3, 1), Utc.ymd(2000, 2, 29)),
        test_yesterday_first_day_year_1: (Utc.ymd(2000, 1, 1), Utc.ymd(1999, 12, 31)),
        test_yesterday_first_day_year_2: (Utc.ymd(1500, 1, 1), Utc.ymd(1499, 12, 31)),
        test_yesterday_first_day_year_3: (Utc.ymd(1501, 1, 1), Utc.ymd(1500, 12, 31)),
    }
}
