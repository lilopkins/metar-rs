use super::types::*;
use super::types::Data::{Known, Unknown};

/// A result with an error case of a 3-tuple containing the start offset, the length and the error
/// information.
type ParserResult<T, E> = Result<T, (usize, usize, E)>;

pub mod errors;
use errors::*;

pub enum CloudVisibilityInfo {
    VerticalVisibility(VertVisibility),
    Visibility(Data<Visibility>),
    // TODO: Fully add RVRs
    RVR(),
    Weather(Weather),
    Clouds(Clouds),
    CloudLayer(CloudLayer),
}

pub fn parse_station<'a>(s: &'a str) -> ParserResult<&'a str, StationError> {
    if s.len() != 4 {
        // Not correct length
        return Err((0, s.len(), StationError::IncorrectLength));
    }

    let chs: Vec<_> = s.chars().collect();
    for i in 0..chs.len() {
        let c = chs[i];
        if !c.is_alphanumeric() {
            return Err((i, 1, StationError::NonAlphanumericCharacter));
        }
    }
    Ok(s)
}

pub fn parse_obs_time<'a>(s: &'a str) -> ParserResult<Time, ObservationTimeError> {
    let mut time = Time {
        date: 0,
        hour: 0,
        minute: 0,
    };

    if s.len() != 7 {
        // Not correct length
        return Err((0, s.len(), ObservationTimeError::IncorrectLength));
    }

    let chs: Vec<_> = s.chars().collect();

    if !chs[0].is_digit(10) {
        // Not valid digit
        return Err((0, 1, ObservationTimeError::DateNotValid));
    } else if !chs[1].is_digit(10) {
        // Not valid digit
        return Err((1, 1, ObservationTimeError::DateNotValid));
    } else {
        // Date in range
        let date = s[0..2].parse().unwrap();
        if date > 31 {
            return Err((0, 2, ObservationTimeError::DateNotValid));
        }
        time.date = date;
    }

    if !chs[2].is_digit(10) {
        // Not valid digit
        return Err((2, 1, ObservationTimeError::HourNotValid));
    } else if !chs[3].is_digit(10) {
        // Not valid digit
        return Err((3, 1, ObservationTimeError::HourNotValid));
    } else {
        // Hour in range
        let hour = s[2..4].parse().unwrap();
        if hour > 23 {
            return Err((2, 2, ObservationTimeError::HourNotValid));
        }
        time.hour = hour;
    }

    if !chs[4].is_digit(10) {
        // Minute valid digit
        return Err((4, 1, ObservationTimeError::MinuteNotValid));
    } else if !chs[5].is_digit(10) {
        // Minute valid digit
        return Err((5, 1, ObservationTimeError::MinuteNotValid));
    } else {
        // Minute in range
        let minute = s[4..6].parse().unwrap();
        if minute > 59 {
            return Err((4, 2, ObservationTimeError::MinuteNotValid));
        }
        time.minute = minute;
    }

    if chs[6] != 'Z' {
        return Err((6, 1, ObservationTimeError::InvalidTimeZone));
    }

    Ok(time)
}

pub fn parse_wind<'a>(s: &'a str) -> ParserResult<Wind, WindError> {
    let mut wind = Wind {
        dir: Unknown,
        speed: Unknown,
        varying: None,
        gusting: None,
    };

    let chs: Vec<_> = s.chars().collect();

    if chs.len() < 7 {
        return Err((0, s.len(), WindError::IncorrectLength));
    }

    if &s[0..3] == "///" {
        wind.dir = Unknown;
    } else if &s[0..3] == "VRB" {
        wind.dir = Known(WindDirection::Variable);
    } else if &s[0..3] == "ABV" {
        wind.dir = Known(WindDirection::Above);
    } else if chs[0].is_digit(10)
        && chs[1].is_digit(10)
        && chs[2].is_digit(10) {
        
        let heading = s[0..3].parse().unwrap();
        if heading > 360 {
            return Err((0, 3, WindError::HeadingNotValid));
        }
        wind.dir = Known(WindDirection::Heading(heading));
    }

    if chs[3].is_digit(10)
        && chs[4].is_digit(10) {

        let speed = s[3..5].parse().unwrap();

        if chs[5] == 'G' {
            if !chs[6].is_digit(10) {
                return Err((6, 1, WindError::GustingNotValid));
            } else if !chs[7].is_digit(10) {
                return Err((7, 1, WindError::GustingNotValid));
            }
            let g_speed = s[6..8].parse().unwrap();

            let unit = &s[8..];
            if unit == "KT" {
                wind.speed = Known(WindSpeed {
                    speed,
                    unit: SpeedUnit::Knot,
                });
                wind.gusting = Some(WindSpeed {
                    speed: g_speed,
                    unit: SpeedUnit::Knot,
                });
            } else if unit == "MPS" {
                wind.speed = Known(WindSpeed {
                    speed,
                    unit: SpeedUnit::MetresPerSecond,
                });
                wind.gusting = Some(WindSpeed {
                    speed: g_speed,
                    unit: SpeedUnit::MetresPerSecond,
                });
            } else {
                return Err((8, unit.len(), WindError::UnitNotValid));
            }
        } else {
            let unit = &s[5..];
            if unit == "KT" {
                wind.speed = Known(WindSpeed {
                    speed,
                    unit: SpeedUnit::Knot,
                });
            } else if unit == "MPS" {
                wind.speed = Known(WindSpeed {
                    speed,
                    unit: SpeedUnit::MetresPerSecond,
                });
            } else {
                return Err((5, unit.len(), WindError::UnitNotValid));
            }
        }
    }

    Ok(wind)
}

pub fn parse_wind_varying<'a>(s: &'a str) -> ParserResult<(u32, u32), WindVaryingError> {
    let chs: Vec<_> = s.chars().collect();

    if s.len() != 7 {
        return Err((0, s.len(), WindVaryingError::NotWindVarying));
    }

    if chs[3] != 'V' {
        return Err((3, 1, WindVaryingError::NotWindVarying));
    }

    if !chs[0].is_digit(10) {
        return Err((0, 1, WindVaryingError::HeadingNotValid));
    } else if !chs[1].is_digit(10) {
        return Err((1, 1, WindVaryingError::HeadingNotValid));
    } else if !chs[2].is_digit(10) {
        return Err((2, 1, WindVaryingError::HeadingNotValid));
    } else if !chs[4].is_digit(10) {
        return Err((4, 1, WindVaryingError::HeadingNotValid));
    } else if !chs[5].is_digit(10) {
        return Err((5, 1, WindVaryingError::HeadingNotValid));
    } else if !chs[6].is_digit(10) {
        return Err((6, 1, WindVaryingError::HeadingNotValid));
    } else {
        let heading_from = s[0..3].parse().unwrap();
        let heading_to = s[4..7].parse().unwrap();
        if heading_from > 360 {
            return Err((0, 3, WindVaryingError::HeadingNotValid));
        }
        if heading_to > 360 {
            return Err((4, 3, WindVaryingError::HeadingNotValid));
        }
        return Ok((heading_from, heading_to));
    }
}

pub fn parse_cloud_visibility_info<'a>(s: &'a str) -> ParserResult<CloudVisibilityInfo, CloudVisibilityError> {
    if s == "CAVOK" {
        return Ok(CloudVisibilityInfo::Visibility(Known(Visibility::infinite())));
    }

    // Simple Cloud States
    if s == "CLR"
        || s == "SKC"{

        return Ok(CloudVisibilityInfo::Clouds(Clouds::SkyClear));
    }
    if s == "NCD" {
        return Ok(CloudVisibilityInfo::Clouds(Clouds::NoCloudDetected));
    }
    if s == "NSC" {
        return Ok(CloudVisibilityInfo::Clouds(Clouds::NoSignificantCloud));
    }

    let chs: Vec<_> = s.chars().collect();

    // Cloud layers
    if s.len() >= 6 {
        if &s[0..3] == "FEW"
            || &s[0..3] == "SCT"
            || &s[0..3] == "BKN"
            || &s[0..3] == "OVC"
            || &s[0..3] == "///" {

            let mut cloud_type = CloudType::Normal;
            if s.len() > 6 {
                let t = &s[6..];
                if t == "TCU" {
                    cloud_type = CloudType::ToweringCumulus;
                } else if t == "CB" {
                    cloud_type = CloudType::Cumulonimbus;
                } else if t == "///" {
                    cloud_type = CloudType::Unknown;
                }
            }

            let mut cloud_floor = None;
            if let Ok(floor) = s[3..6].parse() {
                cloud_floor = Some(floor);
            }

            let cl;
            if &s[0..3] == "FEW" {
                cl = CloudLayer::Few(cloud_type, cloud_floor);
            } else if &s[0..3] == "SCT" {
                cl = CloudLayer::Scattered(cloud_type, cloud_floor);
            } else if &s[0..3] == "BKN" {
                cl = CloudLayer::Broken(cloud_type, cloud_floor);
            } else if &s[0..3] == "OVC" {
                cl = CloudLayer::Overcast(cloud_type, cloud_floor);
            } else {
                cl = CloudLayer::Unknown(cloud_type, cloud_floor);
            }

            return Ok(CloudVisibilityInfo::CloudLayer(cl));
        }
    }

    // RVR
    if s.len() >= 2 {
        if chs[0] == 'R'
            && chs[1].is_digit(10) {
            return Ok(CloudVisibilityInfo::RVR());
        }
    }

    // Vertical visibility
    if s.len() >= 5 {
        if chs[0] == 'V' && chs[1] == 'V' {
            if chs[2].is_digit(10) && chs[3].is_digit(10) && chs[4].is_digit(10) {
                return Ok(CloudVisibilityInfo::VerticalVisibility(VertVisibility::Distance(s[2..5].parse().unwrap())));
            } else {
                return Ok(CloudVisibilityInfo::VerticalVisibility(VertVisibility::ReducedByUnknownAmount));
            }
        }
    }

    // Visibility
    if s.len() >= 4 {
        if chs[0].is_digit(10)
            && chs[1].is_digit(10)
            && chs[2].is_digit(10)
            && chs[3].is_digit(10) {

            return Ok(CloudVisibilityInfo::Visibility(Known(Visibility {
                visibility: s[0..4].parse().unwrap(),
                unit: DistanceUnit::Metres,
            })));
        } else if s == "////" {
            return Ok(CloudVisibilityInfo::Visibility(Unknown));
        }
    }
    if s.ends_with("SM") {
        let s = &s[0..s.len() - 2];
        if s.contains("/") {
            // Fractional visibilty
            let parts: Vec<_> = s.split("/").collect();
            let numerator: u32 = parts[0].parse().unwrap();
            let denominator: u32 = parts[1].parse().unwrap();
            let fraction: f32 = numerator as f32 / denominator as f32;
            return Ok(CloudVisibilityInfo::Visibility(Known(Visibility {
                visibility: fraction,
                unit: DistanceUnit::StatuteMiles,
            })));
        } else {
            return Ok(CloudVisibilityInfo::Visibility(Known(Visibility {
                visibility: s.parse().unwrap(),
                unit: DistanceUnit::StatuteMiles,
            })));
        }
    }

    // If just a number, this might be a visibility in statute miles
    let v = s.parse();
    if v.is_ok() {
        let v: u32 = v.unwrap();
        return Ok(CloudVisibilityInfo::Visibility(Known(Visibility {
            visibility: v as f32,
            unit: DistanceUnit::StatuteMiles,
        })));
    }

    if s.len() < 2 {
        // Not long enough for any weather.
        return Err((0, s.len(), CloudVisibilityError::UnknownData));
    }

    // Weather
    let intensity;
    let mut i = 0;
    if chs[0] == '+' {
        intensity = WeatherIntensity::Heavy;
        i += 1;
    } else if chs[0] == '-' {
        intensity = WeatherIntensity::Light;
        i += 1;
    } else if chs[0] == 'V'
        && chs[1] == 'C' {
        // Vicinity
        intensity = WeatherIntensity::InVicinity;
        i += 2;
    } else {
        intensity = WeatherIntensity::Moderate;
    }
    let mut conditions = Vec::new();
    loop {
        if s.len() < i + 2 {
            break;
        }
        let mut s = String::new();
        s.push(chs[i]);
        s.push(chs[i + 1]);

        if s == "MI" {
            conditions.push(WeatherCondition::Shallow);
        } else if s == "PR" {
            conditions.push(WeatherCondition::Partial);
        } else if s == "BC" {
            conditions.push(WeatherCondition::Patches);
        } else if s == "DR" {
            conditions.push(WeatherCondition::LowDrifting);
        } else if s == "BL" {
            conditions.push(WeatherCondition::Blowing);
        } else if s == "SH" {
            conditions.push(WeatherCondition::Showers);
        } else if s == "TS" {
            conditions.push(WeatherCondition::Thunderstorm);
        } else if s == "FZ" {
            conditions.push(WeatherCondition::Freezing);
        } else if s == "RA" {
            conditions.push(WeatherCondition::Rain);
        } else if s == "DZ" {
            conditions.push(WeatherCondition::Drizzle);
        } else if s == "SN" {
            conditions.push(WeatherCondition::Snow);
        } else if s == "SG" {
            conditions.push(WeatherCondition::SnowGrains);
        } else if s == "IC" {
            conditions.push(WeatherCondition::IceCrystals);
        } else if s == "PL" {
            conditions.push(WeatherCondition::IcePellets);
        } else if s == "GR" {
            conditions.push(WeatherCondition::Hail);
        } else if s == "GS" {
            conditions.push(WeatherCondition::SnowPelletsOrSmallHail);
        } else if s == "UP" {
            conditions.push(WeatherCondition::UnknownPrecipitation);
        } else if s == "FG" {
            conditions.push(WeatherCondition::Fog);
        } else if s == "VA" {
            conditions.push(WeatherCondition::VolcanicAsh);
        } else if s == "BR" {
            conditions.push(WeatherCondition::Mist);
        } else if s == "HZ" {
            conditions.push(WeatherCondition::Haze);
        } else if s == "DU" {
            conditions.push(WeatherCondition::WidespreadDust);
        } else if s == "FU" {
            conditions.push(WeatherCondition::Smoke);
        } else if s == "SA" {
            conditions.push(WeatherCondition::Sand);
        } else if s == "PY" {
            conditions.push(WeatherCondition::Spray);
        } else if s == "SQ" {
            conditions.push(WeatherCondition::Squall);
        } else if s == "PO" {
            conditions.push(WeatherCondition::Dust);
        } else if s == "DS" {
            conditions.push(WeatherCondition::Duststorm);
        } else if s == "SS" {
            conditions.push(WeatherCondition::Sandstorm);
        } else if s == "FC" {
            conditions.push(WeatherCondition::FunnelCloud);
        } else {
            return Err((i, 2, CloudVisibilityError::UnknownData));
        }

        i += 2;
    }

    if conditions.len() < 1 {
        // Need at least one weather condition
        return Err((0, s.len(), CloudVisibilityError::UnknownData));
    }

    let wx = Weather {
        intensity,
        conditions,
    };
    Ok(CloudVisibilityInfo::Weather(wx))
}

pub fn parse_temperatures<'a>(s: &'a str) -> ParserResult<(Data<i32>, Data<i32>), TemperatureError> {
    let chs: Vec<_> = s.chars().collect();

    if s == "/////" {
        return Ok((Unknown, Unknown));
    }
    if s.contains("///") {
        return Err((0, s.len(), TemperatureError::NotTemperatureDewpointPair));
    }
    if !s.contains('/') {
        return Err((0, s.len(), TemperatureError::NotTemperatureDewpointPair));
    }
    if s.contains('R') {
        // To protect against RVRs being interpreted as temperatures
        return Err((0, s.len(), TemperatureError::NotTemperatureDewpointPair));
    }
    if s.contains("SM") {
        // To protect against visibilities measured in statute miles
        return Err((0, s.len(), TemperatureError::NotTemperatureDewpointPair));
    }

    let temp;
    let dewp;

    if s.len() < 5 {
        if let Some(pos) = s.find("/") {
            if pos < 2 {
                return Err((0, pos, TemperatureError::TemperatureNotValid));
            } else {
                return Err((pos + 1, s.len() - pos - 1, TemperatureError::DewpointNotValid));
            }
        } else {
            unreachable!(); // a check earlier in this function prevents this
        }
    }

    let mut i = 0;
    if chs[i] == 'M' {
        if !chs[i + 1].is_digit(10) {
            return Err((i + 1, 1, TemperatureError::TemperatureNotValid));
        }
        if !chs[i + 2].is_digit(10) {
            return Err((i + 2, 1, TemperatureError::TemperatureNotValid));
        }
        temp = -1 * s[i + 1 .. i + 3].parse::<i32>().unwrap();
        i += 4;
    } else {
        if !chs[i].is_digit(10) {
            return Err((i, 1, TemperatureError::TemperatureNotValid));
        }
        if !chs[i + 1].is_digit(10) {
            return Err((i + 1, 1, TemperatureError::TemperatureNotValid));
        }
        temp = s[i .. i + 2].parse().unwrap();
        i += 3;
    }

    if chs[i] == 'M' {
        if !chs[i + 1].is_digit(10) {
            return Err((i + 1, 1, TemperatureError::DewpointNotValid));
        }
        if !chs[i + 2].is_digit(10) {
            return Err((i + 2, 1, TemperatureError::DewpointNotValid));
        }
        dewp = -1 * s[i + 1 .. i + 3].parse::<i32>().unwrap();
    } else {
        if !chs[i].is_digit(10) {
            return Err((i, 1, TemperatureError::DewpointNotValid));
        }
        if !chs[i + 1].is_digit(10) {
            return Err((i + 1, 1, TemperatureError::DewpointNotValid));
        }
        dewp = s[i .. i + 2].parse().unwrap();
    }

    Ok((Known(temp), Known(dewp)))
}

pub fn parse_pressure<'a>(s: &'a str) -> ParserResult<Data<Pressure>, PressureError> {
    if s.len() < 5 {
        return Err((1, s.len(), PressureError::UnitNotValid));
    }

    if s == "Q////"
        || s == "A////" {
            
        return Ok(Unknown);
    }

    let chs: Vec<_> = s.chars().collect();

    if !chs[1].is_digit(10) {
        return Err((1, 1, PressureError::PressureNotValid));
    }
    if !chs[2].is_digit(10) {
        return Err((2, 1, PressureError::PressureNotValid));
    }
    if !chs[3].is_digit(10) {
        return Err((3, 1, PressureError::PressureNotValid));
    }
    if !chs[4].is_digit(10) {
        return Err((4, 1, PressureError::PressureNotValid));
    }

    let pressure = s[1..5].parse().unwrap();

    if chs[0] == 'Q' {
        return Ok(Known(Pressure {
            pressure,
            unit: PressureUnit::Hectopascals,
        }));
    } else if chs[0] == 'A' {
        return Ok(Known(Pressure {
            pressure,
            unit: PressureUnit::InchesMercury,
        }));
    } else {
        return Err((0, 1, PressureError::UnitNotValid));
    }
}
