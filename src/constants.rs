use crate::astronomy::unit::Coordinates;
use crate::models::method::Method;

pub static HIGH_LATITUDE_THRESHOLD: f64 = 48.0;
pub static MOONSIGHTING_COMITTEE_HIGH_LATITUDE: f64 = 55.0;
pub static HIGH_LATITUDE_RESOLUTION_MESSAGE: &str = "At higher latitudes, where Fajr and Isha times are very close to each other, we fallback to high latitude resolution strategy.";

pub fn is_high_latitude(coordinates: Coordinates, method: Option<Method>) -> bool {
    match method {
        Some(Method::MoonsightingCommittee) => {
            coordinates.latitude >= MOONSIGHTING_COMITTEE_HIGH_LATITUDE
        }
        _ => coordinates.latitude >= HIGH_LATITUDE_THRESHOLD,
    }
}
