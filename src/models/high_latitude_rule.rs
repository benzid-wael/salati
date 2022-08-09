use clap::ValueEnum;

use crate::astronomy::unit::Coordinates;
use crate::constants::is_high_latitude;

#[derive(PartialEq, Debug, Copy, Clone, ValueEnum)]
pub enum HighLatitudeRule {
    MiddleOfTheNight,
    SeventhOfTheNight,
    TwilightAngle,
}

impl Default for HighLatitudeRule {
    fn default() -> HighLatitudeRule {
        HighLatitudeRule::TwilightAngle
    }
}

impl HighLatitudeRule {
    pub fn recommended(coordinates: Coordinates) -> Self {
        match is_high_latitude(coordinates, None) {
            true => HighLatitudeRule::SeventhOfTheNight,
            false => HighLatitudeRule::MiddleOfTheNight,
        }
    }
}
