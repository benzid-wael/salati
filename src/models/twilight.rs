#[derive(PartialEq, Debug, Copy, Clone)]

/// Enum to define how Isha prayer should be computed
/// Imam Abu Hanifa consider that scripts refer to white twilight (shafaq), however
/// other schools and two main students of Abu Hanifa (incl. Abu Yussuf)
/// consider that they meant red twilight
pub enum Twilight {
    White,
    Red,
}
