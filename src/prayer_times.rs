use chrono::{Date, DateTime, Datelike, Duration, Utc};

use crate::astronomy::ops;
use crate::astronomy::solar::SolarTime;
use crate::astronomy::unit::{Angle, Coordinates, Stride};
use crate::constants::{is_high_latitude, HIGH_LATITUDE_RESOLUTION_MESSAGE};
use crate::models::method::Method;
use crate::models::parameters::Parameters;
use crate::models::prayer::Prayer;
use crate::models::prayer_time::PrayerTime;
use crate::models::prayer_time::PrayerTimeBuilder;
use crate::models::prayer_time::PrayerTimeResolution;

#[derive(PartialEq, Debug, Clone)]
pub struct PrayerTimes {
    pub fajr: PrayerTime,
    pub sunrise: PrayerTime,
    pub solar_sunrise: PrayerTime,
    pub dhuhr: PrayerTime,
    pub asr: PrayerTime,
    pub maghrib: PrayerTime,
    pub solar_sunset: PrayerTime,
    pub isha: PrayerTime,
    pub middle_of_the_night: PrayerTime,
    pub qiyam: PrayerTime,
    pub fajr_tomorrow: PrayerTime,
    pub coordinates: Coordinates,
    pub date: DateTime<Utc>,
    pub parameters: Parameters,
}

fn calculate_solar_time(
    date: DateTime<Utc>,
    coordinates: Coordinates,
    _parameters: Parameters,
) -> (SolarTime, PrayerTimeResolution) {
    // todo: handle polar circle regions
    (
        SolarTime::new(date, coordinates),
        PrayerTimeResolution::Normal,
    )
}

impl PrayerTimes {
    pub fn new(date: Date<Utc>, coordinates: Coordinates, parameters: Parameters) -> PrayerTimes {
        let prayer_date = date.and_hms(0, 0, 0);
        let tomorrow = prayer_date.tomorrow();
        let (solar_time, _today_prayer_time_resolution) =
            calculate_solar_time(prayer_date, coordinates, parameters);
        let (solar_time_tomorrow, _tomorrow_prayer_time_resolution) =
            calculate_solar_time(tomorrow, coordinates, parameters);

        let asr = solar_time.afternoon(parameters.madhab.shadow_length_ratio().into());
        let night_duration = solar_time_tomorrow
            .sunrise
            .unwrap()
            .signed_duration_since(solar_time.sunset.unwrap());

        let final_fajr = PrayerTimes::calculate_fajr_time(
            parameters,
            solar_time,
            night_duration,
            coordinates,
            prayer_date,
        );
        let final_sunrise = solar_time
            .sunrise
            .unwrap()
            .adjust_time(parameters.time_adjustments(Prayer::Sunrise));
        let final_dhuhr = solar_time
            .transit
            .unwrap()
            .adjust_time(parameters.time_adjustments(Prayer::Dhuhr));
        let final_asr = asr.adjust_time(parameters.time_adjustments(Prayer::Asr));
        let final_maghrib = ops::adjust_time(
            &solar_time.sunset.unwrap(),
            parameters.time_adjustments(Prayer::Maghrib),
        );
        let final_isha = PrayerTimes::calculate_isha_time(
            parameters,
            solar_time,
            night_duration,
            coordinates,
            prayer_date,
        );

        // Calculate the middle of the night and qiyam times
        let (final_middle_of_night, final_qiyam, final_fajr_tomorrow) =
            PrayerTimes::calculate_qiyam_time(
                final_maghrib,
                parameters,
                solar_time_tomorrow,
                coordinates,
                tomorrow,
            );

        PrayerTimes {
            fajr: final_fajr,
            sunrise: PrayerTime::new(Some(final_sunrise)),
            solar_sunrise: PrayerTime::new(solar_time.sunrise),
            dhuhr: PrayerTime::new(Some(final_dhuhr)),
            asr: PrayerTime::new(Some(final_asr)),
            maghrib: PrayerTime::new(Some(final_maghrib)),
            solar_sunset: PrayerTime::new(solar_time.sunset),
            isha: final_isha,
            middle_of_the_night: final_middle_of_night,
            qiyam: final_qiyam,
            fajr_tomorrow: final_fajr_tomorrow,
            coordinates,
            date: prayer_date,
            parameters,
        }
    }

    pub fn prayer_time(&self, prayer: Prayer) -> PrayerTime {
        match prayer {
            Prayer::Fajr => self.fajr.clone(),
            Prayer::Sunrise => self.sunrise.clone(),
            Prayer::Dhuhr => self.dhuhr.clone(),
            Prayer::Asr => self.asr.clone(),
            Prayer::Maghrib => self.maghrib.clone(),
            Prayer::Isha => self.isha.clone(),
            Prayer::MiddleOfTheNight => self.middle_of_the_night.clone(),
            Prayer::Qiyam => self.qiyam.clone(),
            Prayer::FajrTomorrow => self.fajr_tomorrow.clone(),
        }
    }

    pub fn time(&self, prayer: Prayer) -> DateTime<Utc> {
        let prayer_time = self.prayer_time(prayer);
        prayer_time.datetime.unwrap()
    }

    /// Returns current prayer if any
    ///
    /// Indeed, this method returns the last started prayer time, with the following assumptions:
    /// 1) next day did not start yet
    /// 2) prayer time last to the next prayer
    fn current_prayer(&self, time: DateTime<Utc>) -> Option<Prayer> {
        let current_prayer: Option<Prayer>;

        if self
            .fajr_tomorrow
            .datetime?
            .signed_duration_since(time)
            .num_seconds()
            <= 0
        {
            current_prayer = Some(Prayer::FajrTomorrow)
        } else if self
            .qiyam
            .datetime?
            .signed_duration_since(time)
            .num_seconds()
            <= 0
        {
            current_prayer = Some(Prayer::Qiyam)
        } else if self
            .middle_of_the_night
            .datetime?
            .signed_duration_since(time)
            .num_seconds()
            <= 0
        {
            current_prayer = Some(Prayer::MiddleOfTheNight)
        } else if self
            .isha
            .datetime?
            .signed_duration_since(time)
            .num_seconds()
            <= 0
        {
            current_prayer = Some(Prayer::Isha);
        } else if self
            .maghrib
            .datetime?
            .signed_duration_since(time)
            .num_seconds()
            <= 0
        {
            current_prayer = Some(Prayer::Maghrib);
        } else if self.asr.datetime?.signed_duration_since(time).num_seconds() <= 0 {
            current_prayer = Some(Prayer::Asr);
        } else if self
            .dhuhr
            .datetime?
            .signed_duration_since(time)
            .num_seconds()
            <= 0
        {
            current_prayer = Some(Prayer::Dhuhr);
        } else if self
            .sunrise
            .datetime?
            .signed_duration_since(time)
            .num_seconds()
            <= 0
        {
            current_prayer = Some(Prayer::Sunrise);
        } else if self
            .fajr
            .datetime?
            .signed_duration_since(time)
            .num_seconds()
            <= 0
        {
            current_prayer = Some(Prayer::Fajr);
        } else {
            current_prayer = None;
        }

        current_prayer
    }

    /// Returns current prayer
    pub fn current(&self) -> Prayer {
        self.current_prayer(Utc::now()).expect("Out of bounds")
    }

    /// Returns next prayer
    pub fn next(&self) -> Prayer {
        match self.current() {
            Prayer::Fajr => Prayer::Sunrise,
            Prayer::Sunrise => Prayer::Dhuhr,
            Prayer::Dhuhr => Prayer::Asr,
            Prayer::Asr => Prayer::Maghrib,
            Prayer::Maghrib => Prayer::Isha,
            Prayer::Isha => Prayer::MiddleOfTheNight,
            Prayer::MiddleOfTheNight => Prayer::Qiyam,
            Prayer::Qiyam => Prayer::FajrTomorrow,
            _ => Prayer::FajrTomorrow,
        }
    }

    /// Returns time remaining to next prayer
    pub fn time_remaining(&self) -> (u32, u32) {
        let next_time = self.time(self.next());
        let now = Utc::now();
        let now_to_next = next_time.signed_duration_since(now).num_seconds() as f64;
        let whole: f64 = now_to_next / 60.0 / 60.0;
        let fract = whole.fract();
        let hours = whole.trunc() as u32;
        let minutes = (fract * 60.0).round() as u32;

        (hours, minutes)
    }

    fn calculate_fajr_time(
        parameters: Parameters,
        solar_time: SolarTime,
        night: Duration,
        coordinates: Coordinates,
        prayer_date: DateTime<Utc>,
    ) -> PrayerTime {
        let mut fajr = solar_time.time_for_solar_angle(Angle::new(-parameters.fajr_angle), false);
        let mut message = "";
        let mut prayer_time_resolution = PrayerTimeResolution::default();

        // This is a special case for Moonsighting Committee: latitude above 55.0
        if parameters.method == Method::MoonsightingCommittee
            && is_high_latitude(coordinates, Some(parameters.method))
        {
            let night_fraction = night.num_seconds() / 7;
            fajr = solar_time
                .sunrise
                .unwrap()
                .checked_add_signed(Duration::seconds(-night_fraction as i64))
                .unwrap();
        }

        // At latitudes:
        // 1) between 55 and 60: Fajr and Isha time are very close to each other
        // 2) between 60 and 65: hardship prevails (Fajr and Isha time becomes more closer)
        // 3) above 65: sun does not set/rise for a number of days.
        //
        // For the 3rd case, we will fall to polar circle resolution method
        //    the given solar_time will not be the original one based on CirclePolarResolution strategy
        // Otherwise, we will return the latest time for fajr
        let safe_fajr = if parameters.method == Method::MoonsightingCommittee {
            let day_of_year = prayer_date.ordinal();
            ops::season_adjusted_morning_twilight(
                coordinates.latitude,
                day_of_year,
                prayer_date.year() as u32,
                solar_time.sunrise.unwrap(),
            )
        } else {
            let portion = parameters.night_portions().0;
            let night_fraction = portion * (night.num_seconds() as f64);

            solar_time
                .sunrise
                .unwrap()
                .checked_add_signed(Duration::seconds(-night_fraction as i64))
                .unwrap()
        };

        // This check is applied only at high latitudes
        if is_high_latitude(coordinates, None) && fajr < safe_fajr {
            fajr = safe_fajr;
            prayer_time_resolution = PrayerTimeResolution::HighLatitudeRule;
            message = HIGH_LATITUDE_RESOLUTION_MESSAGE;
        }

        // finally, let's apply time adjustments
        fajr = fajr.adjust_time(parameters.time_adjustments(Prayer::Fajr));
        PrayerTimeBuilder::new(Some(fajr))
            .code(prayer_time_resolution)
            .message(String::from(message))
            .build()
    }

    fn calculate_isha_time(
        parameters: Parameters,
        solar_time: SolarTime,
        night: Duration,
        coordinates: Coordinates,
        prayer_date: DateTime<Utc>,
    ) -> PrayerTime {
        let mut isha: DateTime<Utc>;
        let mut message = "";
        let mut prayer_time_resolution = PrayerTimeResolution::default();

        if parameters.isha_interval > 0 {
            isha = solar_time
                .sunset
                .unwrap()
                .checked_add_signed(Duration::seconds((parameters.isha_interval * 60) as i64))
                .unwrap();
        } else {
            isha = solar_time.time_for_solar_angle(Angle::new(-parameters.isha_angle), true);

            // This is a special case for Moonsighting Committee: latitude above 55.0
            if parameters.method == Method::MoonsightingCommittee
                && is_high_latitude(coordinates, Some(parameters.method))
            {
                let night_fraction = night.num_seconds() / 7;
                isha = solar_time
                    .sunset
                    .unwrap()
                    .checked_add_signed(Duration::seconds(night_fraction))
                    .unwrap();
            }

            let safe_isha = if parameters.method == Method::MoonsightingCommittee {
                let day_of_year = prayer_date.ordinal();

                ops::season_adjusted_evening_twilight(
                    coordinates.latitude,
                    day_of_year,
                    prayer_date.year() as u32,
                    solar_time.sunset.unwrap(),
                )
            } else {
                let portion = parameters.night_portions().1;
                let night_fraction = portion * (night.num_seconds() as f64);

                solar_time
                    .sunset
                    .unwrap()
                    .checked_add_signed(Duration::seconds(night_fraction as i64))
                    .unwrap()
            };

            // This check is applied only at high latitudes
            if is_high_latitude(coordinates, None) && isha > safe_isha {
                isha = safe_isha;
                prayer_time_resolution = PrayerTimeResolution::HighLatitudeRule;
                message = HIGH_LATITUDE_RESOLUTION_MESSAGE;
            }
        }

        // finally, let's apply time adjustments
        isha = isha.adjust_time(parameters.time_adjustments(Prayer::Isha));
        PrayerTimeBuilder::new(Some(isha))
            .code(prayer_time_resolution)
            .message(String::from(message))
            .build()
    }

    fn calculate_qiyam_time(
        current_maghrib: DateTime<Utc>,
        parameters: Parameters,
        solar_time: SolarTime,
        coordinates: Coordinates,
        prayer_date: DateTime<Utc>,
    ) -> (PrayerTime, PrayerTime, PrayerTime) {
        // this is actually day after tomorrow
        let tomorrow = prayer_date.tomorrow();
        let (solar_time_tomorrow, _tomorrow_prayer_time_resolution) =
            calculate_solar_time(tomorrow, coordinates, parameters);

        let night = solar_time_tomorrow
            .sunrise
            .unwrap()
            .signed_duration_since(solar_time.sunset.unwrap());

        let tomorrow_fajr = PrayerTimes::calculate_fajr_time(
            parameters,
            solar_time,
            night,
            coordinates,
            prayer_date,
        );
        let night_duration = tomorrow_fajr
            .datetime
            .unwrap()
            .signed_duration_since(current_maghrib)
            .num_seconds() as f64;
        let middle_night_portion = (night_duration / 2.0) as i64;
        let last_third_portion = (night_duration * (2.0 / 3.0)) as i64;
        let middle_of_night = current_maghrib
            .checked_add_signed(Duration::seconds(middle_night_portion))
            .unwrap()
            .nearest_minute();
        let last_third_of_night = current_maghrib
            .checked_add_signed(Duration::seconds(last_third_portion))
            .unwrap()
            .nearest_minute();

        (
            PrayerTimeBuilder::new(Some(middle_of_night)).build(),
            PrayerTimeBuilder::new(Some(last_third_of_night)).build(),
            tomorrow_fajr,
        )
    }
}
