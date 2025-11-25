use super::WeatherCondition;
use super::WeatherIntensity;

#[derive(PartialEq, Eq, Clone, Debug)]
/// A weather information block
pub struct Weather {
    /// The intensity of this weather block
    pub intensity: WeatherIntensity,
    /// The weather condition/s this block describes.
    pub conditions: Vec<WeatherCondition>,
}
