use chumsky::prelude::*;

use crate::traits::Parsable;

#[derive(PartialEq, Eq, Copy, Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
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

impl Parsable for WeatherIntensity {
    fn parser<'src>() -> impl Parser<'src, &'src str, Self, extra::Err<crate::MetarError<'src>>> {
        choice((
            just("-").map(|_| WeatherIntensity::Light),
            just("+").map(|_| WeatherIntensity::Heavy),
            just("VC").map(|_| WeatherIntensity::InVicinity),
            just("RE").map(|_| WeatherIntensity::Recent),
            empty().map(|()| WeatherIntensity::Moderate),
        ))
    }
}
