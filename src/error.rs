use std::{borrow::Cow, fmt};

use annotate_snippets::{renderer::DecorStyle, AnnotationKind, Level, Renderer, Snippet};
use chumsky::prelude::*;
use derive_more::Display;

const NOT_POPULATED: &str = "** not yet populated **";

/// An error when parsing a METAR
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct MetarError<'a> {
    /// The string being parsed
    pub string: &'a str,
    /// The start index of the error
    pub start: usize,
    /// The length of the error'd section
    pub end: usize,
    /// The kind of error that occurred
    pub variant: ErrorVariant,
}

impl std::error::Error for MetarError<'_> {}

impl<'a> chumsky::error::Error<'a, &'a str> for MetarError<'a> {
    fn merge(mut self, other: Self) -> Self {
        if let (
            ErrorVariant::ExpectedFound { expected, .. },
            ErrorVariant::ExpectedFound {
                expected: expected_other,
                ..
            },
        ) = (&mut self.variant, &other.variant)
        {
            for item in expected_other {
                if !expected.contains(item) {
                    expected.push(item.clone());
                }
            }
        }
        self
    }
}

impl<'a> chumsky::error::LabelError<'a, &'a str, chumsky::DefaultExpected<'a, char>>
    for MetarError<'a>
{
    fn expected_found<E: IntoIterator<Item = chumsky::DefaultExpected<'a, char>>>(
        expected: E,
        found: Option<chumsky::util::MaybeRef<'a, char>>,
        span: SimpleSpan,
    ) -> Self {
        MetarError {
            string: NOT_POPULATED,
            start: span.start,
            end: span.end,
            variant: ErrorVariant::ExpectedFound {
                expected: expected
                    .into_iter()
                    .map(|i| match i {
                        chumsky::DefaultExpected::Token(t) => ExpectedNext::Literal {
                            value: (*t).to_string(),
                        },
                        chumsky::DefaultExpected::EndOfInput => ExpectedNext::EndOfInput,
                        _ => ExpectedNext::SomethingElse,
                    })
                    .collect(),
                found: found.map(|inner| *inner),
            },
        }
    }
}

impl<'a> chumsky::error::LabelError<'a, &'a str, chumsky::text::TextExpected<'a, &'a str>>
    for MetarError<'a>
{
    fn expected_found<E: IntoIterator<Item = chumsky::text::TextExpected<'a, &'a str>>>(
        expected: E,
        found: Option<chumsky::util::MaybeRef<'a, char>>,
        span: SimpleSpan,
    ) -> Self {
        MetarError {
            string: NOT_POPULATED,
            start: span.start,
            end: span.end,
            variant: ErrorVariant::ExpectedFound {
                expected: expected
                    .into_iter()
                    .map(|i| match i {
                        chumsky::text::TextExpected::Digit(_) => ExpectedNext::Digits,
                        _ => unimplemented!(),
                    })
                    .collect(),
                found: found.map(|inner| *inner),
            },
        }
    }
}

impl fmt::Display for MetarError<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let report = &[Level::ERROR
            .primary_title(self.variant.to_string())
            .element(
                Snippet::source(self.string).annotation(
                    AnnotationKind::Primary
                        .span(self.start..self.end)
                        .label(self.variant.help()),
                ),
            )];

        let renderer = Renderer::styled().decor_style(DecorStyle::Unicode);

        writeln!(f, "{}", renderer.render(report))
    }
}

/// An error when parsing a METAR
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct OwnedMetarError {
    /// The string being parsed
    pub string: String,
    /// The start index of the error
    pub start: usize,
    /// The length of the error'd section
    pub end: usize,
    /// The kind of error that occurred
    pub variant: ErrorVariant,
}

impl fmt::Display for OwnedMetarError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let report = &[Level::ERROR
            .primary_title(self.variant.to_string())
            .element(
                Snippet::source(&self.string).annotation(
                    AnnotationKind::Primary
                        .span(self.start..self.end)
                        .label(self.variant.help()),
                ),
            )];

        let renderer = Renderer::styled().decor_style(DecorStyle::Unicode);

        writeln!(f, "{}", renderer.render(report))
    }
}

impl MetarError<'_> {
    /// Convert this error into an [`OwnedMetarError`]
    #[must_use]
    pub fn into_owned(&self) -> OwnedMetarError {
        OwnedMetarError {
            string: self.string.to_string(),
            start: self.start,
            end: self.end,
            variant: self.variant.clone(),
        }
    }
}

#[derive(PartialEq, Eq, Clone, Debug, Display)]
pub enum ExpectedNext {
    #[display("\"{value}\"")]
    Literal { value: String },
    #[display("a number")]
    Digits,
    #[display("something else")]
    SomethingElse,
    #[display("end of input")]
    EndOfInput,
}

#[derive(PartialEq, Eq, Clone, Debug, Display)]
#[allow(missing_docs, reason = "self-documenting and with display strings")]
pub enum ErrorVariant {
    // GENERIC //
    #[display(
        "expected one of: {}; {}",
        expected.iter().map(ToString::to_string).collect::<Vec<_>>().join(", "),
        if let Some(found) = found {
            format!(r#"found "{found}""#)
        } else {
            "reached end of input".to_string()
        }
    )]
    ExpectedFound {
        expected: Vec<ExpectedNext>,
        found: Option<char>,
    },

    // DATE //
    #[display("invalid observation date")]
    InvalidDate,
    #[display("invalid observation hour")]
    InvalidHour,
    #[display("invalid observation minute")]
    InvalidMinute,

    // WIND //
    #[display("invalid wind heading")]
    InvalidWindHeading,

    // RVR //
    #[display("invalid runway number in RVR")]
    InvalidRvrRunwayNumber,
    #[display("invalid distance in RVR")]
    InvalidRvrDistance,

    // TREND //
    #[display("data in a trend must be known ahead of time")]
    TrendDataCannotBeUnknown,
}

impl ErrorVariant {
    pub(crate) fn into_err(self, span: SimpleSpan) -> MetarError<'static> {
        MetarError {
            string: NOT_POPULATED,
            start: span.start,
            end: span.end,
            variant: self,
        }
    }

    fn help(&self) -> Cow<'_, str> {
        match self {
            // GENERIC //
            Self::ExpectedFound { expected, .. } => Cow::Owned(format!(
                "must be one of {}",
                expected
                    .iter()
                    .map(ToString::to_string)
                    .collect::<Vec<_>>()
                    .join(", ")
            )),

            // DATE //
            Self::InvalidDate => Cow::Borrowed(
                "the observation date must be a two digit number less than or equal to 31 ",
            ),
            Self::InvalidHour => {
                Cow::Borrowed("the observation date must be a two digit number less than 24")
            }
            Self::InvalidMinute => {
                Cow::Borrowed("the observation date must be a two digit number less than 60")
            }

            // WIND //
            Self::InvalidWindHeading => {
                Cow::Borrowed("the wind heading must be three digits between 000 and 360 inclusive")
            }

            // RVR //
            Self::InvalidRvrRunwayNumber => Cow::Borrowed(
                r#"the runway number must be between 00 and 36, and may be suffixed with "L", "C" or "R""#,
            ),
            Self::InvalidRvrDistance => Cow::Borrowed("the RVR distance must be a 4 digit number"),

            // TREND //
            Self::TrendDataCannotBeUnknown => Cow::Borrowed(
                "trend data cannot be unknown as it isn't reported as a trend if it's unknown!",
            ),
        }
    }
}
