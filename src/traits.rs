use chumsky::prelude::*;

use crate::MetarError;

/// A trait for all parsable structs to inherit.
///
/// If a struct inherits this trait, it is possible to attempt to  parse
/// that struct from a string.
pub trait Parsable
where
    Self: Sized,
{
    /// Build a parser for this type.
    fn parser<'src>() -> impl Parser<'src, &'src str, Self, extra::Err<MetarError<'src>>>;

    /// Attempt to parse a string into [`Self`].
    ///
    /// # Errors
    ///
    /// Returns a list of errors encountered during parsing.
    fn parse(input: &str) -> Result<Self, Vec<MetarError<'_>>> {
        Self::parser().parse(input).into_result()
    }
}
