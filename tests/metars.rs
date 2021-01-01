use metar::Data::{Known, Unknown};
use metar::DistanceUnit::*;
use metar::PressureUnit::*;
use metar::SpeedUnit::*;
use metar::*;

#[test]
fn test_metar_1() {
    let metar = "EGHI 282120Z 19015KT 140V220 6000 RA SCT006 BKN009 16/14 Q1006";
    let r = Metar::parse(metar).unwrap_or_else(|e| {
        eprintln!("{}", e);
        assert!(false);
        std::process::exit(1);
    });

    assert_eq!(r.station, "EGHI");
    assert_eq!(r.time.date, 28);
    assert_eq!(r.time.hour, 21);
    assert_eq!(r.time.minute, 20);
    assert_eq!(r.wind.dir, Known(WindDirection::Heading(190)));
    assert_eq!(
        r.wind.speed,
        Known(WindSpeed {
            speed: 15,
            unit: Knot
        })
    );
    assert_eq!(r.wind.varying, Some((140, 220)));
    assert_eq!(r.wind.gusting, None);
    assert_eq!(
        r.visibility,
        Known(Visibility {
            visibility: 6000.0,
            unit: Metres
        })
    );
    assert_eq!(r.clouds, Known(Clouds::CloudLayers));
    assert_eq!(r.cloud_layers.len(), 2);
    assert!(r
        .cloud_layers
        .contains(&CloudLayer::Scattered(CloudType::Normal, Some(6))));
    assert!(r
        .cloud_layers
        .contains(&CloudLayer::Broken(CloudType::Normal, Some(9))));
    assert_eq!(r.vert_visibility, None);
    assert_eq!(r.weather.len(), 1);
    assert!(r.weather.contains(&Weather {
        intensity: WeatherIntensity::Moderate,
        conditions: vec![WeatherCondition::Rain,],
    }));
    assert_eq!(r.temperature, Known(16));
    assert_eq!(r.dewpoint, Known(14));
    assert_eq!(
        r.pressure,
        Known(Pressure {
            pressure: 1006.0,
            unit: Hectopascals
        })
    );
}

#[test]
fn test_metar_2() {
    let metar = "EGHI 062050Z 31006KT 270V340 CAVOK 13/07 Q1017";
    let r = Metar::parse(metar).unwrap_or_else(|e| {
        eprintln!("{}", e);
        assert!(false);
        std::process::exit(1);
    });

    assert_eq!(r.station, "EGHI");
    assert_eq!(r.time.date, 06);
    assert_eq!(r.time.hour, 20);
    assert_eq!(r.time.minute, 50);
    assert_eq!(r.wind.dir, Known(WindDirection::Heading(310)));
    assert_eq!(
        r.wind.speed,
        Known(WindSpeed {
            speed: 6,
            unit: Knot
        })
    );
    assert_eq!(r.wind.varying, Some((270, 340)));
    assert_eq!(r.wind.gusting, None);
    assert!(r.visibility.unwrap().is_infinite());
    assert_eq!(r.clouds, Known(Clouds::SkyClear));
    assert_eq!(r.cloud_layers.len(), 0);
    assert_eq!(r.vert_visibility, None);
    assert_eq!(r.weather.len(), 0);
    assert_eq!(r.temperature, Known(13));
    assert_eq!(r.dewpoint, Known(7));
    assert_eq!(
        r.pressure,
        Known(Pressure {
            pressure: 1017.0,
            unit: Hectopascals
        })
    );
}

#[test]
fn test_metar_3() {
    let metar = "EGHI 071520Z 19013KT 160V220 3000 -RADZ BR BKN006 15/14 Q1012";
    let r = Metar::parse(metar).unwrap_or_else(|e| {
        eprintln!("{}", e);
        assert!(false);
        std::process::exit(1);
    });

    assert_eq!(r.station, "EGHI");
    assert_eq!(r.time.date, 07);
    assert_eq!(r.time.hour, 15);
    assert_eq!(r.time.minute, 20);
    assert_eq!(r.wind.dir, Known(WindDirection::Heading(190)));
    assert_eq!(
        r.wind.speed,
        Known(WindSpeed {
            speed: 13,
            unit: Knot
        })
    );
    assert_eq!(r.wind.varying, Some((160, 220)));
    assert_eq!(r.wind.gusting, None);
    assert_eq!(
        r.visibility,
        Known(Visibility {
            visibility: 3000.0,
            unit: Metres
        })
    );
    assert_eq!(r.clouds, Known(Clouds::CloudLayers));
    assert_eq!(r.cloud_layers.len(), 1);
    assert!(r
        .cloud_layers
        .contains(&CloudLayer::Broken(CloudType::Normal, Some(6))));
    assert_eq!(r.vert_visibility, None);
    assert_eq!(r.weather.len(), 2);
    assert!(r.weather.contains(&Weather {
        intensity: WeatherIntensity::Light,
        conditions: vec![WeatherCondition::Rain, WeatherCondition::Drizzle,],
    }));
    assert!(r.weather.contains(&Weather {
        intensity: WeatherIntensity::Moderate,
        conditions: vec![WeatherCondition::Mist,],
    }));
    assert_eq!(r.temperature, Known(15));
    assert_eq!(r.dewpoint, Known(14));
    assert_eq!(
        r.pressure,
        Known(Pressure {
            pressure: 1012.0,
            unit: Hectopascals
        })
    );
}

#[test]
fn test_metar_4() {
    let metar = "EGHI 071750Z 21010KT 3500 -RADZ BR BKN004 16/15 Q1011";
    let r = Metar::parse(metar).unwrap_or_else(|e| {
        eprintln!("{}", e);
        assert!(false);
        std::process::exit(1);
    });

    assert_eq!(r.station, "EGHI");
    assert_eq!(r.time.date, 07);
    assert_eq!(r.time.hour, 17);
    assert_eq!(r.time.minute, 50);
    assert_eq!(r.wind.dir, Known(WindDirection::Heading(210)));
    assert_eq!(
        r.wind.speed,
        Known(WindSpeed {
            speed: 10,
            unit: Knot
        })
    );
    assert_eq!(r.wind.varying, None);
    assert_eq!(r.wind.gusting, None);
    assert_eq!(
        r.visibility,
        Known(Visibility {
            visibility: 3500.0,
            unit: Metres
        })
    );
    assert_eq!(r.clouds, Known(Clouds::CloudLayers));
    assert_eq!(r.cloud_layers.len(), 1);
    assert!(r
        .cloud_layers
        .contains(&CloudLayer::Broken(CloudType::Normal, Some(4))));
    assert_eq!(r.vert_visibility, None);
    assert_eq!(r.weather.len(), 2);
    assert!(r.weather.contains(&Weather {
        intensity: WeatherIntensity::Light,
        conditions: vec![WeatherCondition::Rain, WeatherCondition::Drizzle,],
    }));
    assert!(r.weather.contains(&Weather {
        intensity: WeatherIntensity::Moderate,
        conditions: vec![WeatherCondition::Mist,],
    }));
    assert_eq!(r.temperature, Known(16));
    assert_eq!(r.dewpoint, Known(15));
    assert_eq!(
        r.pressure,
        Known(Pressure {
            pressure: 1011.0,
            unit: Hectopascals
        })
    );
}

#[test]
fn test_metar_5() {
    let metar = "EGHI 080650Z VRB03KT CAVOK 12/10 Q1009";
    let r = Metar::parse(metar).unwrap_or_else(|e| {
        eprintln!("{}", e);
        assert!(false);
        std::process::exit(1);
    });

    assert_eq!(r.station, "EGHI");
    assert_eq!(r.time.date, 08);
    assert_eq!(r.time.hour, 06);
    assert_eq!(r.time.minute, 50);
    assert_eq!(r.wind.dir, Known(WindDirection::Variable));
    assert_eq!(
        r.wind.speed,
        Known(WindSpeed {
            speed: 3,
            unit: Knot
        })
    );
    assert_eq!(r.wind.varying, None);
    assert_eq!(r.wind.gusting, None);
    assert!(r.visibility.unwrap().is_infinite());
    assert_eq!(r.clouds, Known(Clouds::SkyClear));
    assert_eq!(r.cloud_layers.len(), 0);
    assert_eq!(r.vert_visibility, None);
    assert_eq!(r.weather.len(), 0);
    assert_eq!(r.temperature, Known(12));
    assert_eq!(r.dewpoint, Known(10));
    assert_eq!(
        r.pressure,
        Known(Pressure {
            pressure: 1009.0,
            unit: Hectopascals
        })
    );
}

#[test]
fn test_metar_6() {
    let metar = "EGHI 081650Z 23010KT 9999 VCSH FEW018 FEW025TCU 15/11 Q1006";
    let r = Metar::parse(metar).unwrap_or_else(|e| {
        eprintln!("{}", e);
        assert!(false);
        std::process::exit(1);
    });

    assert_eq!(r.station, "EGHI");
    assert_eq!(r.time.date, 08);
    assert_eq!(r.time.hour, 16);
    assert_eq!(r.time.minute, 50);
    assert_eq!(r.wind.dir, Known(WindDirection::Heading(230)));
    assert_eq!(
        r.wind.speed,
        Known(WindSpeed {
            speed: 10,
            unit: Knot
        })
    );
    assert_eq!(r.wind.varying, None);
    assert_eq!(r.wind.gusting, None);
    assert_eq!(
        r.visibility,
        Known(Visibility {
            visibility: 9999.0,
            unit: Metres
        })
    );
    assert_eq!(r.clouds, Known(Clouds::CloudLayers));
    assert_eq!(r.cloud_layers.len(), 2);
    assert!(r
        .cloud_layers
        .contains(&CloudLayer::Few(CloudType::Normal, Some(18))));
    assert!(r
        .cloud_layers
        .contains(&CloudLayer::Few(CloudType::ToweringCumulus, Some(25))));
    assert_eq!(r.vert_visibility, None);
    assert_eq!(r.weather.len(), 1);
    assert!(r.weather.contains(&Weather {
        intensity: WeatherIntensity::InVicinity,
        conditions: vec![WeatherCondition::Showers,],
    }));
    assert_eq!(r.temperature, Known(15));
    assert_eq!(r.dewpoint, Known(11));
    assert_eq!(
        r.pressure,
        Known(Pressure {
            pressure: 1006.0,
            unit: Hectopascals
        })
    );
}

#[test]
fn test_metar_7() {
    let metar = "EGHI 110750Z 22017G28KT 190V250 6000 -RA FEW007 BKN010 15/14 Q1008 RERA";
    let r = Metar::parse(metar).unwrap_or_else(|e| {
        eprintln!("{}", e);
        assert!(false);
        std::process::exit(1);
    });

    assert_eq!(r.station, "EGHI");
    assert_eq!(r.time.date, 11);
    assert_eq!(r.time.hour, 07);
    assert_eq!(r.time.minute, 50);
    assert_eq!(r.wind.dir, Known(WindDirection::Heading(220)));
    assert_eq!(
        r.wind.speed,
        Known(WindSpeed {
            speed: 17,
            unit: Knot
        })
    );
    assert_eq!(r.wind.varying, Some((190, 250)));
    assert_eq!(
        r.wind.gusting,
        Some(WindSpeed {
            speed: 28,
            unit: Knot
        })
    );
    assert_eq!(
        r.visibility,
        Known(Visibility {
            visibility: 6000.0,
            unit: Metres
        })
    );
    assert_eq!(r.clouds, Known(Clouds::CloudLayers));
    assert_eq!(r.cloud_layers.len(), 2);
    assert!(r
        .cloud_layers
        .contains(&CloudLayer::Few(CloudType::Normal, Some(7))));
    assert!(r
        .cloud_layers
        .contains(&CloudLayer::Broken(CloudType::Normal, Some(10))));
    assert_eq!(r.vert_visibility, None);
    assert_eq!(r.weather.len(), 2);
    assert!(r.weather.contains(&Weather {
        intensity: WeatherIntensity::Light,
        conditions: vec![WeatherCondition::Rain,],
    }));
    assert_eq!(r.temperature, Known(15));
    assert_eq!(r.dewpoint, Known(14));
    assert_eq!(
        r.pressure,
        Known(Pressure {
            pressure: 1008.0,
            unit: Hectopascals
        })
    );
}

#[test]
fn test_metar_8() {
    let metar = "EGHI 131950Z 06001KT 9999 MIFG NSC 09/08 Q1010";
    let r = Metar::parse(metar).unwrap_or_else(|e| {
        eprintln!("{}", e);
        assert!(false);
        std::process::exit(1);
    });

    assert_eq!(r.station, "EGHI");
    assert_eq!(r.time.date, 13);
    assert_eq!(r.time.hour, 19);
    assert_eq!(r.time.minute, 50);
    assert_eq!(r.wind.dir, Known(WindDirection::Heading(060)));
    assert_eq!(
        r.wind.speed,
        Known(WindSpeed {
            speed: 01,
            unit: Knot
        })
    );
    assert_eq!(r.wind.varying, None);
    assert_eq!(r.wind.gusting, None);
    assert_eq!(
        r.visibility,
        Known(Visibility {
            visibility: 9999.0,
            unit: Metres
        })
    );
    assert_eq!(r.clouds, Known(Clouds::NoSignificantCloud));
    assert_eq!(r.cloud_layers.len(), 0);
    assert_eq!(r.vert_visibility, None);
    assert_eq!(r.weather.len(), 1);
    assert!(r.weather.contains(&Weather {
        intensity: WeatherIntensity::Moderate,
        conditions: vec![WeatherCondition::Shallow, WeatherCondition::Fog,],
    }));
    assert_eq!(r.temperature, Known(09));
    assert_eq!(r.dewpoint, Known(08));
    assert_eq!(
        r.pressure,
        Known(Pressure {
            pressure: 1010.0,
            unit: Hectopascals
        })
    );
}

#[test]
fn test_metar_9() {
    let metar = "EGHI 150650Z 06001KT 0500 R20/1000 FG VV/// 11/10 Q1003";
    let r = Metar::parse(metar).unwrap_or_else(|e| {
        eprintln!("{}", e);
        assert!(false);
        std::process::exit(1);
    });

    assert_eq!(r.station, "EGHI");
    assert_eq!(r.time.date, 15);
    assert_eq!(r.time.hour, 06);
    assert_eq!(r.time.minute, 50);
    assert_eq!(r.wind.dir, Known(WindDirection::Heading(060)));
    assert_eq!(
        r.wind.speed,
        Known(WindSpeed {
            speed: 01,
            unit: Knot
        })
    );
    assert_eq!(r.wind.varying, None);
    assert_eq!(r.wind.gusting, None);
    assert_eq!(
        r.visibility,
        Known(Visibility {
            visibility: 0500.0,
            unit: Metres
        })
    );
    assert_eq!(r.clouds, Unknown);
    assert_eq!(r.cloud_layers.len(), 0);
    assert_eq!(
        r.vert_visibility,
        Some(VertVisibility::ReducedByUnknownAmount)
    );
    assert_eq!(r.weather.len(), 1);
    assert!(r.weather.contains(&Weather {
        intensity: WeatherIntensity::Moderate,
        conditions: vec![WeatherCondition::Fog,],
    }));
    assert_eq!(r.temperature, Known(11));
    assert_eq!(r.dewpoint, Known(10));
    assert_eq!(
        r.pressure,
        Known(Pressure {
            pressure: 1003.0,
            unit: Hectopascals
        })
    );
}

#[test]
fn test_metar_10() {
    let metar = "KEEN 061356Z AUTO 00000KT 10SM CLR 06/M03 A3029 RMK AO2 SLP264 T00611028 $";
    let r = Metar::parse(metar).unwrap_or_else(|e| {
        eprintln!("{}", e);
        assert!(false);
        std::process::exit(1);
    });

    assert_eq!(r.station, "KEEN");
    assert_eq!(r.time.date, 06);
    assert_eq!(r.time.hour, 13);
    assert_eq!(r.time.minute, 56);
    assert_eq!(r.wind.dir, Known(WindDirection::Heading(0)));
    assert_eq!(
        r.wind.speed,
        Known(WindSpeed {
            speed: 0,
            unit: Knot
        })
    );
    assert_eq!(r.wind.varying, None);
    assert_eq!(r.wind.gusting, None);
    assert_eq!(
        r.visibility,
        Known(Visibility {
            visibility: 10.0,
            unit: StatuteMiles
        })
    );
    assert_eq!(r.clouds, Known(Clouds::SkyClear));
    assert_eq!(r.cloud_layers.len(), 0);
    assert_eq!(r.vert_visibility, None);
    assert_eq!(r.weather.len(), 0);
    assert_eq!(r.temperature, Known(6));
    assert_eq!(r.dewpoint, Known(-3));
    assert_eq!(
        r.pressure,
        Known(Pressure {
            pressure: 3029.0,
            unit: InchesMercury
        })
    );
    assert_eq!(
        r.sea_level_pressure,
        Known(Pressure {
            pressure: 1026.4,
            unit: Hectopascals
        })
    );
}

#[test]
fn test_metar_11() {
    let metar = "KLAX 061853Z 26007KT 5SM BR SCT006 BKN013 19/13 A3000 RMK AO2 SLP158 T01890133 $";
    let r = Metar::parse(metar).unwrap_or_else(|e| {
        eprintln!("{}", e);
        assert!(false);
        std::process::exit(1);
    });

    assert_eq!(r.station, "KLAX");
    assert_eq!(r.time.date, 06);
    assert_eq!(r.time.hour, 18);
    assert_eq!(r.time.minute, 53);
    assert_eq!(r.wind.dir, Known(WindDirection::Heading(260)));
    assert_eq!(
        r.wind.speed,
        Known(WindSpeed {
            speed: 7,
            unit: Knot
        })
    );
    assert_eq!(r.wind.varying, None);
    assert_eq!(r.wind.gusting, None);
    assert_eq!(
        r.visibility,
        Known(Visibility {
            visibility: 5.0,
            unit: StatuteMiles
        })
    );
    assert_eq!(r.clouds, Known(Clouds::CloudLayers));
    assert_eq!(r.cloud_layers.len(), 2);
    assert!(r
        .cloud_layers
        .contains(&CloudLayer::Scattered(CloudType::Normal, Some(6))));
    assert!(r
        .cloud_layers
        .contains(&CloudLayer::Broken(CloudType::Normal, Some(13))));
    assert_eq!(r.vert_visibility, None);
    assert_eq!(r.weather.len(), 1);
    assert!(r.weather.contains(&Weather {
        intensity: WeatherIntensity::Moderate,
        conditions: vec![WeatherCondition::Mist,],
    }));
    assert_eq!(r.temperature, Known(19));
    assert_eq!(r.dewpoint, Known(13));
    assert_eq!(
        r.pressure,
        Known(Pressure {
            pressure: 3000.0,
            unit: InchesMercury
        })
    );
    assert_eq!(
        r.sea_level_pressure,
        Known(Pressure {
            pressure: 1015.8,
            unit: Hectopascals
        })
    );
}

#[test]
fn test_metar_12() {
    let metar = "EGGD 061920Z AUTO 14007KT 9999 SCT035/// //////CB 07/06 Q0997";
    let r = Metar::parse(metar).unwrap_or_else(|e| {
        eprintln!("{}", e);
        assert!(false);
        std::process::exit(1);
    });

    assert_eq!(r.station, "EGGD");
    assert_eq!(r.time.date, 06);
    assert_eq!(r.time.hour, 19);
    assert_eq!(r.time.minute, 20);
    assert_eq!(r.wind.dir, Known(WindDirection::Heading(140)));
    assert_eq!(
        r.wind.speed,
        Known(WindSpeed {
            speed: 7,
            unit: Knot
        })
    );
    assert_eq!(r.wind.varying, None);
    assert_eq!(r.wind.gusting, None);
    assert_eq!(
        r.visibility,
        Known(Visibility {
            visibility: 9999.0,
            unit: Metres
        })
    );
    assert_eq!(r.clouds, Known(Clouds::CloudLayers));
    assert_eq!(r.cloud_layers.len(), 2);
    assert!(r
        .cloud_layers
        .contains(&CloudLayer::Scattered(CloudType::Unknown, Some(35))));
    assert!(r
        .cloud_layers
        .contains(&CloudLayer::Unknown(CloudType::Cumulonimbus, None)));
    assert_eq!(r.vert_visibility, None);
    assert_eq!(r.weather.len(), 0);
    assert_eq!(r.temperature, Known(7));
    assert_eq!(r.dewpoint, Known(6));
    assert_eq!(
        r.pressure,
        Known(Pressure {
            pressure: 997.0,
            unit: Hectopascals
        })
    );
}

#[test]
fn test_metar_13() {
    let metar = "EGSS 081750Z AUTO 31006KT 280V360 7000 -RA BKN007 BKN012 BKN019 06/05 Q1009";
    let r = Metar::parse(metar).unwrap_or_else(|e| {
        eprintln!("{}", e);
        assert!(false);
        std::process::exit(1);
    });

    assert_eq!(r.station, "EGSS");
    assert_eq!(r.time.date, 08);
    assert_eq!(r.time.hour, 17);
    assert_eq!(r.time.minute, 50);
    assert_eq!(r.wind.dir, Known(WindDirection::Heading(310)));
    assert_eq!(
        r.wind.speed,
        Known(WindSpeed {
            speed: 6,
            unit: Knot
        })
    );
    assert_eq!(r.wind.varying, Some((280, 360)));
    assert_eq!(r.wind.gusting, None);
    assert_eq!(
        r.visibility,
        Known(Visibility {
            visibility: 7000.0,
            unit: Metres
        })
    );
    assert_eq!(r.clouds, Known(Clouds::CloudLayers));
    assert_eq!(r.cloud_layers.len(), 3);
    assert!(r
        .cloud_layers
        .contains(&CloudLayer::Broken(CloudType::Normal, Some(7))));
    assert!(r
        .cloud_layers
        .contains(&CloudLayer::Broken(CloudType::Normal, Some(12))));
    assert!(r
        .cloud_layers
        .contains(&CloudLayer::Broken(CloudType::Normal, Some(19))));
    assert_eq!(r.vert_visibility, None);
    assert_eq!(r.weather.len(), 1);
    assert!(r.weather.contains(&Weather {
        intensity: WeatherIntensity::Light,
        conditions: vec![WeatherCondition::Rain,],
    }));
    assert_eq!(r.temperature, Known(6));
    assert_eq!(r.dewpoint, Known(5));
    assert_eq!(
        r.pressure,
        Known(Pressure {
            pressure: 1009.0,
            unit: Hectopascals
        })
    );
}

#[test]
fn test_metar_14() {
    let metar = "KLAX 101335Z 10008KT 1/4SM R25L/1800V3000FT FG VV001 16/15 A2999 RMK AO2 VIS 1/8V1/2 T01610150";
    let r = Metar::parse(metar).unwrap_or_else(|e| {
        eprintln!("{}", e);
        assert!(false);
        std::process::exit(1);
    });

    assert_eq!(r.station, "KLAX");
    assert_eq!(r.time.date, 10);
    assert_eq!(r.time.hour, 13);
    assert_eq!(r.time.minute, 35);
    assert_eq!(r.wind.dir, Known(WindDirection::Heading(100)));
    assert_eq!(
        r.wind.speed,
        Known(WindSpeed {
            speed: 8,
            unit: Knot
        })
    );
    assert_eq!(r.wind.varying, None);
    assert_eq!(r.wind.gusting, None);
    assert_eq!(
        r.visibility,
        Known(Visibility {
            visibility: 0.25,
            unit: StatuteMiles
        })
    );
    assert_eq!(r.clouds, Unknown);
    assert_eq!(r.cloud_layers.len(), 0);
    assert_eq!(r.vert_visibility, Some(VertVisibility::Distance(1)));
    assert_eq!(r.weather.len(), 1);
    assert!(r.weather.contains(&Weather {
        intensity: WeatherIntensity::Moderate,
        conditions: vec![WeatherCondition::Fog,],
    }));
    assert_eq!(r.temperature, Known(16));
    assert_eq!(r.dewpoint, Known(15));
    assert_eq!(
        r.pressure,
        Known(Pressure {
            pressure: 2999.0,
            unit: InchesMercury
        })
    );
    assert_eq!(r.sea_level_pressure, Unknown);
}

#[test]
fn test_metar_15() {
    let metar = "KLAX 101753Z COR VRB04KT 5SM HZ FEW009 19/14 A3002 RMK AO2 SLP165 T01940139 10194 20156 51006";
    let r = Metar::parse(metar).unwrap_or_else(|e| {
        eprintln!("{}", e);
        assert!(false);
        std::process::exit(1);
    });

    assert_eq!(r.station, "KLAX");
    assert_eq!(r.time.date, 10);
    assert_eq!(r.time.hour, 17);
    assert_eq!(r.time.minute, 53);
    assert_eq!(r.wind.dir, Known(WindDirection::Variable));
    assert_eq!(
        r.wind.speed,
        Known(WindSpeed {
            speed: 4,
            unit: Knot
        })
    );
    assert_eq!(r.wind.varying, None);
    assert_eq!(r.wind.gusting, None);
    assert_eq!(
        r.visibility,
        Known(Visibility {
            visibility: 5.0,
            unit: StatuteMiles
        })
    );
    assert_eq!(r.clouds, Known(Clouds::CloudLayers));
    assert_eq!(r.cloud_layers.len(), 1);
    assert!(r
        .cloud_layers
        .contains(&CloudLayer::Few(CloudType::Normal, Some(9))));
    assert_eq!(r.vert_visibility, None);
    assert_eq!(r.weather.len(), 1);
    assert!(r.weather.contains(&Weather {
        intensity: WeatherIntensity::Moderate,
        conditions: vec![WeatherCondition::Haze,],
    }));
    assert_eq!(r.temperature, Known(19));
    assert_eq!(r.dewpoint, Known(14));
    assert_eq!(
        r.pressure,
        Known(Pressure {
            pressure: 3002.0,
            unit: InchesMercury
        })
    );
    assert_eq!(
        r.sea_level_pressure,
        Known(Pressure {
            pressure: 1016.5,
            unit: Hectopascals
        })
    );
}

#[test]
fn test_metar_16() {
    let metar = "KLAX 041828Z 02004KT 2 1/2SM -RA BR BKN007 OVC013 14/12 A2996 RMK AO2 VIS 1 1/2V3 P0002 T01390122 $";
    let r = Metar::parse(metar).unwrap_or_else(|e| {
        eprintln!("{}", e);
        assert!(false);
        std::process::exit(1);
    });

    assert_eq!(r.station, "KLAX");
    assert_eq!(r.time.date, 04);
    assert_eq!(r.time.hour, 18);
    assert_eq!(r.time.minute, 28);
    assert_eq!(r.wind.dir, Known(WindDirection::Heading(020)));
    assert_eq!(
        r.wind.speed,
        Known(WindSpeed {
            speed: 4,
            unit: Knot
        })
    );
    assert_eq!(r.wind.varying, None);
    assert_eq!(r.wind.gusting, None);
    assert_eq!(
        r.visibility,
        Known(Visibility {
            visibility: 2.5,
            unit: StatuteMiles
        })
    );
    assert_eq!(r.clouds, Known(Clouds::CloudLayers));
    assert_eq!(r.cloud_layers.len(), 2);
    assert!(r
        .cloud_layers
        .contains(&CloudLayer::Broken(CloudType::Normal, Some(7))));
    assert!(r
        .cloud_layers
        .contains(&CloudLayer::Overcast(CloudType::Normal, Some(13))));
    assert_eq!(r.vert_visibility, None);
    assert_eq!(r.weather.len(), 2);
    assert!(r.weather.contains(&Weather {
        intensity: WeatherIntensity::Light,
        conditions: vec![WeatherCondition::Rain,],
    }));
    assert!(r.weather.contains(&Weather {
        intensity: WeatherIntensity::Moderate,
        conditions: vec![WeatherCondition::Mist,],
    }));
    assert_eq!(r.temperature, Known(14));
    assert_eq!(r.dewpoint, Known(12));
    assert_eq!(
        r.pressure,
        Known(Pressure {
            pressure: 2996.0,
            unit: InchesMercury
        })
    );
    assert_eq!(r.sea_level_pressure, Unknown);
}

#[test]
fn test_metar_17() {
    let metar =
        "ESSA 081950Z 22021KT 9999 OVC025 06/03 Q0973 R01L/29//95 R08/29//95 R01R/29//95 NOSIG";
    let r = Metar::parse(metar).unwrap_or_else(|e| {
        eprintln!("{}", e);
        assert!(false);
        std::process::exit(1);
    });

    assert_eq!(r.station, "ESSA");
    assert_eq!(r.time.date, 08);
    assert_eq!(r.time.hour, 19);
    assert_eq!(r.time.minute, 50);
    assert_eq!(r.wind.dir, Known(WindDirection::Heading(220)));
    assert_eq!(
        r.wind.speed,
        Known(WindSpeed {
            speed: 21,
            unit: Knot
        })
    );
    assert_eq!(r.wind.varying, None);
    assert_eq!(r.wind.gusting, None);
    assert_eq!(
        r.visibility,
        Known(Visibility {
            visibility: 9999.0,
            unit: Metres
        })
    );
    assert_eq!(r.clouds, Known(Clouds::CloudLayers));
    assert_eq!(r.cloud_layers.len(), 1);
    assert!(r
        .cloud_layers
        .contains(&CloudLayer::Overcast(CloudType::Normal, Some(25))));
    assert_eq!(r.vert_visibility, None);
    assert_eq!(r.weather.len(), 0);
    assert_eq!(r.temperature, Known(6));
    assert_eq!(r.dewpoint, Known(3));
    assert_eq!(
        r.pressure,
        Known(Pressure {
            pressure: 973.0,
            unit: Hectopascals
        })
    );
}

#[test]
fn test_metar_18() {
    let metar = "KC62 220155Z AUTO 28015G21KT 10SM 03/00 A2971 RMK AO2 T00360000";
    let r = Metar::parse(metar).unwrap_or_else(|e| {
        eprintln!("{}", e);
        assert!(false);
        std::process::exit(1);
    });

    assert_eq!(r.station, "KC62");
    assert_eq!(r.time.date, 22);
    assert_eq!(r.time.hour, 01);
    assert_eq!(r.time.minute, 55);
    assert_eq!(r.wind.dir, Known(WindDirection::Heading(280)));
    assert_eq!(
        r.wind.speed,
        Known(WindSpeed {
            speed: 15,
            unit: Knot
        })
    );
    assert_eq!(r.wind.varying, None);
    assert_eq!(
        r.wind.gusting,
        Some(WindSpeed {
            speed: 21,
            unit: Knot
        })
    );
    assert_eq!(
        r.visibility,
        Known(Visibility {
            visibility: 10.0,
            unit: StatuteMiles
        })
    );
    assert_eq!(r.clouds, Unknown);
    assert_eq!(r.cloud_layers.len(), 0);
    assert_eq!(r.vert_visibility, None);
    assert_eq!(r.weather.len(), 0);
    assert_eq!(r.temperature, Known(3));
    assert_eq!(r.dewpoint, Known(0));
    assert_eq!(
        r.pressure,
        Known(Pressure {
            pressure: 2971.0,
            unit: InchesMercury
        })
    );
}

#[test]
fn test_metar_19() {
    let metar = "PACZ 220150Z AUTO 11030G43KT M1/4SM +SN FG VV005 M04/M04 A2863 RMK AO2 SLP706";
    let r = Metar::parse(metar).unwrap_or_else(|e| {
        eprintln!("{}", e);
        assert!(false);
        std::process::exit(1);
    });

    assert_eq!(r.station, "PACZ");
    assert_eq!(r.time.date, 22);
    assert_eq!(r.time.hour, 01);
    assert_eq!(r.time.minute, 50);
    assert_eq!(r.wind.dir, Known(WindDirection::Heading(110)));
    assert_eq!(
        r.wind.speed,
        Known(WindSpeed {
            speed: 30,
            unit: Knot
        })
    );
    assert_eq!(r.wind.varying, None);
    assert_eq!(
        r.wind.gusting,
        Some(WindSpeed {
            speed: 43,
            unit: Knot
        })
    );
    assert_eq!(
        r.visibility,
        Known(Visibility {
            visibility: 0.0,
            unit: StatuteMiles
        })
    );
    assert_eq!(r.clouds, Unknown);
    assert_eq!(r.cloud_layers.len(), 0);
    assert_eq!(r.vert_visibility, Some(VertVisibility::Distance(5)));
    assert_eq!(r.weather.len(), 2);
    assert!(r.weather.contains(&Weather {
        intensity: WeatherIntensity::Heavy,
        conditions: vec![WeatherCondition::Snow,],
    }));
    assert!(r.weather.contains(&Weather {
        intensity: WeatherIntensity::Moderate,
        conditions: vec![WeatherCondition::Fog,],
    }));
    assert_eq!(r.temperature, Known(-4));
    assert_eq!(r.dewpoint, Known(-4));
    assert_eq!(
        r.pressure,
        Known(Pressure {
            pressure: 2863.0,
            unit: InchesMercury
        })
    );
}

#[test]
fn test_metar_20() {
    let metar = "CWPF 220145Z AUTO 25008KT 02/01 RMK AO1 SLP034 T00240009 51016";
    let r = Metar::parse(metar).unwrap_or_else(|e| {
        eprintln!("{}", e);
        assert!(false);
        std::process::exit(1);
    });

    assert_eq!(r.station, "CWPF");
    assert_eq!(r.time.date, 22);
    assert_eq!(r.time.hour, 01);
    assert_eq!(r.time.minute, 45);
    assert_eq!(r.wind.dir, Known(WindDirection::Heading(250)));
    assert_eq!(
        r.wind.speed,
        Known(WindSpeed {
            speed: 08,
            unit: Knot
        })
    );
    assert_eq!(r.wind.varying, None);
    assert_eq!(r.wind.gusting, None);
    assert_eq!(r.visibility, Unknown);
    assert_eq!(r.clouds, Unknown);
    assert_eq!(r.cloud_layers.len(), 0);
    assert_eq!(r.vert_visibility, None);
    assert_eq!(r.weather.len(), 0);
    assert_eq!(r.temperature, Known(2));
    assert_eq!(r.dewpoint, Known(1));
    assert_eq!(r.pressure, Unknown);
}

#[test]
fn test_metar_21() {
    let metar = "CWND 220218Z M01/M01 RMK T10091009";
    let r = Metar::parse(metar).unwrap_or_else(|e| {
        eprintln!("{}", e);
        assert!(false);
        std::process::exit(1);
    });
    println!("{:#?}", r);
    assert_eq!(r.station, "CWND");
    assert_eq!(r.time.date, 22);
    assert_eq!(r.time.hour, 02);
    assert_eq!(r.time.minute, 18);
    assert_eq!(r.wind.dir, Unknown);
    assert_eq!(r.wind.speed, Unknown);
    assert_eq!(r.wind.varying, None);
    assert_eq!(r.wind.gusting, None);
    assert_eq!(r.visibility, Unknown);
    assert_eq!(r.clouds, Unknown);
    assert_eq!(r.cloud_layers.len(), 0);
    assert_eq!(r.vert_visibility, None);
    assert_eq!(r.weather.len(), 0);
    assert_eq!(r.temperature, Known(-1));
    assert_eq!(r.dewpoint, Known(-1));
    assert_eq!(r.pressure, Unknown);
}
