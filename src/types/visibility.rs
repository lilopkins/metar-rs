use chumsky::prelude::*;

use crate::{parsers::some_whitespace, traits::Parsable, Data};

#[derive(PartialEq, Copy, Clone, Debug)]
#[allow(missing_docs, reason = "clear what they are!")]
/// A compass direction
pub enum CompassDirection {
    North,
    NorthEast,
    East,
    SouthEast,
    South,
    SouthWest,
    West,
    NorthWest,
}

impl Parsable for CompassDirection {
    fn parser<'src>() -> impl Parser<'src, &'src str, Self, extra::Err<crate::MetarError<'src>>> {
        choice((
            just("NE").map(|_| CompassDirection::NorthEast),
            just("NW").map(|_| CompassDirection::NorthWest),
            just("N").map(|_| CompassDirection::North),
            just("SE").map(|_| CompassDirection::SouthEast),
            just("SW").map(|_| CompassDirection::SouthWest),
            just("S").map(|_| CompassDirection::South),
            just("E").map(|_| CompassDirection::East),
            just("W").map(|_| CompassDirection::West),
        ))
    }
}

#[derive(PartialEq, Copy, Clone, Debug)]
/// Horizontal visibility
pub enum Visibility {
    /// Visibility OK
    CAVOK,
    /// Metres
    Metres(u16),
    /// Statute miles, usually used in the US
    StatuteMiles(f32),
}

impl Parsable for Visibility {
    fn parser<'src>(
    ) -> impl chumsky::Parser<'src, &'src str, Self, chumsky::extra::Err<crate::MetarError<'src>>>
    {
        choice((
            just("CAVOK").map(|_| Visibility::CAVOK),
            // To compensate for a technically incorrect placement:
            just("SKC").map(|_| Visibility::CAVOK),
            // Metres
            text::digits(10)
                .exactly(4)
                .to_slice()
                .map(|digits: &str| Visibility::Metres(digits.parse().unwrap())),
            // Whole miles
            text::digits(10)
                .at_least(1)
                .at_most(2)
                .to_slice()
                .then_ignore(just("SM"))
                .map(|digits: &str| Visibility::StatuteMiles(digits.parse().unwrap())),
            // Fractional miles
            group((
                text::digits(10).exactly(1).to_slice(),
                just("/"),
                text::digits(10).exactly(1).to_slice(),
                just("SM"),
            ))
            .map(|(numerator, _, denominator, _): (&str, &str, &str, &str)| {
                let numerator: f32 = numerator.parse().unwrap();
                let denominator: f32 = denominator.parse().unwrap();
                Visibility::StatuteMiles(numerator / denominator)
            }),
            // Whole and fractional miles
            group((
                text::digits(10).at_least(1).at_most(2).to_slice(),
                some_whitespace(),
                text::digits(10).exactly(1).to_slice(),
                just("/"),
                text::digits(10).exactly(1).to_slice(),
                just("SM"),
            ))
            .map(
                |(whole_part, (), numerator, _, denominator, _): (
                    &str,
                    (),
                    &str,
                    &str,
                    &str,
                    &str,
                )| {
                    let whole_part: f32 = whole_part.parse().unwrap();
                    let numerator: f32 = numerator.parse().unwrap();
                    let denominator: f32 = denominator.parse().unwrap();
                    Visibility::StatuteMiles(whole_part + numerator / denominator)
                },
            ),
        ))
    }
}

impl Parsable for (Option<CompassDirection>, Data<Visibility>) {
    fn parser<'src>() -> impl Parser<'src, &'src str, Self, extra::Err<crate::MetarError<'src>>> {
        group((
            Data::parser_inline(4, Visibility::parser()),
            choice((
                just("NDV").map(|_| None),
                CompassDirection::parser().map(Some),
            )),
        ))
        .map(|(vis, dir)| (dir, vis))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn valid_visibility() {
        assert_eq!(Visibility::parse("CAVOK").unwrap(), Visibility::CAVOK);
        assert_eq!(Visibility::parse("5000").unwrap(), Visibility::Metres(5000));
        assert_eq!(
            Visibility::parse("3SM").unwrap(),
            Visibility::StatuteMiles(3.)
        );
        assert_eq!(
            Visibility::parse("1/4SM").unwrap(),
            Visibility::StatuteMiles(0.25)
        );
        assert_eq!(
            Visibility::parse("3 1/2SM").unwrap(),
            Visibility::StatuteMiles(3.5)
        );
    }
}
