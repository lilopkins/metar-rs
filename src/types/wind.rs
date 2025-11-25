use super::Data;
use super::WindDirection;
use super::WindSpeed;

#[derive(PartialEq, Eq, Copy, Clone, Debug)]
/// Wind information.
pub struct Wind {
    /// The wind direction, in degrees
    pub dir: Data<WindDirection>,
    /// The current wind speed
    pub speed: Data<WindSpeed>,
    /// The direction the wind may be varying between, smaller always comes first
    pub varying: Option<(Data<u32>, Data<u32>)>,
    /// The gusting speed of the wind
    pub gusting: Option<WindSpeed>,
}
