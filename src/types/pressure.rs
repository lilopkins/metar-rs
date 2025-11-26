use crate::{traits::Parsable, MetarError};

use super::Data;
use chumsky::prelude::*;

#[derive(PartialEq, Copy, Clone, Debug)]
/// Measured air pressure
pub enum Pressure {
    /// Pressure in hectopascals
    Hectopascals(Data<u16>),
    /// Pressure in inches of mercury (inHg)
    InchesOfMercury(Data<f32>),
}

impl Parsable for Pressure {
    fn parser<'src>() -> impl chumsky::Parser<'src, &'src str, Self, extra::Err<MetarError<'src>>> {
        let four_digits = text::digits(10)
            .exactly(4)
            .to_slice()
            .map(|d: &str| d.parse::<u16>().unwrap());

        choice((
            just("Q")
                .then(Data::parser_inline(4, four_digits))
                .map(|(_, d)| Pressure::Hectopascals(d)),
            just("A")
                .then(Data::parser_inline(4, four_digits))
                .map(|(_, d)| Pressure::InchesOfMercury(d.map(|v| (v as f32) / 100.))),
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn valid_hpa() {
        assert_eq!(
            Pressure::parse("Q1013").unwrap(),
            Pressure::Hectopascals(Data::Known(1013))
        );
    }

    #[test]
    fn valid_mmhg() {
        assert_eq!(
            Pressure::parse("A3012").unwrap(),
            Pressure::InchesOfMercury(Data::Known(30.12))
        );
    }

    #[test]
    fn valid_unknown_hpa() {
        assert_eq!(
            Pressure::parse("Q////").unwrap(),
            Pressure::Hectopascals(Data::Unknown)
        );
    }

    #[test]
    fn valid_unknown_mmhg() {
        assert_eq!(
            Pressure::parse("A////").unwrap(),
            Pressure::InchesOfMercury(Data::Unknown)
        );
    }
}
