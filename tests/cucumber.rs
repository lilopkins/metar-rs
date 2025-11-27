use cucumber::{then, when, World as _};
use metar::{Data, Metar, OwnedMetarError, Pressure, Visibility, Wind, WindDirection, WindSpeed};

#[derive(cucumber::World, Debug, Default)]
struct World {
    has_parsed: bool,
    metar: Option<Result<Metar, Vec<OwnedMetarError>>>,
}

impl World {
    fn metar(&self) -> &Metar {
        self.metar.as_ref().unwrap().as_ref().unwrap()
    }
}

#[when(expr = "{string} is parsed")]
fn parse_metar(w: &mut World, metar: String) {
    w.metar = Some(
        Metar::parse(metar.as_str()).map_err(|es| es.into_iter().map(|e| e.into_owned()).collect()),
    );
    w.has_parsed = true;
}

#[then(expr = "it parses successfully")]
fn parse_successful(w: &mut World) {
    assert!(w.has_parsed);
    assert!(w.metar.is_some());

    let metar_result = w.metar.as_ref().unwrap();
    if let Err(es) = &metar_result {
        let mut failure = String::new();
        for e in es {
            failure.push_str(&e.to_string());
        }
        panic!("{failure}")
    }
}

#[then(expr = "it cleanly fails")]
fn parse_fails_cleanly(w: &mut World) {
    assert!(w.has_parsed);
    assert!(w.metar.is_some());

    let metar_result = w.metar.as_ref().unwrap();
    assert!(metar_result.is_err());
}

#[then(expr = "the station is {string}")]
fn check_station(w: &mut World, station: String) {
    let metar = w.metar();
    assert_eq!(station, metar.station);
}

#[then(expr = "the date is {int} {int} {int}")]
fn check_date(w: &mut World, date: u8, hour: u8, minute: u8) {
    let metar = w.metar();
    assert_eq!(date, metar.time.date);
    assert_eq!(hour, metar.time.hour);
    assert_eq!(minute, metar.time.minute);
}

#[then(expr = "the wind direction is {int}")]
fn check_wind_dir(w: &mut World, target_dir: u32) {
    let metar = w.metar();
    if let Wind::Present {
        dir: WindDirection::Heading(Data::Known(dir)),
        ..
    } = metar.wind
    {
        assert_eq!(dir, target_dir);
    } else {
        panic!();
    }
}

#[then(expr = "the wind direction is variable")]
fn check_wind_dir_var(w: &mut World) {
    let metar = w.metar();
    assert!(matches!(
        metar.wind,
        Wind::Present {
            dir: WindDirection::Variable,
            ..
        }
    ));
}

#[then(expr = "the wind direction is unknown")]
fn check_wind_dir_unk(w: &mut World) {
    let metar = w.metar();
    assert!(matches!(
        metar.wind,
        Wind::Present {
            dir: WindDirection::Heading(Data::Unknown),
            ..
        }
    ));
}

#[then(expr = "the wind speed is unknown")]
fn check_wind_spd_unk(w: &mut World) {
    let metar = w.metar();
    if let Wind::Present { speed, .. } = metar.wind {
        match speed {
            WindSpeed::KilometresPerHour { speed, .. }
            | WindSpeed::Knots { speed, .. }
            | WindSpeed::MetresPerSecond { speed, .. } => assert_eq!(speed, Data::Unknown),
            _ => panic!(),
        }
    } else {
        panic!();
    }
}

#[then(expr = "the wind is varying between {int} {int}")]
fn check_wind_varying(w: &mut World, start: u32, end: u32) {
    let metar = w.metar();
    if let Wind::Present {
        varying: Some((from, to)),
        ..
    } = metar.wind
    {
        assert_eq!(Data::Known(start), from);
        assert_eq!(Data::Known(end), to);
    } else {
        panic!();
    }
}

#[then(expr = "the wind is varying between unk {int}")]
fn check_wind_varying_unknown_start(w: &mut World, end: u32) {
    let metar = w.metar();
    if let Wind::Present {
        varying: Some((from, to)),
        ..
    } = metar.wind
    {
        assert_eq!(Data::Unknown, from);
        assert_eq!(Data::Known(end), to);
    } else {
        panic!();
    }
}

#[then(expr = "the wind is varying between {int} unk")]
fn check_wind_varying_unknown_end(w: &mut World, start: u32) {
    let metar = w.metar();
    if let Wind::Present {
        varying: Some((from, to)),
        ..
    } = metar.wind
    {
        assert_eq!(Data::Known(start), from);
        assert_eq!(Data::Unknown, to);
    } else {
        panic!();
    }
}

#[then(expr = "the wind is varying between unk unk")]
fn check_wind_varying_unknown_both(w: &mut World) {
    let metar = w.metar();
    if let Wind::Present {
        varying: Some((from, to)),
        ..
    } = metar.wind
    {
        assert_eq!(Data::Unknown, from);
        assert_eq!(Data::Unknown, to);
    } else {
        panic!();
    }
}

#[then(expr = "the wind is varying between none")]
fn wind_not_varying(_w: &mut World) {}

#[then(expr = "the wind speed is {int} kt")]
fn check_wind_spd_knots(w: &mut World, spd: u32) {
    let metar = w.metar();
    if let Wind::Present {
        speed: WindSpeed::Knots { speed, .. },
        ..
    } = metar.wind
    {
        assert_eq!(Data::Known(spd), speed);
    } else {
        panic!();
    }
}

#[then(expr = "the wind speed is {int} mps")]
fn check_wind_spd_mps(w: &mut World, spd: u32) {
    let metar = w.metar();
    if let Wind::Present {
        speed: WindSpeed::MetresPerSecond { speed, .. },
        ..
    } = metar.wind
    {
        assert_eq!(Data::Known(spd), speed);
    } else {
        panic!();
    }
}

#[then(expr = "the wind is gusting to {int} kt")]
fn check_wind_gusting_knots(w: &mut World, spd: u32) {
    let metar = w.metar();
    if let Wind::Present {
        speed: WindSpeed::Knots { gusting, .. },
        ..
    } = metar.wind
    {
        assert_eq!(Some(Data::Known(spd)), gusting);
    } else {
        panic!();
    }
}

#[then(expr = "the wind is gusting to none")]
fn wind_not_gusting(w: &mut World) {
    let metar = w.metar();
    if let Wind::Present { speed, .. } = metar.wind {
        match speed {
            WindSpeed::KilometresPerHour { gusting, .. }
            | WindSpeed::Knots { gusting, .. }
            | WindSpeed::MetresPerSecond { gusting, .. } => assert_eq!(None, gusting),
            _ => panic!(),
        }
    } else {
        panic!();
    }
}

#[then(expr = "the visibility is {int} m")]
fn check_visibility_metres(w: &mut World, visibility: u16) {
    let metar = w.metar();
    assert_eq!(Visibility::Metres(visibility), metar.visibility.unwrap());
}

#[then(expr = "the visibility is {float} mi")]
fn check_visibility_miles(w: &mut World, visibility: f32) {
    let metar = w.metar();
    assert_eq!(
        Visibility::StatuteMiles(visibility),
        metar.visibility.unwrap()
    );
}

#[then(expr = "the visibility is CAVOK")]
fn check_visibility_cavok(w: &mut World) {
    let metar = w.metar();
    assert_eq!(Visibility::CAVOK, metar.visibility.unwrap());
}

#[then(expr = "the visibility is unknown")]
fn check_visibility_unk(w: &mut World) {
    let metar = w.metar();
    assert_eq!(Data::Unknown, metar.visibility);
}

#[then(expr = "the temperature is {int}")]
fn check_temp(w: &mut World, temp: i32) {
    let metar = w.metar();
    assert_eq!(temp, metar.temperature.unwrap());
}

#[then(expr = "the temperature is unk")]
fn check_temp_unk(w: &mut World) {
    let metar = w.metar();
    assert_eq!(Data::Unknown, metar.temperature);
}

#[then(expr = "the dewpoint is {int}")]
fn check_dewp(w: &mut World, dewp: i32) {
    let metar = w.metar();
    assert_eq!(dewp, metar.dewpoint.unwrap());
}

#[then(expr = "the dewpoint is unk")]
fn check_dewp_unk(w: &mut World) {
    let metar = w.metar();
    assert_eq!(Data::Unknown, metar.dewpoint);
}

#[then(expr = "the pressure is {int} hPa")]
fn check_pressure_hpa(w: &mut World, pressure: u16) {
    let metar = w.metar();
    assert_eq!(
        Pressure::Hectopascals(Data::Known(pressure)),
        metar.pressure
    );
}

#[then(expr = "the pressure is {float} inHg")]
fn check_pressure_inhg(w: &mut World, pressure: f32) {
    let metar = w.metar();
    assert_eq!(
        Pressure::InchesOfMercury(Data::Known(pressure)),
        metar.pressure
    );
}

#[then(expr = "the pressure is unknown")]
fn check_pressure_unk(w: &mut World) {
    let metar = w.metar();
    match metar.pressure {
        Pressure::Hectopascals(v) => assert_eq!(v, Data::Unknown),
        Pressure::InchesOfMercury(v) => assert_eq!(v, Data::Unknown),
    }
}

fn main() {
    futures::executor::block_on(World::run("tests/features"));
}
