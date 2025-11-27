use chumsky::prelude::*;

use crate::ErrorVariant;

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

pub(crate) fn whitespace<'src>(
) -> impl Parser<'src, &'src str, (), extra::Err<crate::MetarError<'src>>> {
    text::inline_whitespace().or(end())
}

pub(crate) fn whitespace_1plus<'src>(
) -> impl Parser<'src, &'src str, (), extra::Err<crate::MetarError<'src>>> {
    text::inline_whitespace().at_least(1).or(end())
}
