use chumsky::prelude::*;

use crate::{traits::Parsable, Data, ErrorVariant, MetarError};

/// A representation of wind direction
#[derive(PartialEq, Eq, Copy, Clone, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum WindDirection {
    /// A heading defining wind direction
    Heading(Data<u32>),
    /// Wind direction is variable
    Variable,
}

impl WindDirection {
    pub(crate) fn unwrap_heading(self) -> Data<u32> {
        match self {
            Self::Heading(v) => v,
            Self::Variable => panic!(),
        }
    }
}

impl Parsable for WindDirection {
    fn parser<'src>() -> impl chumsky::Parser<'src, &'src str, Self, extra::Err<MetarError<'src>>> {
        choice((
            just("VRB").map(|_| WindDirection::Variable),
            just("///").map(|_| WindDirection::Heading(Data::Unknown)),
            text::digits(10)
                .exactly(3)
                .to_slice()
                .try_map(|hdg: &str, span| {
                    let hdg = hdg
                        .parse()
                        .map_err(|_| ErrorVariant::InvalidWindHeading.into_err(span))?;
                    if hdg > 360 {
                        return Err(ErrorVariant::InvalidWindHeading.into_err(span));
                    }
                    Ok(WindDirection::Heading(Data::Known(hdg)))
                }),
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn valid_heading() {
        assert_eq!(
            WindDirection::parse("015").unwrap(),
            WindDirection::Heading(Data::Known(15))
        );
    }

    #[test]
    fn valid_unknown() {
        assert_eq!(
            WindDirection::parse("///").unwrap(),
            WindDirection::Heading(Data::Unknown)
        );
    }

    #[test]
    fn valid_variable() {
        assert_eq!(
            WindDirection::parse("VRB").unwrap(),
            WindDirection::Variable
        );
    }
}
