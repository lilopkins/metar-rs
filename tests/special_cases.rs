use metar::{Data::*, *};

#[test]
fn test_all_blank() {
    let metar = "EGPC 241950Z AUTO /////KT //// ///////// ///// Q////";
    let r = Metar::parse(metar);
    if r.is_err() {
        let e = r.unwrap_err();
        eprintln!("{}", e);
        eprintln!("{:#?}", e);
        assert!(false);
        return;
    }
    let r = r.unwrap();
    assert_eq!(r.station, "EGPC");

    assert_eq!(r.time.date, 24);
    assert_eq!(r.time.hour, 19);
    assert_eq!(r.time.minute, 50);

    assert_eq!(r.wind.dir, Unknown);
    assert_eq!(r.wind.speed, Unknown);
    assert_eq!(r.wind.varying, None);
    assert_eq!(r.wind.gusting, None);

    assert_eq!(r.visibility, Unknown);
    assert_eq!(r.clouds, Known(Clouds::CloudLayers));
    assert_eq!(r.cloud_layers.len(), 1);
    assert!(r
        .cloud_layers
        .contains(&CloudLayer::Unknown(CloudType::Unknown, None)));

    assert_eq!(r.temperature, Unknown);
    assert_eq!(r.dewpoint, Unknown);
    assert_eq!(r.pressure, Unknown);
}

#[test]
fn test_doesnt_panic_with_bad_pressure() {
    let metar = "EGHI 282120Z 19015KT 140V220 6000 RA SCT006 BKN009 16/14 1006";
    // Test fails automatically if this panics
    let r = Metar::parse(metar);
    assert!(r.is_err());

    let metar = "EGHI 282120Z 19015KT 140V220 6000 RA SCT006 BKN009 16/14 Q10";
    // Test fails automatically if this panics
    let r = Metar::parse(metar);
    assert!(r.is_err());
}

#[test]
fn test_doesnt_panic_with_bad_temps() {
    let metar = "EGPC 211650Z 33026G37KT 9999 FEW021 12/7 Q1026";
    // Test fails automatically if this panics
    let r = Metar::parse(metar);
    assert!(r.is_err());

    let metar = "EGPC 211650Z 33026G37KT 9999 FEW021 1/70 Q1026";
    // Test fails automatically if this panics
    let r = Metar::parse(metar);
    assert!(r.is_err());
}

#[test]
fn test_doesnt_panic_with_bad_visibility() {
    let metar = "EGPC 211650Z 33026G37KT 1 FEW021 12/7 Q1026";
    // Test fails automatically if this panics
    let r = Metar::parse(metar);
    assert!(r.is_err());

    let metar = "EGPC 211650Z 33026G37KT 100SM FEW021 1/70 Q1026";
    // Test fails automatically if this panics
    let r = Metar::parse(metar);
    assert!(r.is_err());
}

#[test]
fn test_doesnt_panic_with_bad_wind() {
    let metar = "EGPC 211650Z 3026KT 9999 FEW021 12/7 Q1026";
    // Test fails automatically if this panics
    let r = Metar::parse(metar);
    assert!(r.is_err());
}
