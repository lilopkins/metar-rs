use chumsky::prelude::*;

use crate::traits::Parsable;

use super::Data;
use super::WindDirection;
use super::WindSpeed;

#[derive(PartialEq, Eq, Copy, Clone, Debug)]
/// Wind information.
pub enum Wind {
    /// Calm winds are at 0 kts
    Calm,
    /// Winds are present. More information is available in the struct.
    Present {
        /// The wind direction, in degrees
        dir: WindDirection,
        /// The current wind speed
        speed: WindSpeed,
        /// The direction the wind may be varying between, smaller always comes first
        varying: Option<(Data<u32>, Data<u32>)>,
    },
}

impl Parsable for Wind {
    fn parser<'src>() -> impl Parser<'src, &'src str, Self, extra::Err<crate::MetarError<'src>>> {
        choice((
            just("CALM").map(|_| Wind::Calm),
            group((
                WindDirection::parser(),
                WindSpeed::parser(),
                text::inline_whitespace(),
                choice((
                    group((WindDirection::parser(), just("V"), WindDirection::parser()))
                        .map(|(from, _, to)| Some((from.unwrap_heading(), to.unwrap_heading()))),
                    empty().map(|()| None),
                )),
            ))
            .map(|(dir, speed, (), varying)| Wind::Present {
                dir,
                speed,
                varying,
            }),
        ))
    }
}
