use chrono::{DateTime, Utc};

#[derive(PartialEq, Debug, Copy, Clone)]
pub enum PrayerTimeResolution {
    /// Indicates that no adjustment or correction was applied
    Normal,
    /// Indicates that prayer was adjusted according to HighLatitudeRule
    HighLatitudeRule,
    /// Sometimes, we get invalid time for high latitude regions
    /// For example, Fajr time cannot be after sunrise
    // and Isha time cannot be after sunrise or before middle of the night (in the prepondirant)
    Invalid,
    /// In polar circle region (not all cities), the does not set or does not rise
    /// in specific times of the year
    PolarCircle,
}

impl Default for PrayerTimeResolution {
    fn default() -> PrayerTimeResolution {
        PrayerTimeResolution::Normal
    }
}

#[derive(PartialEq, Debug, Clone)]
pub struct PrayerTime {
    pub datetime: Option<DateTime<Utc>>,
    pub code: PrayerTimeResolution,
    pub message: String,
}

impl PrayerTime {
    pub fn new(datetime: Option<DateTime<Utc>>) -> Self {
        PrayerTime {
            datetime,
            code: match datetime {
                Some(_dt) => PrayerTimeResolution::Normal,
                None => PrayerTimeResolution::Invalid,
            },
            message: String::default(),
        }
    }
}

#[derive(PartialEq, Debug, Clone)]
pub struct PrayerTimeBuilder {
    pub datetime: Option<DateTime<Utc>>,
    pub code: PrayerTimeResolution,
    pub message: String,
}

impl PrayerTimeBuilder {
    pub fn new(datetime: Option<DateTime<Utc>>) -> PrayerTimeBuilder {
        PrayerTimeBuilder {
            datetime,
            code: match datetime {
                Some(_dt) => PrayerTimeResolution::Normal,
                None => PrayerTimeResolution::Invalid,
            },
            message: String::default(),
        }
    }

    pub fn code(&mut self, code: PrayerTimeResolution) -> &mut PrayerTimeBuilder {
        self.code = code;
        self
    }

    pub fn message(&mut self, message: String) -> &mut PrayerTimeBuilder {
        self.message = message;
        self
    }

    pub fn build(&self) -> PrayerTime {
        PrayerTime {
            datetime: self.datetime,
            code: self.code,
            message: self.message.clone(),
        }
    }
}
