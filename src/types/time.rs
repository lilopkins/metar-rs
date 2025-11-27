use crate::{error::ErrorVariant, traits::Parsable, MetarError};

use chumsky::prelude::*;

#[derive(PartialEq, Eq, Copy, Clone, Debug)]
/// A struct to store time as it is represented in a METAR
pub struct Time {
    /// The date the METAR was made
    pub date: u8,
    /// The hour the METAR was made
    pub hour: u8,
    /// The minute the METAR was made
    pub minute: u8,
}

impl Parsable for Time {
    fn parser<'src>() -> impl chumsky::Parser<'src, &'src str, Self, extra::Err<MetarError<'src>>> {
        let two_digits = text::digits(10).exactly(2).to_slice();

        group((two_digits, two_digits, two_digits, just("Z"))).try_map(
            |(d, h, m, _): (&str, &str, &str, &str), span| {
                let date = d
                    .parse()
                    .map_err(|_| ErrorVariant::InvalidDate.into_err(span))?;
                if date > 31 {
                    return Err(ErrorVariant::InvalidDate.into_err(span));
                }

                let hour = h
                    .parse()
                    .map_err(|_| ErrorVariant::InvalidHour.into_err(span))?;
                if hour >= 24 {
                    return Err(ErrorVariant::InvalidHour.into_err(span));
                }

                let minute = m
                    .parse()
                    .map_err(|_| ErrorVariant::InvalidMinute.into_err(span))?;
                if minute >= 60 {
                    return Err(ErrorVariant::InvalidMinute.into_err(span));
                }

                Ok(Time { date, hour, minute })
            },
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn valid_time() {
        assert_eq!(
            Time::parse("010101Z").unwrap(),
            Time {
                date: 1,
                hour: 1,
                minute: 1,
            }
        );
    }

    #[test]
    fn invalid_date() {
        assert!(Time::parse("320101Z").is_err());
    }

    #[test]
    fn invalid_hour() {
        assert!(Time::parse("012401Z").is_err());
    }

    #[test]
    fn invalid_minute() {
        assert!(Time::parse("010160Z").is_err());
    }
}
