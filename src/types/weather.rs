use chumsky::prelude::*;

use crate::traits::Parsable;

use super::WeatherCondition;
use super::WeatherIntensity;

#[derive(PartialEq, Eq, Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
/// A weather information block
pub struct Weather {
    /// The intensity of this weather block
    pub intensity: WeatherIntensity,
    /// The weather condition/s this block describes.
    pub conditions: Vec<WeatherCondition>,
}

impl Parsable for Weather {
    fn parser<'src>() -> impl Parser<'src, &'src str, Self, extra::Err<crate::MetarError<'src>>> {
        WeatherIntensity::parser()
            .then(
                WeatherCondition::parser()
                    .repeated()
                    .at_least(1)
                    .collect::<Vec<_>>(),
            )
            .map(|(intensity, conditions)| Weather {
                intensity,
                conditions,
            })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_weather() {
        assert_eq!(
            Weather::parse("+SHRA").unwrap(),
            Weather {
                intensity: WeatherIntensity::Heavy,
                conditions: vec![WeatherCondition::Showers, WeatherCondition::Rain,]
            }
        );
        assert_eq!(
            Weather::parse("VCTS").unwrap(),
            Weather {
                intensity: WeatherIntensity::InVicinity,
                conditions: vec![WeatherCondition::Thunderstorm,]
            }
        );
    }
}
