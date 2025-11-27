use chumsky::prelude::*;

use crate::{parsers::temperature, traits::Parsable, Data};

/// Describes the condition of the sea
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct SeaCondition {
    /// Sea temperature
    pub temperature: Data<i32>,
    /// Sea condition
    pub condition: Data<SeaConditionInner>,
}

impl Parsable for SeaCondition {
    fn parser<'src>() -> impl Parser<'src, &'src str, Self, extra::Err<crate::MetarError<'src>>> {
        group((
            just("W"),
            Data::parser_inline(2, temperature()),
            just("/"),
            Data::parser_inline(2, SeaConditionInner::parser()),
        ))
        .map(|(_, temperature, _, condition)| SeaCondition {
            temperature,
            condition,
        })
    }
}

/// Sea condition
#[derive(PartialEq, Eq, Clone, Debug)]
pub enum SeaConditionInner {
    /// Predefined sea state
    State(Data<SeaState>),
    /// Significant wave height in decimeters
    WaveHeight(Data<u32>),
}

impl Parsable for SeaConditionInner {
    fn parser<'src>() -> impl Parser<'src, &'src str, Self, extra::Err<crate::MetarError<'src>>> {
        choice((
            just("S")
                .then(Data::parser_inline(1, SeaState::parser()))
                .map(|(_, s)| SeaConditionInner::State(s)),
            just("H")
                .then(Data::parser_inline(
                    2,
                    text::digits(10)
                        .at_least(1)
                        .at_most(3)
                        .to_slice()
                        .map(|d: &str| d.parse::<u32>().unwrap()),
                ))
                .map(|(_, h)| SeaConditionInner::WaveHeight(h)),
        ))
    }
}

/// The state of the sea
#[derive(PartialEq, Eq, Clone, Debug)]
#[allow(missing_docs)]
pub enum SeaState {
    CalmGlassy,
    CalmRippled,
    Smooth,
    Slight,
    Moderate,
    Rough,
    VeryRough,
    High,
    VeryHigh,
    Phenomenal,
}

impl Parsable for SeaState {
    fn parser<'src>() -> impl Parser<'src, &'src str, Self, extra::Err<crate::MetarError<'src>>> {
        choice((
            just("0").map(|_| SeaState::CalmGlassy),
            just("1").map(|_| SeaState::CalmRippled),
            just("2").map(|_| SeaState::Smooth),
            just("3").map(|_| SeaState::Slight),
            just("4").map(|_| SeaState::Moderate),
            just("5").map(|_| SeaState::Rough),
            just("6").map(|_| SeaState::VeryRough),
            just("7").map(|_| SeaState::High),
            just("8").map(|_| SeaState::VeryHigh),
            just("9").map(|_| SeaState::Phenomenal),
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sea_condition() {
        assert_eq!(
            SeaCondition::parse("W15/S2").unwrap(),
            SeaCondition {
                temperature: Data::Known(15),
                condition: Data::Known(SeaConditionInner::State(Data::Known(SeaState::Smooth))),
            }
        );
        assert_eq!(
            SeaCondition::parse("W15/H123").unwrap(),
            SeaCondition {
                temperature: Data::Known(15),
                condition: Data::Known(SeaConditionInner::WaveHeight(Data::Known(123))),
            }
        );
    }
}
