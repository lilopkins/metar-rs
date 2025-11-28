use chumsky::prelude::*;

use crate::{parsers::runway_number, traits::Parsable, Data};

/// Describes contamination on a runway
#[derive(PartialEq, Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct RunwayCondition {
    /// Affected runway number
    pub runway_number: String,
    /// Contamination detail
    pub contamination: RunwayContamination,
    /// Percentage of braking action on the runway
    pub braking_action: Data<u8>,
}

impl Parsable for RunwayCondition {
    fn parser<'src>() -> impl Parser<'src, &'src str, Self, extra::Err<crate::MetarError<'src>>> {
        group((
            runway_number(),
            just("/"),
            RunwayContamination::parser(),
            Data::parser_inline(
                2,
                text::digits(10)
                    .exactly(2)
                    .to_slice()
                    .map(|d: &str| d.parse().unwrap()),
            ),
        ))
        .map(
            |(runway_number, _, contamination, braking_action)| RunwayCondition {
                runway_number,
                contamination,
                braking_action,
            },
        )
    }
}

/// Describes contamination on a runway
#[derive(PartialEq, Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum RunwayContamination {
    /// Contamination is cleared
    Cleared,
    /// Contamination is present
    Present {
        /// Runway deposits
        deposits: Data<RunwayDeposits>,
        /// Runway contamination
        contamination: Data<u8>,
        /// Runway deposit depth, usually in millimetres, but figures
        /// above 90 may have different interpretations
        deposit_depth: Data<u8>,
    },
}

impl Parsable for RunwayContamination {
    fn parser<'src>() -> impl Parser<'src, &'src str, Self, extra::Err<crate::MetarError<'src>>> {
        choice((
            just("CLRD").map(|_| RunwayContamination::Cleared),
            group((
                Data::parser_inline(1, RunwayDeposits::parser()),
                Data::parser_inline(
                    1,
                    text::digits(10)
                        .exactly(1)
                        .to_slice()
                        .map(|d: &str| d.parse().unwrap()),
                ),
                Data::parser_inline(
                    2,
                    text::digits(10)
                        .exactly(2)
                        .to_slice()
                        .map(|d: &str| d.parse().unwrap()),
                ),
            ))
            .map(|(deposits, contamination, deposit_depth)| {
                RunwayContamination::Present {
                    deposits,
                    contamination,
                    deposit_depth,
                }
            }),
        ))
    }
}

/// Describes deposits on a runway
#[derive(PartialEq, Clone, Debug)]
#[allow(missing_docs, reason = "clear what each means")]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum RunwayDeposits {
    ClearAndDry,
    Damp,
    WetOrWaterPatches,
    RimeOrFrostCovered,
    DrySnow,
    WetSnow,
    Slush,
    Ice,
    CompactedOrRolledSnow,
    FrozenRutsOrRidgets,
}

impl Parsable for RunwayDeposits {
    fn parser<'src>() -> impl Parser<'src, &'src str, Self, extra::Err<crate::MetarError<'src>>> {
        choice((
            just("0").map(|_| RunwayDeposits::ClearAndDry),
            just("1").map(|_| RunwayDeposits::Damp),
            just("2").map(|_| RunwayDeposits::WetOrWaterPatches),
            just("3").map(|_| RunwayDeposits::RimeOrFrostCovered),
            just("4").map(|_| RunwayDeposits::DrySnow),
            just("5").map(|_| RunwayDeposits::WetSnow),
            just("6").map(|_| RunwayDeposits::Slush),
            just("7").map(|_| RunwayDeposits::Ice),
            just("8").map(|_| RunwayDeposits::CompactedOrRolledSnow),
            just("9").map(|_| RunwayDeposits::FrozenRutsOrRidgets),
        ))
    }
}
