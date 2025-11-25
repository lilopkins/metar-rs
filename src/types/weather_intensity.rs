#[derive(PartialEq, Eq, Copy, Clone, Debug)]
/// Intensity of weather
pub enum WeatherIntensity {
    /// Light (-)
    Light,
    /// Moderate (no prefix)
    Moderate,
    /// Heavy (+)
    Heavy,
    /// In the vicinity (VC)
    InVicinity,
    /// Recent (RE)
    Recent,
}
