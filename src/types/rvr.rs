use chumsky::prelude::*;

use crate::{parsers::runway_number, traits::Parsable, ErrorVariant};

/// The visibility measured for a specific runway.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct RunwayVisualRange {
    /// The runway this measurement applies to
    pub runway: String,
    /// The visibility for this runway
    pub value: RvrValue,
    /// The visibility unit
    pub unit: RvrUnit,
    /// How is the RVR trending?
    pub trend: RvrTrend,
}

impl Parsable for RunwayVisualRange {
    fn parser<'src>() -> impl Parser<'src, &'src str, Self, extra::Err<crate::MetarError<'src>>> {
        group((
            runway_number(),
            just("/"),
            RvrValue::parser(),
            RvrUnit::parser(),
            RvrTrend::parser(),
        ))
        .map(|(runway, _, value, unit, trend)| RunwayVisualRange {
            runway,
            value,
            unit,
            trend,
        })
    }
}

/// The visibility value
#[derive(PartialEq, Eq, Clone, Debug)]
pub enum RvrValue {
    /// There is a single value specified
    Single(RvrValueInner),
    /// The value is between
    Between(RvrValueInner, RvrValueInner),
}

impl Parsable for RvrValue {
    fn parser<'src>() -> impl Parser<'src, &'src str, Self, extra::Err<crate::MetarError<'src>>> {
        RvrValueInner::parser()
            .separated_by(just("V"))
            .at_least(1)
            .at_most(2)
            .collect::<Vec<_>>()
            .map(|vals| {
                if vals.len() == 1 {
                    RvrValue::Single(vals.first().unwrap().clone())
                } else {
                    let mut iter = vals.into_iter();
                    RvrValue::Between(iter.next().unwrap(), iter.next().unwrap())
                }
            })
    }
}

/// The visibility value
#[derive(PartialEq, Eq, Clone, Debug)]
pub enum RvrValueInner {
    /// The value is exactly
    Exactly(u32),
    /// The value is greater than
    GreaterThan(u32),
    /// The value is less than
    LessThan(u32),
}

impl Parsable for RvrValueInner {
    fn parser<'src>() -> impl Parser<'src, &'src str, Self, extra::Err<crate::MetarError<'src>>> {
        let rvr_vis = text::digits(10)
            .exactly(4)
            .to_slice()
            .try_map(|d: &str, span| {
                d.parse::<u32>()
                    .map_err(|_| ErrorVariant::InvalidRvrDistance.into_err(span))
            });

        choice((
            just("P")
                .then(rvr_vis)
                .map(|(_, vis)| RvrValueInner::GreaterThan(vis)),
            just("M")
                .then(rvr_vis)
                .map(|(_, vis)| RvrValueInner::LessThan(vis)),
            rvr_vis.map(|vis| RvrValueInner::Exactly(vis)),
        ))
    }
}

/// The unit of measurement
#[derive(PartialEq, Eq, Copy, Clone, Debug)]
pub enum RvrUnit {
    /// Metres
    Metres,
    /// Feet
    Feet,
}

impl Parsable for RvrUnit {
    fn parser<'src>() -> impl Parser<'src, &'src str, Self, extra::Err<crate::MetarError<'src>>> {
        choice((
            just("FT").map(|_| RvrUnit::Feet),
            empty().map(|_| RvrUnit::Metres),
        ))
    }
}

/// How is the RVR trending?
#[derive(PartialEq, Eq, Copy, Clone, Debug)]
pub enum RvrTrend {
    /// Trending upwards
    Upwards,
    /// Trending downwards
    Downwards,
    /// No change
    None,
}

impl Parsable for RvrTrend {
    fn parser<'src>() -> impl Parser<'src, &'src str, Self, extra::Err<crate::MetarError<'src>>> {
        choice((
            just("U").map(|_| RvrTrend::Upwards),
            just("D").map(|_| RvrTrend::Downwards),
            empty().map(|_| RvrTrend::None),
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rvr() {
        assert_eq!(
            RunwayVisualRange::parse("R24L/P1500").unwrap(),
            RunwayVisualRange {
                runway: "24L".to_string(),
                value: RvrValue::Single(RvrValueInner::GreaterThan(1500)),
                unit: RvrUnit::Metres,
                trend: RvrTrend::None,
            }
        );
        assert_eq!(
            RunwayVisualRange::parse("R25L/1800V3000FT").unwrap(),
            RunwayVisualRange {
                runway: "25L".to_string(),
                value: RvrValue::Between(
                    RvrValueInner::Exactly(1800),
                    RvrValueInner::Exactly(3000)
                ),
                unit: RvrUnit::Feet,
                trend: RvrTrend::None,
            }
        );
    }
}
