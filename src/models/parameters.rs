use super::adjustments::TimeAdjustment;
use super::high_latitude_rule::HighLatitudeRule;
use super::madhab::Madhab;
use super::method::Method;
use super::polar_circle_resolution::PolarCircleResolution;
use super::prayer::Prayer;
use super::twilight::Twilight;

// Parameters to calculate prayer times
#[derive(PartialEq, Debug, Copy, Clone)]
pub struct Parameters {
    pub method: Method,
    pub fajr_angle: f64,
    pub isha_angle: f64,
    pub isha_interval: i32,
    pub madhab: Madhab,
    pub twilight: Twilight,
    pub high_latitude_rule: HighLatitudeRule,
    pub polar_circle_resolution: PolarCircleResolution,
    pub adjustments: TimeAdjustment,
    pub method_adjustments: TimeAdjustment,
}

impl Parameters {
    pub fn new(fajr_angle: f64, isha_angle: f64) -> Parameters {
        Parameters {
            fajr_angle,
            isha_angle,
            method: Method::Other,
            isha_interval: 0,
            madhab: Madhab::Shafi,
            twilight: Twilight::Red,
            high_latitude_rule: HighLatitudeRule::default(),
            polar_circle_resolution: PolarCircleResolution::Unresolved,
            adjustments: TimeAdjustment::default(),
            method_adjustments: TimeAdjustment::default(),
        }
    }

    pub fn night_portions(&self) -> (f64, f64) {
        match self.high_latitude_rule {
            HighLatitudeRule::MiddleOfTheNight => (1.0 / 2.0, 1.0 / 2.0),
            HighLatitudeRule::SeventhOfTheNight => (1.0 / 7.0, 1.0 / 7.0),
            HighLatitudeRule::TwilightAngle => (self.fajr_angle / 60.0, self.isha_angle / 60.0),
        }
    }

    pub fn time_adjustments(&self, prayer: Prayer) -> i64 {
        match prayer {
            Prayer::Fajr => self.adjustments.fajr + self.method_adjustments.fajr,
            Prayer::Sunrise => self.adjustments.sunrise + self.method_adjustments.sunrise,
            Prayer::Dhuhr => self.adjustments.dhuhr + self.method_adjustments.dhuhr,
            Prayer::Asr => self.adjustments.asr + self.method_adjustments.asr,
            Prayer::Maghrib => self.adjustments.maghrib + self.method_adjustments.maghrib,
            Prayer::Isha => self.adjustments.isha + self.method_adjustments.isha,
            _ => 0,
        }
    }
}

/// A builder for the the [Parameters](struct.Parameters.html).
///
/// It is recommended that this is used for setting
/// all parameters that are needed.
pub struct ParametersBuilder {
    method: Method,
    fajr_angle: f64,
    isha_angle: f64,
    isha_interval: i32,
    madhab: Madhab,
    pub twilight: Twilight,
    pub high_latitude_rule: HighLatitudeRule,
    pub polar_circle_resolution: PolarCircleResolution,
    adjustments: TimeAdjustment,
    method_adjustments: TimeAdjustment,
}

impl ParametersBuilder {
    pub fn new(fajr_angle: f64, isha_angle: f64) -> ParametersBuilder {
        ParametersBuilder {
            fajr_angle,
            isha_angle,
            method: Method::Other,
            isha_interval: 0,
            madhab: Madhab::Shafi,
            twilight: Twilight::Red,
            high_latitude_rule: HighLatitudeRule::MiddleOfTheNight,
            polar_circle_resolution: PolarCircleResolution::Unresolved,
            adjustments: TimeAdjustment::default(),
            method_adjustments: TimeAdjustment::default(),
        }
    }

    pub fn with(method: Method, madhab: Madhab) -> Parameters {
        let mut params = method.parameters();
        params.madhab = madhab;

        params
    }

    pub fn method(&mut self, method: Method) -> &mut ParametersBuilder {
        self.method = method;
        self
    }

    pub fn method_adjustments(
        &mut self,
        method_adjustments: TimeAdjustment,
    ) -> &mut ParametersBuilder {
        self.method_adjustments = method_adjustments;
        self
    }

    pub fn polar_circle_resolution(
        &mut self,
        polar_circle_resolution: PolarCircleResolution,
    ) -> &mut ParametersBuilder {
        self.polar_circle_resolution = polar_circle_resolution;
        self
    }

    pub fn high_latitude_rule(
        &mut self,
        high_latitude_rule: HighLatitudeRule,
    ) -> &mut ParametersBuilder {
        self.high_latitude_rule = high_latitude_rule;
        self
    }

    pub fn madhab(&mut self, madhab: Madhab) -> &mut ParametersBuilder {
        self.madhab = madhab;
        self
    }

    pub fn twilight(&mut self, twilight: Twilight) -> &mut ParametersBuilder {
        self.twilight = twilight;
        self
    }

    pub fn isha_interval(&mut self, isha_interval: i32) -> &mut ParametersBuilder {
        self.isha_angle = 0.0;
        self.isha_interval = isha_interval;
        self
    }

    pub fn build(&self) -> Parameters {
        Parameters {
            fajr_angle: self.fajr_angle,
            isha_angle: self.isha_angle,
            method: self.method,
            isha_interval: self.isha_interval,
            madhab: self.madhab,
            twilight: self.twilight,
            high_latitude_rule: self.high_latitude_rule,
            polar_circle_resolution: self.polar_circle_resolution,
            adjustments: self.adjustments,
            method_adjustments: self.method_adjustments,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn calculate_parameters_with_fajr_and_isha_angles() {
        let params = Parameters::new(18.0, 18.0);

        assert_eq!(params.fajr_angle, 18.0);
        assert_eq!(params.isha_angle, 18.0);
        assert_eq!(params.isha_interval, 0);
    }

    #[test]
    fn calculated_night_portions_default_to_twilight_angle() {
        let params = Parameters::new(18.0, 18.0);

        assert_eq!(params.night_portions().0, 18. / 60.);
        assert_eq!(params.night_portions().1, 18. / 60.);
    }

    #[test]
    fn calculated_night_portions_middle_of_the_night() {
        let params = ParametersBuilder::new(18.0, 18.0)
            .high_latitude_rule(HighLatitudeRule::MiddleOfTheNight)
            .build();

        assert_eq!(params.night_portions().0, 1. / 2.);
        assert_eq!(params.night_portions().1, 1. / 2.);
    }

    #[test]
    fn calculated_night_portions_seventh_of_the_night() {
        let params = ParametersBuilder::new(18.0, 18.0)
            .high_latitude_rule(HighLatitudeRule::SeventhOfTheNight)
            .build();

        assert_eq!(params.night_portions().0, 1.0 / 7.0);
        assert_eq!(params.night_portions().1, 1.0 / 7.0);
    }

    #[test]
    fn calculated_night_portions_twilight_angle() {
        let params = ParametersBuilder::new(10.0, 15.0)
            .high_latitude_rule(HighLatitudeRule::TwilightAngle)
            .build();

        assert_eq!(params.night_portions().0, 10.0 / 60.0);
        assert_eq!(params.night_portions().1, 15.0 / 60.0);
    }

    #[test]
    fn parameters_using_method_and_madhab() {
        let params = ParametersBuilder::with(Method::NorthAmerica, Madhab::Hanafi);

        assert_eq!(params.method, Method::NorthAmerica);
        assert_eq!(params.fajr_angle, 15.0);
        assert_eq!(params.isha_angle, 15.0);
        assert_eq!(params.isha_interval, 0);
        assert_eq!(params.madhab, Madhab::Hanafi);
    }
}
