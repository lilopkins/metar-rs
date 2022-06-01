#![deny(missing_docs)]

//! # METAR parsing library for Rust
//!
//! ## Quick usage
//!
//! This simple usage will print out the parsed data from the METAR.
//!
//! ```rust
//! use metar::Metar;
//!
//! fn main() {
//!     let metar = "EGHI 282120Z 19015KT 140V220 6000 RA SCT006 BKN009 16/14 Q1006";
//!     let r = Metar::parse(metar).unwrap();
//!     println!("{:#?}", r);
//! }
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
pub struct Metar<'a> {
    /// The station making the METAR measurement
    pub station: &'a str,
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
    pub vert_visibility: Option<VertVisibility>,
    /// The current weather conditions
    pub weather: Vec<Weather>,
    /// The current temperature
    pub temperature: Data<i32>,
    /// The current dewpoint
    pub dewpoint: Data<i32>,
    /// The current air pressure
    pub pressure: Data<Pressure>,
    /// The current air pressure converted to sea level
    pub sea_level_pressure: Data<Pressure>,
}

#[derive(PartialEq, Eq, Clone, Debug)]
/// An error when parsing a METAR
pub struct MetarError<'a> {
    /// The string being parsed
    pub string: &'a str,
    /// The start index of the error
    pub start: usize,
    /// The length of the error'd section
    pub length: usize,
    /// The kind of error that occurred
    pub variant: pest::error::ErrorVariant<parser::Rule>,
}

impl<'a> std::error::Error for MetarError<'a> {}

impl<'a> fmt::Display for MetarError<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut caret = String::new();
        for _ in 0..self.start {
            caret.push(' ');
        }
        caret.push('^');
        for _ in 1..self.length {
            caret.push('~');
        }
        writeln!(f, "{}\n{}\n{:?}", self.string, caret, self.variant)
    }
}

impl<'a> Metar<'a> {
    /// Parse a string into a METAR
    pub fn parse(data: &'a str) -> Result<Self, MetarError> {
        parser::parse(data)
    }
}
