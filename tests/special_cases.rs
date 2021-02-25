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
    assert_eq!(r.cloud_layers.len(), 2);
    assert!(r
        .cloud_layers
        .contains(&CloudLayer::Unknown(CloudType::Unknown, None)));

    assert_eq!(r.temperature, Unknown);
    assert_eq!(r.dewpoint, Unknown);
    assert_eq!(r.pressure, Unknown);
}
