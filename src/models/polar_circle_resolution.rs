use clap::ValueEnum;

#[derive(PartialEq, Debug, Copy, Clone, ValueEnum)]
pub enum PolarCircleResolution {
    NearestTown,
    NearestDay,
    UmmAlQura,
    Unresolved,
}

impl Default for PolarCircleResolution {
    fn default() -> Self {
        PolarCircleResolution::UmmAlQura
    }
}
