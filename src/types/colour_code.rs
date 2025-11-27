use chumsky::prelude::*;

use crate::traits::Parsable;

/// Military airport colour code
#[derive(PartialEq, Eq, Clone, Debug)]
pub enum ColourCode {
    /// 2500ft cloud base, 8000m visibility
    Blue,
    /// 1500ft cloud base, 5000m visibility
    White,
    /// 700ft cloud base, 3700m visibility
    Green,
    /// 300ft cloud base, 1600m visibility
    Yellow,
    /// 200ft cloud base, 800m visibility
    Amber,
    /// Less than amber
    Red,
}

impl Parsable for ColourCode {
    fn parser<'src>() -> impl Parser<'src, &'src str, Self, extra::Err<crate::MetarError<'src>>> {
        choice((
            just("BLU").map(|_| ColourCode::Blue),
            just("WHT").map(|_| ColourCode::White),
            just("GRN").map(|_| ColourCode::Green),
            just("YLO").map(|_| ColourCode::Yellow),
            just("AMB").map(|_| ColourCode::Amber),
            just("RED").map(|_| ColourCode::Red),
        ))
    }
}
