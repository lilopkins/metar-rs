use chumsky::prelude::*;

use crate::{traits::Parsable, Data, MetarError};

/// The wind speed
#[derive(PartialEq, Eq, Copy, Clone, Debug, Hash)]
pub enum WindSpeed {
    /// Nautical miles per hour
    Knots {
        /// The wind speed
        speed: Data<u32>,
        /// The wind gusts speed
        gusting: Option<Data<u32>>,
    },
    /// Metres per second
    MetresPerSecond {
        /// The wind speed
        speed: Data<u32>,
        /// The wind gusts speed
        gusting: Option<Data<u32>>,
    },
    /// Kilometres per hour
    KilometresPerHour {
        /// The wind speed
        speed: Data<u32>,
        /// The wind gusts speed
        gusting: Option<Data<u32>>,
    },
    /// Wind speed is greater than 100 knots, 100 m/s or 200 kph
    Greater,
}

impl Parsable for WindSpeed {
    fn parser<'src>() -> impl chumsky::Parser<'src, &'src str, Self, extra::Err<MetarError<'src>>> {
        choice((
            // Greater than speeds
            just("P99KT").map(|_| WindSpeed::Greater),
            just("P99MPS").map(|_| WindSpeed::Greater),
            just("P199KPH").map(|_| WindSpeed::Greater),
            // Knots
            just("//KT").map(|_| WindSpeed::Knots {
                speed: Data::Unknown,
                gusting: None,
            }),
            group((
                text::digits(10).exactly(2).to_slice(),
                choice((
                    just("G//").map(|_| Some(Data::Unknown)),
                    just("G")
                        .then(text::digits(10).exactly(2).to_slice())
                        .map(|(_, gust): (&str, &str)| Some(Data::Known(gust.parse().unwrap()))),
                    empty().map(|()| None),
                )),
                just("KT"),
            ))
            .map(|(spd, gusting, _): (&str, Option<Data<u32>>, &str)| {
                WindSpeed::Knots {
                    speed: Data::Known(spd.parse().unwrap()),
                    gusting,
                }
            }),
            // MPS
            just("//MPS").map(|_| WindSpeed::MetresPerSecond {
                speed: Data::Unknown,
                gusting: None,
            }),
            group((
                text::digits(10).exactly(2).to_slice(),
                choice((
                    just("G//").map(|_| Some(Data::Unknown)),
                    just("G")
                        .then(text::digits(10).exactly(2).to_slice())
                        .map(|(_, gust): (&str, &str)| Some(Data::Known(gust.parse().unwrap()))),
                    empty().map(|()| None),
                )),
                just("MPS"),
            ))
            .map(|(spd, gusting, _): (&str, Option<Data<u32>>, &str)| {
                WindSpeed::MetresPerSecond {
                    speed: Data::Known(spd.parse().unwrap()),
                    gusting,
                }
            }),
            // KPH
            just("//KPH").map(|_| WindSpeed::KilometresPerHour {
                speed: Data::Unknown,
                gusting: None,
            }),
            group((
                text::digits(10).exactly(3).to_slice(),
                choice((
                    just("G//").map(|_| Some(Data::Unknown)),
                    just("G")
                        .then(text::digits(10).exactly(3).to_slice())
                        .map(|(_, gust): (&str, &str)| Some(Data::Known(gust.parse().unwrap()))),
                    empty().map(|()| None),
                )),
                just("KPH"),
            ))
            .map(|(spd, gusting, _): (&str, Option<Data<u32>>, &str)| {
                WindSpeed::KilometresPerHour {
                    speed: Data::Known(spd.parse().unwrap()),
                    gusting,
                }
            }),
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn valid_kts() {
        assert_eq!(
            WindSpeed::parse("//KT").unwrap(),
            WindSpeed::Knots {
                speed: Data::Unknown,
                gusting: None
            }
        );
        assert_eq!(
            WindSpeed::parse("00KT").unwrap(),
            WindSpeed::Knots {
                speed: Data::Known(0),
                gusting: None
            }
        );
        assert_eq!(
            WindSpeed::parse("40G60KT").unwrap(),
            WindSpeed::Knots {
                speed: Data::Known(40),
                gusting: Some(Data::Known(60))
            }
        );
        assert_eq!(
            WindSpeed::parse("40G//KT").unwrap(),
            WindSpeed::Knots {
                speed: Data::Known(40),
                gusting: Some(Data::Unknown)
            }
        );
        assert_eq!(
            WindSpeed::parse("99KT").unwrap(),
            WindSpeed::Knots {
                speed: Data::Known(99),
                gusting: None
            }
        );
        assert_eq!(WindSpeed::parse("P99KT").unwrap(), WindSpeed::Greater);
    }

    #[test]
    fn valid_mps() {
        assert_eq!(
            WindSpeed::parse("//MPS").unwrap(),
            WindSpeed::MetresPerSecond {
                speed: Data::Unknown,
                gusting: None
            }
        );
        assert_eq!(
            WindSpeed::parse("00MPS").unwrap(),
            WindSpeed::MetresPerSecond {
                speed: Data::Known(0),
                gusting: None
            }
        );
        assert_eq!(
            WindSpeed::parse("40G60MPS").unwrap(),
            WindSpeed::MetresPerSecond {
                speed: Data::Known(40),
                gusting: Some(Data::Known(60))
            }
        );
        assert_eq!(
            WindSpeed::parse("40G//MPS").unwrap(),
            WindSpeed::MetresPerSecond {
                speed: Data::Known(40),
                gusting: Some(Data::Unknown)
            }
        );
        assert_eq!(
            WindSpeed::parse("99MPS").unwrap(),
            WindSpeed::MetresPerSecond {
                speed: Data::Known(99),
                gusting: None
            }
        );
        assert_eq!(WindSpeed::parse("P99MPS").unwrap(), WindSpeed::Greater);
    }

    #[test]
    fn valid_kph() {
        assert_eq!(
            WindSpeed::parse("//KPH").unwrap(),
            WindSpeed::KilometresPerHour {
                speed: Data::Unknown,
                gusting: None
            }
        );
        assert_eq!(
            WindSpeed::parse("000KPH").unwrap(),
            WindSpeed::KilometresPerHour {
                speed: Data::Known(0),
                gusting: None
            }
        );
        assert_eq!(
            WindSpeed::parse("040G060KPH").unwrap(),
            WindSpeed::KilometresPerHour {
                speed: Data::Known(40),
                gusting: Some(Data::Known(60))
            }
        );
        assert_eq!(
            WindSpeed::parse("040G//KPH").unwrap(),
            WindSpeed::KilometresPerHour {
                speed: Data::Known(40),
                gusting: Some(Data::Unknown)
            }
        );
        assert_eq!(
            WindSpeed::parse("199KPH").unwrap(),
            WindSpeed::KilometresPerHour {
                speed: Data::Known(199),
                gusting: None
            }
        );
        assert_eq!(WindSpeed::parse("P199KPH").unwrap(), WindSpeed::Greater);
    }
}
