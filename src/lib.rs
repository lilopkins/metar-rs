#![deny(missing_docs)]

//!

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
    /// No significant clouds
    NoSignificantClouds,
    /// Sky clear, no clouds
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
    pub vert_visibility: Option<u32>,
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
    TimeParseError(std::num::ParseIntError),
    /// An error parsing wind direction
    WindDirectionError(std::num::ParseIntError),
    /// An error parsing wind speed
    WindSpeedError(std::num::ParseIntError),
    /// An error parsing how the winds are gusting
    WindGustError(std::num::ParseIntError),
    /// An error parsing the wind directional variation
    WindVaryingError(std::num::ParseIntError),
    /// An error parsing the current horizontal visibility
    VisibilityError(std::num::ParseIntError),
    /// An error in parsing the cloud layer floor
    CloudFloorError(std::num::ParseIntError),
    /// An error parsing the vertical visibility
    VerticalVisibilityError(std::num::ParseIntError),
    /// An error parsing the current barometric pressure
    AirPressureError(std::num::ParseIntError),
    /// An error parsing the current temperature
    TemperatureError(std::num::ParseIntError),
    /// An error parsing the current dewpoint
    DewpointError(std::num::ParseIntError),
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

        let parts: Vec<&str> = data.split(" ").collect();

        let mut index_offset = 0;

        // Parse station
        let station = parts[index_offset].to_string();
        index_offset += 1;

        // Parse time
        let time_s = parts[index_offset].to_string();
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
        index_offset += 1;

        // Deal with NIL/AUTO
        if parts[index_offset] == "NIL" {
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
        } else if parts[index_offset] == "AUTO" {
            index_offset += 1;
        }

        // Wind
        let wind_p = parts[index_offset];
        // Wind heading
        let hdg = &wind_p[0..3];
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
        let speed = match wind_p[3..5].parse::<u32>() {
            Ok(v) => v,
            Err(e) => return Err(MetarError::WindSpeedError(e)),
        };;
        let mut gusting: Option<u32> = None;
        if wind_p.contains("G") {
            gusting = Some(match wind_p[6..8].parse::<u32>() {
                Ok(v) => v,
                Err(e) => return Err(MetarError::WindGustError(e)),
            });
        }
        if wind_p.ends_with("KT") {
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
        index_offset += 1;

        if parts[index_offset].contains("V") {
            // Wind varying
            let part = parts[index_offset];
            let from = match part[0..3].parse::<u32>() {
                Ok(v) => v,
                Err(e) => return Err(MetarError::WindVaryingError(e)),
            };
            let to = match part[4..].parse::<u32>() {
                Ok(v) => v,
                Err(e) => return Err(MetarError::WindVaryingError(e)),
            };
            wind.varying = Some((from, to));
            index_offset += 1;
        }

        let visibility_p = parts[index_offset];
        if visibility_p == "CAVOK" {
            visibility = Visibility::CavOK;
        } else if visibility_p == "NSC" {
            visibility = Visibility::NoSignificantClouds;
        } else if visibility_p == "SKC" {
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
        index_offset += 1;

        while parts[index_offset].starts_with("R")
            && parts[index_offset].contains("/") {
            // RVR
            // TODO: RVR
            index_offset += 1;
        }

        // Weather
        // TODO: Currently this is skipped
        while !(parts[index_offset].starts_with("NCD")
                || parts[index_offset].starts_with("FEW")
                || parts[index_offset].starts_with("SCT")
                || parts[index_offset].starts_with("BKN")
                || parts[index_offset].starts_with("OVC")) {

            index_offset += 1;
        }

        // Clouds
        while parts[index_offset].starts_with("NCD")
                || parts[index_offset].starts_with("FEW")
                || parts[index_offset].starts_with("SCT")
                || parts[index_offset].starts_with("BKN")
                || parts[index_offset].starts_with("OVC") {

            let part = parts[index_offset];
            if part == "NCD" {
                index_offset += 1;
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
            index_offset += 1;
        }

        if parts[index_offset].starts_with("VV") {
            // Vertical visibilty
            vert_visibility = match parts[index_offset][2..].parse::<u32>() {
                Ok(v) => Some(v),
                Err(e) => return Err(MetarError::VerticalVisibilityError(e)),
            };
            index_offset += 1;
        }

        if parts[index_offset].contains("/") {
            // temp/dewpoint
            let internal_parts: Vec<&str> = parts[index_offset].split("/").collect();
            let temp = internal_parts[0];
            let dewp = internal_parts[1];
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
            index_offset += 1;
        }

        if parts[index_offset].starts_with("Q") {
            // hPa pressure
            pressure = Pressure::Hectopascals(match parts[index_offset][1..].parse::<u32>() {
                Ok(v) => v,
                Err(e) => return Err(MetarError::AirPressureError(e)),
            });
            index_offset += 1;
        } else if parts[index_offset].starts_with("A") {
            // inMg pressure
            pressure = Pressure::InchesMercury(match parts[index_offset][1..].parse::<u32>() {
                Ok(v) => v,
                Err(e) => return Err(MetarError::AirPressureError(e)),
            });
            index_offset += 1;
        }

        remarks = Some(parts[index_offset..].join(" "));

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

#[cfg(test)]
mod tests {
    #[test]
    fn test_metar_1() {
        let metar = "EGHI 282120Z 19015KT 140V220 6000 RA SCT006 BKN009 16/14 Q1006".to_string();
        let r = super::Metar::parse(metar).unwrap_or_else(|e| {
            eprintln!("{:#?}", e);
            assert!(false);
            std::process::exit(1);
        });

        assert_eq!(r.station, "EGHI");

        assert_eq!(r.time.date, 28);
        assert_eq!(r.time.hour, 21);
        assert_eq!(r.time.minute, 20);

        assert_eq!(r.wind.dir, super::WindDirection::Heading(190));
        assert_eq!(r.wind.speed, super::WindSpeed::Knot(15));
        assert_eq!(r.wind.varying, Some((140, 220)));

        assert_eq!(r.visibility, super::Visibility::Metres(6000));

        assert_eq!(r.temperature, 16);
        assert_eq!(r.dewpoint, 14);

        assert_eq!(r.pressure, super::Pressure::Hectopascals(1006));
    }
}
