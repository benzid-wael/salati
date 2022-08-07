use wasm_bindgen::prelude::*;

mod astronomy;
mod constants;
mod models;
mod prayer_times;

pub use crate::astronomy::qiblah::Qiblah;
pub use crate::models::adjustments::{TimeAdjustment, TimeAdjustmentBuilder};
pub use crate::models::madhab::Madhab;
pub use crate::models::method::Method;
pub use crate::models::parameters::{Parameters, ParametersBuilder};
pub use crate::models::prayer::Prayer;
pub use crate::models::twilight::Twilight;
pub use crate::prayer_times::PrayerTimes;

#[wasm_bindgen]
extern "C" {
    pub fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet(name: &str) {
    alert(&format!("Hello, {}!", name));
}
