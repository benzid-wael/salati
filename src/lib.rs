// Salati
//
//! Rust library to compute prayer times
//!
//! ## Example
//! ```
//! use salati::prelude::*;
//!
//! let tunis = Coordinates::new(36.8065, 10.1815);
//! let date  = Utc.ymd(2022, 8, 1);
//! let params = ParametersBuilder::with(Method::MuslimWorldLeague, Madhab::Shafi);
//! let prayers = PrayerTimes::new(date, tunis, params);
//!
//! println!("fajr: {}", prayers.fajr.datetime.unwrap());
//! println!("sunrise: {}", prayers.sunrise.datetime.unwrap());
//! println!("dhuhr: {}", prayers.dhuhr.datetime.unwrap());
//! println!("asr: {}", prayers.asr.datetime.unwrap());
//! println!("maghrib: {}", prayers.maghrib.datetime.unwrap());
//! println!("isha: {}", prayers.isha.datetime.unwrap());
//! ```
mod astronomy;
mod constants;
mod models;
mod prayer_times;

pub mod prelude {
    #[doc(no_inline)]
    pub use crate::astronomy::qiblah::Qiblah;
    #[doc(no_inline)]
    pub use crate::astronomy::unit::{Coordinates, Stride};
    #[doc(no_inline)]
    pub use crate::models::adjustments::{TimeAdjustment, TimeAdjustmentBuilder};
    #[doc(no_inline)]
    pub use crate::models::high_latitude_rule::HighLatitudeRule;
    #[doc(no_inline)]
    pub use crate::models::madhab::Madhab;
    #[doc(no_inline)]
    pub use crate::models::method::Method;
    #[doc(no_inline)]
    pub use crate::models::parameters::{Parameters, ParametersBuilder};
    #[doc(no_inline)]
    pub use crate::models::polar_circle_resolution::PolarCircleResolution;
    #[doc(no_inline)]
    pub use crate::models::prayer::Prayer;
    #[doc(no_inline)]
    pub use crate::models::prayer_time::PrayerTime;
    #[doc(no_inline)]
    pub use crate::models::twilight::Twilight;
    #[doc(no_inline)]
    pub use crate::prayer_times::PrayerTimes;
    #[doc(no_inline)]
    pub use chrono::{Date, DateTime, Datelike, Duration, Local, TimeZone, Timelike, Utc};
}
