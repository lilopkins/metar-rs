use chumsky::prelude::*;

use crate::ErrorVariant;

/// Parse a runway number
pub(crate) fn runway_number<'src>(
) -> impl Parser<'src, &'src str, String, extra::Err<crate::MetarError<'src>>> {
    group((
        just("R"),
        text::digits(10)
            .at_least(1)
            .at_most(2)
            .to_slice()
            .try_map(|d: &str, span| {
                if !d.parse::<u8>().is_ok_and(|v| v <= 36 || v == 88) {
                    return Err(ErrorVariant::InvalidRvrRunwayNumber.into_err(span));
                }
                Ok(d)
            }),
        choice((
            just("L").map(|_| "L"),
            just("C").map(|_| "C"),
            just("R").map(|_| "R"),
            empty().map(|()| ""),
        )),
    ))
    .map(|(_, rwy, suffix)| format!("{rwy}{suffix}"))
}

/// Match and parse any whitespace, including none
pub(crate) fn any_whitespace<'src>(
) -> impl Parser<'src, &'src str, (), extra::Err<crate::MetarError<'src>>> {
    text::inline_whitespace().or(end())
}

/// Match and parse some whitespace, demanding at least one character of whitespace
pub(crate) fn some_whitespace<'src>(
) -> impl Parser<'src, &'src str, (), extra::Err<crate::MetarError<'src>>> {
    text::inline_whitespace().at_least(1).or(end())
}
