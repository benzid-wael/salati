// Setting for the Asr prayer time.
// For Hanafi madhab, the Asr will be later
#[derive(PartialEq, Debug, Copy, Clone)]
pub enum Madhab {
    Shafi = 1,
    Hanafi = 2,
}

impl Madhab {
    pub fn shadow_length_ratio(&self) -> i32 {
        *self as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn shafi_shadow_length_ratio() {
        let shafi = Madhab::Shafi;

        assert_eq!(shafi.shadow_length_ratio(), 1);
    }

    #[test]
    fn hanafi_shadow_length_ratio() {
        let hanafi = Madhab::Hanafi;

        assert_eq!(hanafi.shadow_length_ratio(), 2);
    }
}
