use super::adjustments::TimeAdjustmentBuilder;
use super::high_latitude_rule::HighLatitudeRule;

use super::parameters::{Parameters, ParametersBuilder};

/// Provides preset configuration for a few authorities
/// for calculating prayer times.
#[derive(PartialEq, Debug, Copy, Clone)]
pub enum Method {
    /// Muslim World League
    MuslimWorldLeague,

    /// Egyptian General Authority of Survey
    Egyptian,

    /// University of Islamic Sciences, Karachi
    Karachi,

    /// Umm al-Qura University, Makkah
    UmmAlQura,

    /// The Gulf Region
    Dubai,

    /// Moonsighting Committee
    MoonsightingCommittee,

    /// ISNA
    NorthAmerica,

    /// Kuwait
    Kuwait,

    /// Qatar
    Qatar,

    /// Singapore
    Singapore,

    /// Other
    Other,
}

impl Method {
    pub fn parameters(&self) -> Parameters {
        match self {
            Method::MuslimWorldLeague => ParametersBuilder::new(18.0, 17.0)
                .method(*self)
                .method_adjustments(TimeAdjustmentBuilder::new().dhuhr(1).build())
                .build(),

            Method::Egyptian => ParametersBuilder::new(19.5, 17.5)
                .method(*self)
                .method_adjustments(TimeAdjustmentBuilder::new().dhuhr(1).build())
                .build(),

            Method::Karachi => ParametersBuilder::new(18.0, 18.0)
                .method(*self)
                .method_adjustments(TimeAdjustmentBuilder::new().dhuhr(1).build())
                .build(),

            Method::UmmAlQura => ParametersBuilder::new(18.5, 0.0)
                .method(*self)
                .isha_interval(90)
                .build(),

            Method::Dubai => ParametersBuilder::new(18.2, 18.2)
                .method(*self)
                .method_adjustments(
                    TimeAdjustmentBuilder::new()
                        .sunrise(-3)
                        .dhuhr(3)
                        .asr(3)
                        .maghrib(3)
                        .build(),
                )
                .build(),

            Method::MoonsightingCommittee => ParametersBuilder::new(18.0, 18.0)
                .method(*self)
                .high_latitude_rule(HighLatitudeRule::SeventhOfTheNight)
                .method_adjustments(TimeAdjustmentBuilder::new().dhuhr(5).maghrib(3).build())
                .build(),

            Method::NorthAmerica => ParametersBuilder::new(15.0, 15.0)
                .method(*self)
                .method_adjustments(TimeAdjustmentBuilder::new().dhuhr(1).build())
                .build(),

            Method::Kuwait => ParametersBuilder::new(18.0, 17.5).method(*self).build(),

            Method::Qatar => ParametersBuilder::new(18.0, 0.0)
                .method(*self)
                .isha_interval(90)
                .build(),

            Method::Singapore => ParametersBuilder::new(20.0, 18.0)
                .method(*self)
                .method_adjustments(TimeAdjustmentBuilder::new().dhuhr(1).build())
                .build(),

            Method::Other => ParametersBuilder::new(0.0, 0.0).method(*self).build(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parameters_for_muslim_world_league() {
        let method = Method::MuslimWorldLeague;
        let params = method.parameters();

        assert_eq!(params.method, Method::MuslimWorldLeague);
        assert_eq!(params.fajr_angle, 18.0);
        assert_eq!(params.isha_angle, 17.0);
        assert_eq!(params.isha_interval, 0);
    }

    #[test]
    fn parameters_for_egyptian() {
        let method = Method::Egyptian;
        let params = method.parameters();

        assert_eq!(params.method, Method::Egyptian);
        assert_eq!(params.fajr_angle, 19.5);
        assert_eq!(params.isha_angle, 17.5);
        assert_eq!(params.isha_interval, 0);
    }

    #[test]
    fn parameters_for_karachi() {
        let method = Method::Karachi;
        let params = method.parameters();

        assert_eq!(params.method, Method::Karachi);
        assert_eq!(params.fajr_angle, 18.0);
        assert_eq!(params.isha_angle, 18.0);
        assert_eq!(params.isha_interval, 0);
    }

    #[test]
    fn parameters_for_umm_al_qura() {
        let method = Method::UmmAlQura;
        let params = method.parameters();

        assert_eq!(params.method, Method::UmmAlQura);
        assert_eq!(params.fajr_angle, 18.5);
        assert_eq!(params.isha_angle, 0.0);
        assert_eq!(params.isha_interval, 90);
    }

    #[test]
    fn parameters_for_dubai() {
        let method = Method::Dubai;
        let params = method.parameters();

        assert_eq!(params.method, Method::Dubai);
        assert_eq!(params.fajr_angle, 18.2, "Parameters: {:?}", params);
        assert_eq!(params.isha_angle, 18.2);
        assert_eq!(params.isha_interval, 0);
    }

    #[test]
    fn parameters_for_moonsighting_committee() {
        let method = Method::MoonsightingCommittee;
        let params = method.parameters();

        assert_eq!(params.method, Method::MoonsightingCommittee);
        assert_eq!(params.fajr_angle, 18.0);
        assert_eq!(params.isha_angle, 18.0);
        assert_eq!(params.isha_interval, 0);
    }

    #[test]
    fn parameters_for_north_america() {
        let method = Method::NorthAmerica;
        let params = method.parameters();

        assert_eq!(params.method, Method::NorthAmerica);
        assert_eq!(params.fajr_angle, 15.0);
        assert_eq!(params.isha_angle, 15.0);
        assert_eq!(params.isha_interval, 0);
    }

    #[test]
    fn parameters_for_kuwait() {
        let method = Method::Kuwait;
        let params = method.parameters();

        assert_eq!(params.method, Method::Kuwait);
        assert_eq!(params.fajr_angle, 18.0);
        assert_eq!(params.isha_angle, 17.5);
        assert_eq!(params.isha_interval, 0);
    }

    #[test]
    fn parameters_for_qatar() {
        let method = Method::Qatar;
        let params = method.parameters();

        assert_eq!(params.method, Method::Qatar);
        assert_eq!(params.fajr_angle, 18.0);
        assert_eq!(params.isha_angle, 0.0);
        assert_eq!(params.isha_interval, 90);
    }

    #[test]
    fn parameters_for_singapore() {
        let method = Method::Singapore;
        let params = method.parameters();

        assert_eq!(params.method, Method::Singapore);
        assert_eq!(params.fajr_angle, 20.0);
        assert_eq!(params.isha_angle, 18.0);
        assert_eq!(params.isha_interval, 0);
    }

    #[test]
    fn parameters_for_other() {
        let method = Method::Other;
        let params = method.parameters();

        assert_eq!(params.method, Method::Other);
        assert_eq!(params.fajr_angle, 0.0);
        assert_eq!(params.isha_angle, 0.0);
        assert_eq!(params.isha_interval, 0);
    }
}
