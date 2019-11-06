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

extern crate regex;

use regex::Regex;

use std::num::ParseIntError;

#[derive(PartialEq, Eq, Clone, Debug)]
/// A struct to store time as it is represented in a METAR
pub struct Time {
    /// The date the METAR was made
    pub date: u8,
    /// The hour the METAR was made
    pub hour: u8,
    /// The minute the METAR was made
    pub minute: u8,
}

#[derive(PartialEq, Eq, Clone, Debug)]
/// A struct representing the wind speed
pub enum WindSpeed {
    /// A wind speed measured in knots
    Knot(u32),
    /// A wind speed measured in metres per second
    MetresPerSecond(u32),
}

#[derive(PartialEq, Eq, Clone, Debug)]
/// A representation of wind direction
pub enum WindDirection {
    /// A heading defining wind direction
    Heading(u32),
    /// Wind direction is variable
    Variable,
    /// Wind speed is above 49mps or 99kt
    Above,
}

#[derive(PartialEq, Eq, Clone, Debug)]
/// Horizontal visibility
pub enum Visibility {
    /// Visibility is less than this number of metres
    LessThanMetres(u32),
    /// Visibility is less than this number of statute miles
    LessThanStatuteMiles(u32),
    /// Visibility in metres
    Metres(u32),
    /// Visibility in statute miles
    StatuteMiles(u32),
    /// Clouds and Visibility OK (CAVOK)
    CavOK,
    /// No significant clouds (NSC)
    NoSignificantClouds,
    /// Sky clear, no clouds (SKC or CLR)
    SkyClear,
}

#[derive(PartialEq, Eq, Clone, Debug)]
/// Measured air pressure
pub enum Pressure {
    /// Pressure in hectopascals
    Hectopascals(u32),
    /// Pressure in inches of mercury (inHg)
    InchesMercury(u32),
}

#[derive(PartialEq, Eq, Clone, Debug)]
/// Vertical visibility measurement
pub enum VertVisibility {
    /// A distance of vertical visibility
    Distance(u32),
    /// The vertical visibility value is present, so is reduced, but by an amount that hasn't or
    /// cannot be measured
    ReducedByUnknownAmount,
}

#[derive(PartialEq, Eq, Clone, Debug)]
/// Cloud cover
pub enum CloudLayer {
    /// Few clouds (1/8)
    Few(CloudType, u32),
    /// Scattered cloud cover (3/8)
    Scattered(CloudType, u32),
    /// Broken cloud cover (5/8)
    Broken(CloudType, u32),
    /// Overcast cloud cover (7/8)
    Overcast(CloudType, u32),
}

#[derive(PartialEq, Eq, Clone, Debug)]
/// A cloud type description
pub enum CloudType {
    /// A normal cloud
    Normal,
    /// A cumulonimbus cloud
    Cumulonimbus,
    /// A towering cumulus cloud
    ToweringCumulus,
}

#[derive(PartialEq, Eq, Clone, Debug)]
/// Wind information
pub struct Wind {
    /// The wind direction, in degrees
    pub dir: WindDirection,
    /// The current wind speed
    pub speed: WindSpeed,
    /// The direction the wind may be varying between, smaller always comes first
    pub varying: Option<(u32, u32)>,
    /// The gusting speed of the wind
    pub gusting: Option<WindSpeed>,
}

#[derive(PartialEq, Eq, Clone, Debug)]
/// A complete METAR
pub struct Metar {
    /// The station making the METAR measurement
    pub station: String,
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
    pub remarks: Option<String>,
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

impl Metar {

    /// Parse a string into a METAR
    pub fn parse(data: String) -> Result<Self, MetarError> {
        let mut time = Time {
            date: 0,
            hour: 0,
            minute: 0,
        };
        let mut wind = Wind {
            dir: WindDirection::Heading(0),
            speed: WindSpeed::Knot(0),
            varying: None,
            gusting: None,
        };
        let mut visibility = Visibility::Metres(10000);
        let mut cloud_layers = Vec::new();
        let mut vert_visibility = None;
        let mut temperature = 0;
        let mut dewpoint = 0;
        let mut pressure = Pressure::Hectopascals(0);
        let mut remarks = None;

        let re = Regex::new(&r"(?P<station>[A-Z0-9]{4}) (?P<time>[0-9]{6}Z) (?P<data>NIL|(?:AUTO )?(?P<wind_dir>[0-9]{3}|VRB|ABV)(?P<wind_speed>[0-9]{2})(?:G(?P<wind_gusts>[0-9]{2}))?(?P<wind_unit>KT|MPS) (?:(?P<wind_varying_from>[0-9]{3})V(?P<wind_varying_to>[0-9]{3}) )?(?P<visibility>CAVOK|NSC|SKC|M?[0-9]{2}SM|M?[0-9]{4}) (?P<rvr>(?:R[0-9]{2}[LCR]?\/[PM]?[0-9]{4}(?:V[0-9]{4})?[DUN]? )*)(?P<wx>(?:(?:VC|\-|\+)?(?:TS|SH|FZ|BL|DR|MI|BC|PR|DZ|RA|SN|SG|PL|IC|GR|GS|UP|FG|BR|SA|DU|HZ|FU|VA|PO|SQ|FC|DS|SS) ?)*)(?P<cloud>CLR |NCD |NSC |(?:(?:FEW|SCT|BKN|OVC)[0-9]{3}(?:CB|TCU)? )*)(?:VV(?:\/\/\/|(?P<vert_visibility>[0-9]{3})) )?(?P<temperature>M?[0-9]{2})\/(?P<dewpoint>M?[0-9]{2}) (?P<pressure>(?:Q|A)[0-9]{4}))(?: RMK (?P<remarks>.*))?".replace("\\/", "/")).unwrap();

        let parts = re.captures(&data);
        if parts.is_none() {
            return Err(MetarError::InvalidMetarError(data));
        }
        let parts = parts.unwrap();

        // Parse station
        let station = parts["station"].to_string();

        // Parse time
        let time_s = parts["time"].to_string();
        time.date = match time_s[0..2].parse::<u8>() {
            Ok(v) => v,
            Err(e) => return Err(MetarError::TimeParseError(e)),
        };
        time.hour = match time_s[2..4].parse::<u8>() {
            Ok(v) => v,
            Err(e) => return Err(MetarError::TimeParseError(e)),
        };
        time.minute = match time_s[4..6].parse::<u8>() {
            Ok(v) => v,
            Err(e) => return Err(MetarError::TimeParseError(e)),
        };

        // Deal with NIL/AUTO
        if &parts["data"] == "NIL" {
            // NIL METAR
            return Ok(Metar {
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
            });
        }

        // Wind
        // Wind heading
        let hdg = &parts["wind_dir"];
        if hdg == "VRB" {
            wind.dir = WindDirection::Variable;
        } else if hdg == "ABV" {
            wind.dir = WindDirection::Above;
        } else {
            wind.dir = WindDirection::Heading(match hdg.parse::<u32>() {
                Ok(v) => v,
                Err(e) => return Err(MetarError::WindDirectionError(e)),
            });
        }
        // Wind speed and gusting
        let speed = match parts["wind_speed"].parse::<u32>() {
            Ok(v) => v,
            Err(e) => return Err(MetarError::WindSpeedError(e)),
        };;
        let mut gusting: Option<u32> = None;
        if let Some(part) = parts.name("wind_gusts") {
            gusting = Some(match part.as_str().parse::<u32>() {
                Ok(v) => v,
                Err(e) => return Err(MetarError::WindGustError(e)),
            });
        }
        if parts["wind_unit"].ends_with("KT") {
            // knots
            wind.speed = WindSpeed::Knot(speed);
            if let Some(g) = gusting {
                wind.gusting = Some(WindSpeed::Knot(g));
            }
        } else {
            // mps
            wind.speed = WindSpeed::MetresPerSecond(speed);
            if let Some(g) = gusting {
                wind.gusting = Some(WindSpeed::MetresPerSecond(g));
            }
        }

        if let Some(part) = parts.name("wind_varying_from") {
            // Wind varying
            let from = match part.as_str().parse::<u32>() {
                Ok(v) => v,
                Err(e) => return Err(MetarError::WindVaryingError(e)),
            };
            let to = match parts["wind_varying_to"].parse::<u32>() {
                Ok(v) => v,
                Err(e) => return Err(MetarError::WindVaryingError(e)),
            };
            wind.varying = Some((from, to));
        }

        let visibility_p = &parts["visibility"];
        if visibility_p == "CAVOK" {
            visibility = Visibility::CavOK;
        } else if visibility_p == "NSC" {
            visibility = Visibility::NoSignificantClouds;
        } else if visibility_p == "SKC"
            || visibility_p == "CLR" {

            visibility = Visibility::SkyClear;
        } else if visibility_p.starts_with("M") {
            if visibility_p.ends_with("SM") {
                visibility = Visibility::LessThanStatuteMiles(match visibility_p[1..3].parse::<u32>() {
                    Ok(v) => v,
                    Err(e) => return Err(MetarError::VisibilityError(e)),
                });
            } else {
                visibility = Visibility::LessThanMetres(match visibility_p[1..5].parse::<u32>() {
                    Ok(v) => v,
                    Err(e) => return Err(MetarError::VisibilityError(e)),
                });
            }
        } else {
            if visibility_p.ends_with("SM") {
                visibility = Visibility::StatuteMiles(match visibility_p[0..2].parse::<u32>() {
                    Ok(v) => v,
                    Err(e) => return Err(MetarError::VisibilityError(e)),
                });
            } else {
                visibility = Visibility::Metres(match visibility_p[0..4].parse::<u32>() {
                    Ok(v) => v,
                    Err(e) => return Err(MetarError::VisibilityError(e)),
                });
            }
        }

        // TODO: RVRs

        // TODO: Weather

        // Clouds
        if let Some(clouds_s) = parts.name("clouds") {
            let clouds_p: Vec<_> = clouds_s.as_str().split(" ").collect();
            for cloud in clouds_p {
                let part = cloud.trim();
                if part == "NCD"
                    || part == "NSC" {
                    break;
                }
                // Cloud type
                let mut typ = CloudType::Normal;
                if part.ends_with("TCU") {
                    typ = CloudType::ToweringCumulus;
                } else if part.ends_with("CB") {
                    typ = CloudType::Cumulonimbus;
                }
                // Cloud floor
                let floor = match part[3..6].parse::<u32>() {
                    Ok(v) => v,
                    Err(e) => return Err(MetarError::CloudFloorError(e)),
                };
                // Cloud cover
                if part.starts_with("FEW") {
                    cloud_layers.push(CloudLayer::Few(typ, floor));
                } else if part.starts_with("SCT") {
                    cloud_layers.push(CloudLayer::Scattered(typ, floor));
                } else if part.starts_with("BKN") {
                    cloud_layers.push(CloudLayer::Broken(typ, floor));
                } else if part.starts_with("OVC") {
                    cloud_layers.push(CloudLayer::Overcast(typ, floor));
                }
            }
        }

        if let Some(part) = parts.name("vert_visibility") {
            // Vertical visibility
            if part.as_str() == "///" {
                vert_visibility = Some(VertVisibility::ReducedByUnknownAmount);
            } else {
                vert_visibility = match part.as_str().parse::<u32>() {
                    Ok(v) => Some(VertVisibility::Distance(v)),
                    Err(e) => return Err(MetarError::VerticalVisibilityError(e)),
                };
            }
        }

        let temp = &parts["temperature"];
        let dewp = &parts["dewpoint"];
        if temp.starts_with("M") {
            temperature = -1 * match temp[1..].parse::<i32>() {
                Ok(v) => v,
                Err(e) => return Err(MetarError::TemperatureError(e)),
            };
        } else {
            temperature = match temp.parse::<i32>() {
                Ok(v) => v,
                Err(e) => return Err(MetarError::TemperatureError(e)),
            };
        }
        if dewp.starts_with("M") {
            dewpoint = -1 * match dewp[1..].parse::<i32>() {
                Ok(v) => v,
                Err(e) => return Err(MetarError::DewpointError(e)),
            };
        } else {
            dewpoint = match dewp.parse::<i32>() {
                Ok(v) => v,
                Err(e) => return Err(MetarError::DewpointError(e)),
            };
        }

        if parts["pressure"].starts_with("Q") {
            // hPa pressure
            pressure = Pressure::Hectopascals(match parts["pressure"][1..].parse::<u32>() {
                Ok(v) => v,
                Err(e) => return Err(MetarError::AirPressureError(e)),
            });
        } else if parts["pressure"].starts_with("A") {
            // inMg pressure
            pressure = Pressure::InchesMercury(match parts["pressure"][1..].parse::<u32>() {
                Ok(v) => v,
                Err(e) => return Err(MetarError::AirPressureError(e)),
            });
        }

        if let Some(part) = parts.name("remarks") {
            remarks = Some(part.as_str().to_string());
        } else {
            remarks = None;
        }

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
