use metar::*;

#[test]
fn test_doesnt_panic_with_bad_pressure() {
    let metar = "EGHI 282120Z 19015KT 140V220 6000 RA SCT006 BKN009 16/14 1006";
    // Test fails automatically if this panics
    let r = Metar::parse(metar);
    assert!(r.is_err());
}
