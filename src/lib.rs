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
//!     let metar = "EGHI 282120Z 19015KT 140V220 6000 RA SCT006 BKN009 16/14 Q1006";
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
//! ```bnf
//! <metar> ::= <station> ' ' <observationtime> ' ' <method> ' ' <wind> ' ' <wind_varying> <cloudsvis> ' ' <temps> ' ' <pressure> <remark>
//!
//! <station> ::= <letter><letter><letter><letter>
//!
//! <method> ::= '' | 'AUTO' | 'COR'
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
//! <angle> ::= <angle_1><digit><digit> | '3' <angle_2><digit> | '360'
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
//! <vertical_visibility_distance> ::= '///' | <digit><digit><digit>
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

mod parsers;
mod types;
pub use parsers::errors::*;
use std::fmt;
use types::Data::*;
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
    /// Any remarks made about the METAR
    pub remarks: Option<&'a str>,
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
    /// The current parser state (what it was expecting to read)
    pub parser_state: ParseState,
    /// The error from the internal parser
    pub error: ParserError,
}

impl<'a> MetarError<'a> {
    fn new(
        string: &'a str,
        start: usize,
        length: usize,
        parser_state: ParseState,
        error: ParserError,
    ) -> Self {
        Self {
            string,
            start,
            length,
            parser_state,
            error,
        }
    }
}

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
        write!(f, "{}\n{}\n{}", self.string, caret, self.error)
    }
}

impl fmt::Display for ParserError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Station(e) => write!(f, "{}", e),
            Self::ObservationTime(e) => write!(f, "{}", e),
            Self::Wind(e) => write!(f, "{}", e),
            Self::WindVarying(e) => write!(f, "{}", e),
            Self::CloudVisibility(e) => write!(f, "{}", e),
            Self::Temperature(e) => write!(f, "{}", e),
            Self::Pressure(e) => write!(f, "{}", e),
        }
    }
}

#[derive(PartialEq, Eq, Clone, Debug)]
/// An error from an internal parser
pub enum ParserError {
    /// An error from the station parser
    Station(StationError),
    /// An error from the observation time parser
    ObservationTime(ObservationTimeError),
    /// An error from the wind parser
    Wind(WindError),
    /// An error from the wind varying parser
    WindVarying(WindVaryingError),
    /// An error from the cloud/visibility parser
    CloudVisibility(CloudVisibilityError),
    /// An error from the temperature parser
    Temperature(TemperatureError),
    /// An error from the pressure parser
    Pressure(PressureError),
}

/// Find the words in a string by splitting into an array of usize tuples with the start index and
/// length of each word
fn find_words<'a>(s: &'a str) -> Vec<(&'a str, usize, usize)> {
    let mut words = Vec::new();
    let chs: Vec<_> = s.chars().collect();
    let mut start_idx = 0;
    let mut last_read_ws = false;
    let len = chs.len();
    for i in 0..len {
        if chs[i].is_whitespace() && !last_read_ws {
            last_read_ws = true;
            words.push((&s[start_idx..i], start_idx, i - start_idx));
        }
        if !chs[i].is_whitespace() {
            if last_read_ws {
                start_idx = i;
            }
            last_read_ws = false;
        }
    }

    if !last_read_ws {
        words.push((&s[start_idx..], start_idx, len - start_idx));
    }

    words
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
/// The state of the parser, used in error messages to describe the expected next occurence when it
/// wasn't reached.
pub enum ParseState {
    /// Expected an ICAO station
    Station,
    /// Expected an observation time
    ObservationTime,
    /// Expected either a recording method ('AUTO') or
    /// wind information, cloud information, weather information, or pressure
    AfterObsTime,
    /// Expected either remarks or the METAR end
    RemarksOrEnd,
}

impl<'a> Metar<'a> {
    /// Parse a string into a METAR
    pub fn parse(data: &'a str) -> Result<Self, MetarError> {
        let mut metar = Metar {
            station: "XXXX",
            time: Time {
                date: 0,
                hour: 0,
                minute: 0,
            },
            wind: Wind {
                dir: Unknown,
                speed: Unknown,
                varying: None,
                gusting: None,
            },
            visibility: Unknown,
            clouds: Unknown,
            cloud_layers: Vec::new(),
            vert_visibility: None,
            weather: Vec::new(),
            temperature: Unknown,
            dewpoint: Unknown,
            pressure: Unknown,
            remarks: None,
        };

        let mut state = ParseState::Station;
        let words = find_words(data);
        for word_idx in words {
            let word = word_idx.0;

            match state {
                ParseState::Station => {
                    let r = parsers::parse_station(word);
                    if let Ok(data) = r {
                        metar.station = data;
                        state = ParseState::ObservationTime;
                    } else if let Err(e) = r {
                        return Err(MetarError::new(
                            data,
                            word_idx.1 + e.0,
                            e.1,
                            state,
                            ParserError::Station(e.2),
                        ));
                    }
                }
                ParseState::ObservationTime => {
                    let r = parsers::parse_obs_time(word);
                    if let Ok(data) = r {
                        metar.time = data;
                        state = ParseState::AfterObsTime;
                    } else if let Err(e) = r {
                        return Err(MetarError::new(
                            data,
                            word_idx.1 + e.0,
                            e.1,
                            state,
                            ParserError::ObservationTime(e.2),
                        ));
                    }
                }
                ParseState::AfterObsTime => {
                    if word == "AUTO" || word == "COR" {
                        // Method - just ignore for now
                        continue;
                    }

                    if word == "RMK" {
                        // We are now in the remark section, switch state
                        state = ParseState::RemarksOrEnd;
                        continue;
                    }

                    if metar.wind.dir == Unknown {
                        let r = parsers::parse_wind(word);
                        if let Ok(data) = r {
                            metar.wind = data;
                            continue;
                        } // else if let Err(e) = r {
                          //	return Err(MetarError::new(data, word_idx.1 + e.0, e.1,
                          //    	state, ParserError::Wind(e.2)));
                          // }
                    }
                    // Treat as wind varying
                    let r = parsers::parse_wind_varying(word);
                    if let Ok(data) = r {
                        metar.wind.varying = Some(data);
                        continue;
                    }

                    // Parse cloud/vis data
                    let r2 = parsers::parse_cloud_visibility_info(word);
                    if let Ok(data) = r2 {
                        match data {
                            parsers::CloudVisibilityInfo::CloudLayer(layer) => {
                                metar.clouds = Known(Clouds::CloudLayers);
                                metar.cloud_layers.push(layer);
                            }
                            parsers::CloudVisibilityInfo::Clouds(clouds) => {
                                metar.clouds = Known(clouds);
                            }
                            parsers::CloudVisibilityInfo::RVR(..) => {}
                            parsers::CloudVisibilityInfo::VerticalVisibility(vv) => {
                                metar.vert_visibility = Some(vv);
                                metar.clouds = Unknown;
                            }
                            parsers::CloudVisibilityInfo::Visibility(visibility) => {
                                if visibility == Unknown {
                                    metar.visibility = Unknown;
                                    continue;
                                }
                                if visibility.unwrap().is_infinite() {
                                    metar.clouds = Known(Clouds::SkyClear);
                                }
                                if visibility.unwrap().unit == DistanceUnit::StatuteMiles {
                                    if metar.visibility == Unknown {
                                        metar.visibility = visibility;
                                    } else {
                                        if metar.visibility.unwrap().unit
                                            == DistanceUnit::StatuteMiles
                                        {
                                            metar.visibility.unwrap_mut().visibility +=
                                                visibility.unwrap().visibility;
                                        } else {
                                            metar.visibility = visibility;
                                        }
                                    }
                                } else {
                                    metar.visibility = visibility;
                                }
                            }
                            parsers::CloudVisibilityInfo::Weather(wx) => {
                                metar.weather.push(wx);
                            }
                        };
                        continue;
                    }

                    // Treat as temperatures
                    let r3 = parsers::parse_temperatures(word);
                    if let Ok(data) = r3 {
                        metar.temperature = data.0;
                        metar.dewpoint = data.1;
                        continue;
                    }

                    let r4 = parsers::parse_pressure(word);
                    if let Ok(data) = r4 {
                        metar.pressure = data;
                    }
                }
                ParseState::RemarksOrEnd => {
                    metar.remarks = Some(&data[word_idx.1..]);
                    break;
                }
            }
        }

        Ok(metar)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_words() {
        let r = find_words("The quick brown fox.");
        assert_eq!(
            r,
            [
                ("The", 0, 3),
                ("quick", 4, 5),
                ("brown", 10, 5),
                ("fox.", 16, 4)
            ]
        );
    }
}
