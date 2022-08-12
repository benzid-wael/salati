use chrono::{Datelike, Utc, Weekday};

/// Names of all obligatory prayers,
/// sunrise, and Qiyam.
#[derive(PartialEq, Debug, Copy, Clone)]
pub enum Prayer {
    Fajr,
    Sunrise,
    Dhuhr,
    Asr,
    Maghrib,
    Isha,
    MiddleOfTheNight,
    Qiyam,
    FajrTomorrow,
}

impl Prayer {
    pub fn name(&self) -> String {
        match self {
            Prayer::Fajr | Prayer::FajrTomorrow => String::from("Fajr"),
            Prayer::Sunrise => String::from("Sunrise"),
            Prayer::Dhuhr => {
                if Utc::now().weekday() == Weekday::Fri {
                    String::from("Jumua")
                } else {
                    String::from("Dhuhr")
                }
            }
            Prayer::Asr => String::from("Asr"),
            Prayer::Maghrib => String::from("Maghrib"),
            Prayer::Isha => String::from("Isha"),
            Prayer::MiddleOfTheNight => String::from("Middle Of The Night"),
            Prayer::Qiyam => String::from("Qiyam"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn prayer_name_for_fajr_en_transliteration() {
        assert_eq!(Prayer::Fajr.name(), "Fajr");
        assert_eq!(Prayer::Sunrise.name(), "Sunrise");

        if Utc::now().weekday() == Weekday::Fri {
            assert_eq!(Prayer::Dhuhr.name(), "Jumua");
        } else {
            assert_eq!(Prayer::Dhuhr.name(), "Dhuhr");
        }

        assert_eq!(Prayer::Asr.name(), "Asr");
        assert_eq!(Prayer::Maghrib.name(), "Maghrib");
        assert_eq!(Prayer::Isha.name(), "Isha");
        assert_eq!(Prayer::MiddleOfTheNight.name(), "Middle Of The Night");
        assert_eq!(Prayer::Qiyam.name(), "Qiyam");
    }
}
