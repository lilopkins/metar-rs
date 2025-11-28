use chumsky::prelude::*;

use crate::MetarError;

/// Data that is provided in a metar which might be unknown.
/// Note that this differs from an `Option<T>` field which is used when data
/// might not be given at all. In the cases where `Data<T>` is used, data is
/// usually given but has been replaced in the METAR by slashes, indicating
/// that it is not known.
#[derive(PartialEq, Eq, Copy, Clone, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Data<T> {
    /// The data is known and given
    Known(T),
    /// The data isn't or cannot be known
    Unknown,
}

impl<T> Data<T> {
    /// Gets a new `Data` that has a `&T` inside it.
    pub fn as_ref(&self) -> Data<&T> {
        match *self {
            Data::Known(ref v) => Data::Known(v),
            Data::Unknown => Data::Unknown,
        }
    }

    /// Gets a new `Data` that has a `&mut T` inside it.
    pub fn as_mut(&mut self) -> Data<&mut T> {
        match *self {
            Data::Known(ref mut v) => Data::Known(v),
            Data::Unknown => Data::Unknown,
        }
    }

    /// Unwraps the inner data type, panics otherwise.
    ///
    /// # Panics
    ///
    /// Panics if the data is [`Data::Unknown`].
    pub fn unwrap(self) -> T {
        match self {
            Data::Known(v) => v,
            Data::Unknown => panic!("cannot unwrap unknown data"),
        }
    }

    /// Apply a function to the contained value in this [`Data`].
    pub(crate) fn map<F, O>(self, f: F) -> Data<O>
    where
        F: FnOnce(T) -> O,
    {
        match self {
            Self::Unknown => Data::Unknown,
            Self::Known(val) => Data::Known(f(val)),
        }
    }
}

impl<T> Data<T> {
    /// Generate a parser for this Data<T>, with a given parser to get `T`.
    pub(crate) fn parser_inline<'src>(
        num_slashes: usize,
        parser: impl Parser<'src, &'src str, T, extra::Err<MetarError<'src>>>,
    ) -> impl Parser<'src, &'src str, Data<T>, extra::Err<MetarError<'src>>> {
        let slashes = just("/").repeated().exactly(num_slashes);
        choice((
            slashes.map(|()| Data::Unknown),
            parser.map(|v| Data::Known(v)),
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_data() {
        assert_eq!(
            Data::parser_inline(4, just("ABCD").map(|_| true))
                .parse("ABCD")
                .into_result()
                .unwrap(),
            Data::Known(true)
        );
        assert_eq!(
            Data::parser_inline(4, just("ABCD").map(|_| true))
                .parse("////")
                .into_result()
                .unwrap(),
            Data::Unknown
        );
    }
}
