#![deny(missing_docs)]
#![deny(unsafe_code)]
#![deny(clippy::pedantic)]

//! # METAR parsing library for Rust
//!
//! ## Quick usage
//!
//! This simple usage will print out the parsed data from the METAR.
//!
//! ```rust
//! use metar::Metar;
//!
//! let metar = "EGHI 282120Z 19015KT 140V220 6000 RA SCT006 BKN009 16/14 Q1006";
//! let r = Metar::parse(metar).unwrap();
//! println!("{:#?}", r);
//! ```
//!
//! ## Issues
//!
//! METARs are complicated structures. If you come across a METAR that doesn't parse
//! correctly, please open an issue and include the METAR. This will aid in debugging
//! the issue significantly.

mod parser;
mod types;

use std::fmt;
pub use types::*;

#[derive(PartialEq, Clone, Debug)]
/// A complete METAR
pub struct Metar {
    /// The station making the METAR measurement
    pub station: String,
    /// The measurement time
    pub time: Time,
    /// The current wind information
    pub wind: Wind,
    /// The current visibility
    pub visibility: Data<Visibility>,
    /// The current clouds
    pub clouds: Data<Clouds>,
    /// The current clouds
    pub cloud_layers: Vec<CloudLayer>,
    /// The current vertical visibility, in feet
    pub vert_visibility: Option<VerticalVisibility>,
    /// The current weather conditions
    pub weather: Vec<Weather>,
    /// The current temperature
    pub temperature: Data<i32>,
    /// The current dewpoint
    pub dewpoint: Data<i32>,
    /// The current air pressure
    pub pressure: Data<Pressure>,
    /// Remarks added on to the METAR
    pub remarks: Option<String>,
}

#[derive(PartialEq, Eq, Clone, Debug)]
/// An error when parsing a METAR
pub struct MetarError {
    /// The string being parsed
    pub string: String,
    /// The start index of the error
    pub start: usize,
    /// The length of the error'd section
    pub length: usize,
    /// The kind of error that occurred
    pub variant: pest::error::ErrorVariant<parser::Rule>,
}

impl std::error::Error for MetarError {}

impl fmt::Display for MetarError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use annotate_snippets::{renderer::DecorStyle, AnnotationKind, Level, Renderer, Snippet};

        let end = self.start + self.length;
        let report = &[Level::ERROR.primary_title(self.variant.message()).element(
            Snippet::source(self.string.clone()).annotation(
                AnnotationKind::Primary
                    .span(self.start..end)
                    .label(self.variant.message()),
            ),
        )];

        let renderer = Renderer::styled().decor_style(DecorStyle::Unicode);

        writeln!(f, "{}", renderer.render(report))
    }
}

impl Metar {
    /// Parse a string into a METAR.
    ///
    /// # Errors
    ///
    /// Returns a [`MetarError`] if parsing failed.
    pub fn parse<S>(data: S) -> Result<Self, MetarError>
    where
        S: Into<String>,
    {
        parser::parse(data.into())
    }
}
