use chumsky::prelude::*;

use crate::{
    parsers::{runway_number, whitespace_1plus},
    traits::Parsable,
};

/// A windshear warnings
#[derive(PartialEq, Clone, Debug)]
pub enum WindshearWarnings {
    /// All runways are affected by windshear
    AllRunways,
    /// Only specific runways are affected by windshear
    SpecificRunways(Vec<WindshearGroup>),
}

impl Parsable for WindshearWarnings {
    fn parser<'src>() -> impl Parser<'src, &'src str, Self, extra::Err<crate::MetarError<'src>>> {
        choice((
            just("WS ALL RWY").map(|_| WindshearWarnings::AllRunways),
            WindshearGroup::parser()
                .separated_by(whitespace_1plus())
                .collect::<Vec<_>>()
                .map(WindshearWarnings::SpecificRunways),
        ))
    }
}

/// A runway affected by windshear
#[derive(PartialEq, Clone, Debug)]
pub struct WindshearGroup {
    /// The runway number
    pub runway_number: String,
}

impl Parsable for WindshearGroup {
    fn parser<'src>() -> impl Parser<'src, &'src str, Self, extra::Err<crate::MetarError<'src>>> {
        group((
            just("WS"),
            text::inline_whitespace().at_least(1),
            runway_number(),
        ))
        .map(|(_, (), runway_number)| WindshearGroup { runway_number })
    }
}
