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
            sea_level_pressure: Unknown,
        };

        let mut state = ParseState::Station;
        for word in data.split_whitespace() {
            if !word.is_ascii() {
                continue;
            }

            match state {
                ParseState::Station => {
                    let r = parsers::parse_station(word);
                    if let Ok(data) = r {
                        metar.station = data;
                        state = ParseState::ObservationTime;
                    } else if let Err(e) = r {
                        return Err(MetarError::new(
                            data,
                            e.0,
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
                            e.0,
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
                    let r = parsers::parse_sea_level_pressure(word);
                    if let Ok(data) = r {
                        metar.sea_level_pressure = data;
                    }
                }
            }
        }

        Ok(metar)
    }
}
