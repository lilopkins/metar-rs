use super::types::Data::{Known, Unknown};
use super::types::{
    CloudLayer, CloudType, Clouds, Data, Pressure, Time, VerticalVisibility, Visibility, Weather,
    WeatherCondition, WeatherIntensity, Wind, WindDirection, WindSpeed,
};
use super::Metar;
use pest::iterators::Pair;
use pest::Parser;
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "parser/metar.pest"]
pub struct MetarParser;

impl super::MetarError {
    fn from_pest_err(e: pest::error::Error<Rule>, data: String) -> Self {
        match e.location {
            pest::error::InputLocation::Pos(p) => Self {
                string: data,
                start: p,
                length: 0,
                variant: e.variant,
            },
            pest::error::InputLocation::Span((s, end)) => Self {
                string: data,
                start: s,
                length: end - s,
                variant: e.variant,
            },
        }
    }
}

pub(crate) fn parse(data: String) -> Result<super::Metar, super::MetarError> {
    let res = MetarParser::parse(Rule::metar, &data);
    res.map(|mut pairs| {
        let metar_pair = pairs.next().unwrap();
        metar_pair.into()
    })
    .map_err(|e| super::MetarError::from_pest_err(e, data))
}

impl<'i> From<Pair<'i, Rule>> for Metar {
    #[allow(clippy::too_many_lines, reason = "due a refactor here anyway")]
    fn from(pair: Pair<'i, Rule>) -> Self {
        let mut metar = Metar {
            station: "ZZZZ".to_owned(),
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
            clouds: Known(Clouds::NoCloudDetected),
            cloud_layers: Vec::new(),
            vert_visibility: None,
            weather: Vec::new(),
            temperature: Unknown,
            dewpoint: Unknown,
            pressure: Unknown,
            remarks: None,
        };

        assert_eq!(pair.as_rule(), Rule::metar);
        for part in pair.into_inner() {
            match part.as_rule() {
                Rule::station => part.as_str().clone_into(&mut metar.station),
                Rule::observation_time => metar.time = Time::from(part),
                Rule::wind => metar.wind = Wind::from(part),
                Rule::wind_varying => {
                    let mut hdgs = part.into_inner();
                    let from = hdgs.next().unwrap().as_str();
                    let from = match from {
                        "///" => Data::Unknown,
                        v => Data::Known(v.parse().unwrap()),
                    };
                    let to = hdgs.next().unwrap().as_str();
                    let to = match to {
                        "///" => Data::Unknown,
                        v => Data::Known(v.parse().unwrap()),
                    };
                    metar.wind.varying = Some((from, to));
                }
                Rule::atmos_condition => {
                    if part.as_str() == "CAVOK" {
                        metar.visibility = Known(Visibility::CAVOK);
                        metar.clouds = Known(Clouds::NoCloudDetected);
                    } else if part.as_str() == "SKC" {
                        metar.clouds = Known(Clouds::NoCloudDetected);
                    } else {
                        for c in part.into_inner() {
                            match c.as_rule() {
                                Rule::visibility_horizontal => {
                                    if c.as_str() == "////" {
                                        // Do nothing
                                    } else if c.as_str().ends_with("SM") {
                                        // Statute miles
                                        let mut total = 0f32;
                                        let dist = &c.as_str()[..c.as_str().len() - 2];
                                        let parts = dist.split(' ');
                                        for p in parts {
                                            if p.contains('/') {
                                                let mut parts = p.split('/');
                                                let n: f32 = parts.next().unwrap().parse().unwrap();
                                                let d: f32 = parts.next().unwrap().parse().unwrap();
                                                total += n / d;
                                            } else {
                                                total += p.parse::<f32>().unwrap();
                                            }
                                        }
                                        metar.visibility = Known(Visibility::StatuteMiles(total));
                                    } else {
                                        // Metres
                                        metar.visibility =
                                            Known(Visibility::Metres(c.as_str().parse().unwrap()));
                                    }
                                }
                                Rule::visibility_vertical => {
                                    let data = &c.as_str()[2..];
                                    match data {
                                        "///" => {
                                            metar.vert_visibility =
                                                Some(VerticalVisibility::ReducedByUnknownAmount);
                                        }
                                        _ => {
                                            metar.vert_visibility = Some(
                                                VerticalVisibility::Distance(data.parse().unwrap()),
                                            );
                                        }
                                    }
                                }
                                Rule::wx => metar.weather.push(Weather::from(c)),
                                Rule::cloud => {
                                    metar.clouds = Known(Clouds::CloudLayers);
                                    metar.cloud_layers.push(CloudLayer::from(c));
                                }
                                _ => (),
                            }
                        }
                    }
                }
                Rule::temperatures => {
                    let mut temps = part.into_inner();
                    let temp = temps.next().unwrap();
                    let dewp = temps.next().unwrap();
                    metar.temperature = match temp.as_str() {
                        "//" => Unknown,
                        v => {
                            if let Some(stripped) = v.strip_prefix('M') {
                                Known(-stripped.parse::<i32>().unwrap())
                            } else {
                                Known(v.parse().unwrap())
                            }
                        }
                    };
                    metar.dewpoint = match dewp.as_str() {
                        "//" => Unknown,
                        v => {
                            if let Some(stripped) = v.strip_prefix('M') {
                                Known(-stripped.parse::<i32>().unwrap())
                            } else {
                                Known(v.parse().unwrap())
                            }
                        }
                    };
                }
                Rule::pressure => {
                    let s = part.as_str();
                    let data = &s[1..];
                    if data == "////" {
                        break;
                    }
                    if s.starts_with('Q') {
                        // QNH
                        metar.pressure = Known(Pressure::Hectopascals(data.parse().unwrap()));
                    } else if s.starts_with('A') {
                        // inHg
                        metar.pressure = Known(Pressure::InchesOfMercury(
                            data.parse::<f32>().unwrap() / 100f32,
                        ));
                    } else {
                        unreachable!()
                    }
                }
                Rule::remarks => metar.remarks = Some(part.as_str().to_owned()),
                _ => (),
            }
        }

        metar
    }
}

impl<'i> From<Pair<'i, Rule>> for Time {
    fn from(pair: Pair<'i, Rule>) -> Self {
        let mut time = Time {
            date: 0,
            hour: 0,
            minute: 0,
        };
        assert_eq!(pair.as_rule(), Rule::observation_time);
        for part in pair.into_inner() {
            match part.as_rule() {
                Rule::observation_day => time.date = part.as_str().parse().unwrap(),
                Rule::observation_hour => time.hour = part.as_str().parse().unwrap(),
                Rule::observation_minute => time.minute = part.as_str().parse().unwrap(),
                _ => (),
            }
        }
        time
    }
}

impl<'i> From<Pair<'i, Rule>> for Wind {
    fn from(pair: Pair<'i, Rule>) -> Self {
        let mut wind = Wind {
            dir: Unknown,
            speed: Unknown,
            varying: None,
            gusting: None,
        };
        assert_eq!(pair.as_rule(), Rule::wind);

        if pair.as_str() == "CALM" {
            wind.speed = Known(WindSpeed::Calm);
            return wind;
        }

        let mut speed = None;
        let mut gusting = None;
        let mut unit = None;
        for part in pair.into_inner() {
            match part.as_rule() {
                Rule::wind_dir => {
                    wind.dir = match part.as_str() {
                        "///" => Unknown,
                        "VRB" => Known(WindDirection::Variable),
                        v => Known(WindDirection::Heading(v.parse().unwrap())),
                    };
                }
                Rule::wind_speed => {
                    let mut s = part.as_str();
                    if s == "//" {
                        break;
                    }
                    if s.starts_with('P') {
                        s = &s[1..];
                    }
                    speed = Some(s.parse().unwrap());
                }
                Rule::wind_gusts => {
                    gusting = Some(part.as_str()[1..].parse().unwrap());
                }
                Rule::wind_unit => {
                    let unit_s = part.as_str();
                    unit = match unit_s {
                        "KT" => Some(WindSpeed::Knot(0)),
                        "KPH" => Some(WindSpeed::KilometresPerHour(0)),
                        "MPS" => Some(WindSpeed::MetresPerSecond(0)),
                        _ => unreachable!(),
                    }
                }
                _ => (),
            }
        }

        if let Some(spd) = speed {
            wind.speed = Known(unit.unwrap().clone_changing_contents(spd));
        }
        if let Some(gust) = gusting {
            wind.gusting = unit.map(|u| u.clone_changing_contents(gust));
        }

        wind
    }
}

impl<'i> From<Pair<'i, Rule>> for Weather {
    fn from(pair: Pair<'i, Rule>) -> Self {
        let mut wx = Weather {
            intensity: WeatherIntensity::Moderate,
            conditions: Vec::new(),
        };
        assert_eq!(pair.as_rule(), Rule::wx);
        for part in pair.into_inner() {
            match part.as_rule() {
                Rule::wx_intensity => {
                    wx.intensity = match part.as_str() {
                        "+" => WeatherIntensity::Heavy,
                        "-" => WeatherIntensity::Light,
                        "VC" => WeatherIntensity::InVicinity,
                        _ => unreachable!(),
                    }
                }
                Rule::wx_condition => {
                    let cond = match part.as_str() {
                        "MI" => WeatherCondition::Shallow,
                        "PR" => WeatherCondition::Partial,
                        "BC" => WeatherCondition::Patches,
                        "DR" => WeatherCondition::LowDrifting,
                        "BL" => WeatherCondition::Blowing,
                        "SH" => WeatherCondition::Showers,
                        "TS" => WeatherCondition::Thunderstorm,
                        "FZ" => WeatherCondition::Freezing,
                        "RA" => WeatherCondition::Rain,
                        "DZ" => WeatherCondition::Drizzle,
                        "SN" => WeatherCondition::Snow,
                        "SG" => WeatherCondition::SnowGrains,
                        "IC" => WeatherCondition::IceCrystals,
                        "PL" => WeatherCondition::IcePellets,
                        "GR" => WeatherCondition::Hail,
                        "GS" => WeatherCondition::SnowPelletsOrSmallHail,
                        "UP" => WeatherCondition::UnknownPrecipitation,
                        "FG" => WeatherCondition::Fog,
                        "VA" => WeatherCondition::VolcanicAsh,
                        "BR" => WeatherCondition::Mist,
                        "HZ" => WeatherCondition::Haze,
                        "DU" => WeatherCondition::WidespreadDust,
                        "FU" => WeatherCondition::Smoke,
                        "SA" => WeatherCondition::Sand,
                        "PY" => WeatherCondition::Spray,
                        "SQ" => WeatherCondition::Squall,
                        "PO" => WeatherCondition::Dust,
                        "DS" => WeatherCondition::Duststorm,
                        "SS" => WeatherCondition::Sandstorm,
                        "FC" => WeatherCondition::FunnelCloud,
                        _ => unreachable!(),
                    };
                    wx.conditions.push(cond);
                }
                _ => (),
            }
        }
        wx
    }
}

impl<'i> From<Pair<'i, Rule>> for CloudLayer {
    fn from(pair: Pair<'i, Rule>) -> Self {
        assert_eq!(pair.as_rule(), Rule::cloud);
        let mut density = "";
        let mut typ = CloudType::Normal;
        let mut floor = None;

        for part in pair.into_inner() {
            match part.as_rule() {
                Rule::cloud_density => density = part.as_str(),
                Rule::cloud_type => match part.as_str() {
                    "///" => typ = CloudType::Unknown,
                    "CB" => typ = CloudType::Cumulonimbus,
                    "TCU" => typ = CloudType::ToweringCumulus,
                    _ => unreachable!(),
                },
                Rule::cloud_floor => match part.as_str() {
                    "///" => floor = None,
                    _ => floor = Some(part.as_str().parse().unwrap()),
                },
                _ => (),
            }
        }

        match density {
            "///" => CloudLayer::Unknown(typ, floor),
            "FEW" => CloudLayer::Few(typ, floor),
            "SCT" => CloudLayer::Scattered(typ, floor),
            "BKN" => CloudLayer::Broken(typ, floor),
            "OVC" => CloudLayer::Overcast(typ, floor),
            _ => unreachable!(),
        }
    }
}
