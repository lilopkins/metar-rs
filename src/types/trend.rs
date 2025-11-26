use chumsky::prelude::*;

use crate::{traits::Parsable, CloudLayer, Data, ErrorVariant, Visibility, Weather, Wind};

/// How is the weather expected to change in the near future?
#[derive(PartialEq, Debug, Clone)]
#[allow(missing_docs)]
pub enum Trend {
    NoSignificantChanges,
    NoSignificantWeather,
    Becoming(TrendNewCondition),
    Temporarily(TrendNewCondition),
}

impl Parsable for Trend {
    fn parser<'src>() -> impl Parser<'src, &'src str, Self, extra::Err<crate::MetarError<'src>>> {
        choice((
            just("NOSIG").map(|_| Trend::NoSignificantChanges),
            just("NSW").map(|_| Trend::NoSignificantWeather),
            just("BECMG ")
                .then(TrendNewCondition::parser())
                .map(|(_, cond)| Trend::Becoming(cond)),
            just("TEMPO ")
                .then(TrendNewCondition::parser())
                .map(|(_, cond)| Trend::Temporarily(cond)),
        ))
    }
}

/// New conditions apply
#[derive(PartialEq, Debug, Clone)]
pub struct TrendNewCondition {
    /// The time from which conditions apply
    pub time: Option<TrendTime>,
    /// New wind values, if specified
    pub wind: Option<Wind>,
    /// New visibility values, if specified
    pub visibility: Option<Visibility>,
    /// New weather conditions, if specified
    pub weather: Vec<Weather>,
    /// New cloud layers, if specified
    pub cloud: Vec<CloudLayer>,
}

impl Parsable for TrendNewCondition {
    fn parser<'src>() -> impl Parser<'src, &'src str, Self, extra::Err<crate::MetarError<'src>>> {
        let whitespace = text::inline_whitespace();
        let whitespace_1plus = text::inline_whitespace().at_least(1);

        group((
            TrendTime::parser()
                .map(|v| Some(v))
                .or(empty().map(|_| None)),
            whitespace,
            Wind::parser().map(|v| Some(v)).or(empty().map(|_| None)),
            whitespace,
            <Data<Visibility> as Parsable>::parser()
                .try_map(|v, span| match v {
                    Data::Known(v) => Ok(Some(v)),
                    Data::Unknown => Err(ErrorVariant::TrendDataCannotBeUnknown.into_err(span)),
                })
                .or(empty().map(|_| None)),
            whitespace,
            choice((
                just("NSW").map(|_| vec![]),
                Weather::parser()
                    .separated_by(whitespace_1plus)
                    .collect::<Vec<_>>(),
            )),
            whitespace,
            CloudLayer::parser()
                .separated_by(whitespace_1plus)
                .collect::<Vec<_>>(),
        ))
        .map(
            |(time, _, wind, _, visibility, _, weather, _, cloud)| TrendNewCondition {
                time,
                wind,
                visibility,
                weather,
                cloud,
            },
        )
    }
}

/// The time at which conditions change
#[derive(PartialEq, Debug, Clone, Copy)]
pub enum TrendTime {
    /// From a particular time, in 24 hour format, eg. 1345
    From(u16),
    /// Until a particular time, in 24 hour format, eg. 1345
    Until(u16),
    /// At a particular time, in 24 hour format, eg. 1345
    At(u16),
}

impl Parsable for TrendTime {
    fn parser<'src>() -> impl Parser<'src, &'src str, Self, extra::Err<crate::MetarError<'src>>> {
        let time = text::digits(10)
            .exactly(4)
            .to_slice()
            .map(|d: &str| d.parse().unwrap());
        choice((
            just("FM").then(time).map(|(_, time)| TrendTime::From(time)),
            just("TL")
                .then(time)
                .map(|(_, time)| TrendTime::Until(time)),
            just("AT").then(time).map(|(_, time)| TrendTime::At(time)),
        ))
    }
}
