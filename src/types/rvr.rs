use chumsky::prelude::*;

use crate::{parsers::runway_number, traits::Parsable, Data, ErrorVariant};

/// The visibility measured for a specific runway.
#[derive(PartialEq, Eq, Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct RunwayVisualRange {
    /// The runway this measurement applies to
    pub runway: String,
    /// The visibility for this runway
    pub value: Data<RvrValue>,
    /// The visibility unit
    pub unit: RvrUnit,
    /// How is the RVR trending?
    pub trend: Data<RvrTrend>,
}

impl Parsable for RunwayVisualRange {
    fn parser<'src>() -> impl Parser<'src, &'src str, Self, extra::Err<crate::MetarError<'src>>> {
        group((
            runway_number(),
            just("/"),
            Data::parser_inline(4, RvrValue::parser()),
            RvrUnit::parser(),
            just("/").map(|_| ()).or(empty()),
            Data::parser_inline(1, RvrTrend::parser()),
        ))
        .map(|(runway, _, value, unit, (), trend)| RunwayVisualRange {
            runway,
            value,
            unit,
            trend,
        })
    }
}

/// The visibility value
#[derive(PartialEq, Eq, Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
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
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
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
            rvr_vis.map(RvrValueInner::Exactly),
        ))
    }
}

/// The unit of measurement
#[derive(PartialEq, Eq, Copy, Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
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
            empty().map(|()| RvrUnit::Metres),
        ))
    }
}

/// How is the RVR trending?
#[derive(PartialEq, Eq, Copy, Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
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
            just("N").map(|_| RvrTrend::None),
            empty().map(|()| RvrTrend::None),
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
                value: Data::Known(RvrValue::Single(RvrValueInner::GreaterThan(1500))),
                unit: RvrUnit::Metres,
                trend: Data::Known(RvrTrend::None),
            }
        );
        assert_eq!(
            RunwayVisualRange::parse("R25L/1800V3000FT").unwrap(),
            RunwayVisualRange {
                runway: "25L".to_string(),
                value: Data::Known(RvrValue::Between(
                    RvrValueInner::Exactly(1800),
                    RvrValueInner::Exactly(3000)
                )),
                unit: RvrUnit::Feet,
                trend: Data::Known(RvrTrend::None),
            }
        );
    }
}
