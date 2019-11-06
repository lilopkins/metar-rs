#![deny(missing_docs)]

//! # METAR parsing library for Rust
//!
//! ## Quick usage
//!
//! This simple usage will print out the parsed data from the METAR.
//!
//! ```rust
//! extern crate metar;
//!
//! fn main() {
//!     let metar = "EGHI 282120Z 19015KT 140V220 6000 RA SCT006 BKN009 16/14 Q1006".to_string();
//!     let r = metar::Metar::parse(metar).unwrap();
//!     println!("{:#?}", r);
//! }
//! ```
//!
//! ## Issues?
//!
//! METARs are complicated structures. If you come across a METAR that doesn't parse
//! correctly, please open an issue and include the METAR. This will aid in debugging
//! the issue significantly.
//!
//! ## Definition of a METAR
//!
//! A METAR can be defined with the following Backus-Naur Form description:
//!
//! ```
//! <metar> ::= <station> ' ' <observationtime> ' ' <method> ' ' <wind> ' ' <wind_varying> <cloudsvis> ' ' <temps> ' ' <pressure> <remark>
//!
//! <station> ::= <letter><letter><letter><letter>
//!
//! <method> ::= '' | 'AUTO'
//!
//! <observationtime> ::= <obs_day><obs_hour><obs_minute> 'Z'
//! <obs_day> ::= <obs_day_1><digit> | '3' <obs_day_2>
//! <obs_day_1> ::= '0' | '1' | '2'
//! <obs_day_2> ::= '0' | '1'
//! <obs_hour> ::= <obs_hour_1><digit> | '2' <obs_hour_2>
//! <obs_hour_1> ::= '0' | '1'
//! <obs_hour_2> ::= '0' | '1' | '2' | '3'
//! <obs_minute> ::= <obs_minute_1><digit>
//! <obs_minute_1> ::= '0' | '1' | '2' | '3' | '4' | '5'
//!
//! <wind> ::= <wind_dir><digit><digit><wind_gusts> 'KT'
//!			 | <wind_dir><digit><digit><wind_gusts> 'MPS'
//! <wind_dir> ::= <angle> | 'VRB'
//! <wind_gusts> ::= '' | 'G' <digit><digit>
//!
//! <wind_varying> ::= '' | <angle> 'V' <angle> ' '
//!
//! <angle> ::= <angle_1><digit><digit> | '3' <angle_2><digit>
//! <angle_1> ::= '0' | '1' | '2'
//! <angle_2> ::= '0' | '1' | '2' | '3' | '4' | '5'
//!
//! <cloudsvis> ::= 'CAVOK' | <visibility> <rvr> <weather> <clouds>
//! <visibility> ::= <digit><digit><digit><digit> | <digit><digit> 'SM'
//! <clouds> ::= 'CLR' | 'SKC' | 'NCD' | 'NSC' | <cloud_description_list> | <vertical_visibility>
//! <rvr> ::= <rvr_entry> | <rvr_entry><rvr>
//! <rvr_entry> ::= 'R' <runway_number> '/' <rvr_vis> <rvr_trend>
//! <runway_number> ::= <angle_1><digit><runway_modifier> | '3' <angle_2><runway_modifier>
//! <runway_modifier> ::= '' | 'L' | 'C' | 'R'
//! <rvr_vis> ::= 'P' <digit><digit><digit><digit> | 'M' <digit><digit><digit><digit>
//! <rvr_trend> ::= 'D' | 'U' | 'N'
//!
//! <cloud_description_list> ::= <cloud_description> | <cloud_description> <cloud_description_list>
//! <cloud_description> ::= <cloud_density> <cloud_floor> <cloud_type>
//! <cloud_density> ::= 'FEW' | 'SCT' | 'BKN' | 'OVC' | '///'
//! <cloud_floor> ::= <digit><digit><digit> | '///'
//! <cloud_type> ::= '' | 'CB' | 'TCU' | '///'
//!
//! <vertical_visibility> ::= 'VV' <vertical_visibility_distance>
//! <vertical_visibility_distance> ::= '///' | <digit><digit>
//!
//! <weather> ::= <weather_cond> | <weather_cond> <weather>
//! <weather_cond> ::= <weather_intesity><weather_descriptor><weather_preceipitation>
//!				 	 | <weather_obscuration>
//!					 | <weather_other>
//!					 | <weather_preceipitation><weather_timing>
//! <weather_intesity> ::= '' | '+' | '-' | 'VC'
//! <weather_descriptor> ::= '' | 'MI' | 'PR' | 'BC' | 'DR' | 'BL' | 'SH' | 'TS' | 'FZ'
//! <weather_preceipitation> ::= 'RA' | 'DZ' | 'SN' | 'SG' | 'IC' | 'PL' | 'GR' | 'GS' | 'UP'
//! <weather_obscuration> ::= 'FG' | 'VA' | 'BR' | 'HZ' | 'DU' | 'FU' | 'SA' | 'PY'
//! <weather_other> ::= 'SQ' | 'PO' | 'DS' | 'SS' | 'FC'
//! <weather_timing> ::= 'B' <weather_timing_time> 'E' <weather_timing_time>
//!				       | 'B' <weather_timing_time>
//!				       | 'E' <weather_timing_time>
//! <weather_timing_time> ::= <digit><digit> | <digit><digit><digit><digit>
//!
//!
//! <temps> ::= <temperature> '/' <temperature>
//! <temperature> ::= 'M' <digit><digit>
//!					| <digit><digit>
//!
//! <pressure> ::= 'Q' <digit><digit><digit><digit>
//!				  | 'A' <digit><digit><digit><digit>
//!
//! <remark> ::= ' RMK' ...
//!
//! <digit> ::= '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9'
//! ```


use std::num::ParseIntError;
mod types;

pub use types::*;

#[derive(PartialEq, Eq, Clone, Debug)]
/// A complete METAR
pub struct Metar<'a> {
    /// The station making the METAR measurement
    pub station: &'a str,
    /// The measurement time
    pub time: Time,
    /// The current wind information
    pub wind: Wind,
    /// The current visibility
    pub visibility: Visibility,
    /// The current cloud layers
    pub cloud_layers: Vec<CloudLayer>,
    /// The current vertical visibility, in feet
    pub vert_visibility: Option<VertVisibility>,
    /// The current temperature
    pub temperature: i32,
    /// The current dewpoint
    pub dewpoint: i32,
    /// The current air pressure
    pub pressure: Pressure,
    /// Any remarks made about the METAR
    pub remarks: Option<&'a str>,
}

#[derive(PartialEq, Eq, Clone, Debug)]
/// An error when parsing a METAR
pub enum MetarError {
    /// An error whilst parsing time
    TimeParseError(ParseIntError),
    /// An error parsing wind direction
    WindDirectionError(ParseIntError),
    /// An error parsing wind speed
    WindSpeedError(ParseIntError),
    /// An error parsing how the winds are gusting
    WindGustError(ParseIntError),
    /// An error parsing the wind directional variation
    WindVaryingError(ParseIntError),
    /// An error parsing the current horizontal visibility
    VisibilityError(ParseIntError),
    /// An error in parsing the cloud layer floor
    CloudFloorError(ParseIntError),
    /// An error parsing the vertical visibility
    VerticalVisibilityError(ParseIntError),
    /// An error parsing the current barometric pressure
    AirPressureError(ParseIntError),
    /// An error parsing the current temperature
    TemperatureError(ParseIntError),
    /// An error parsing the current dewpoint
    DewpointError(ParseIntError),
    /// This METAR doesn't conform to the standard and so cannot be parsed
    InvalidMetarError(String),
}

impl<'a> Metar<'a> {

    /// Parse a string into a METAR
    pub fn parse(_data: &'a str) -> Result<Self, MetarError> {
        let time = Time {
            date: 0,
            hour: 0,
            minute: 0,
        };
        let wind = Wind {
            dir: WindDirection::Heading(0),
            speed: WindSpeed::Knot(0),
            varying: None,
            gusting: None,
        };
        let station = "EGHI";
        let visibility = Visibility::Metres(10000);
        let cloud_layers = Vec::new();
        let vert_visibility = None;
        let temperature = 0;
        let dewpoint = 0;
        let pressure = Pressure::Hectopascals(0);
        let remarks = None;

        Ok(Metar {
            station,
            time,
            wind,
            visibility,
            cloud_layers,
            vert_visibility,
            temperature,
            dewpoint,
            pressure,
            remarks,
        })
    }
}
