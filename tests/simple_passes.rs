use metar::Metar;

macro_rules! simple_pass {
    ($ident: ident, $metar: expr) => {
        #[test]
        fn $ident() {
            let input = $metar;
            let m = Metar::parse(input);
            if let Err(e) = &m {
                eprintln!("{}", e);
            }
            assert!(m.is_ok());
        }
    };
}

simple_pass!(test_metar_1, "EDDK 061950Z AUTO 28008KT CAVOK 18/10 Q1015 BECMG 20006KT");
simple_pass!(test_metar_2, "EGPC 061950Z AUTO 03004KT 9999 NCD 11/09 Q1017");
simple_pass!(test_metar_3, "RJAA 061930Z AUTO 31010KT 9999 SCT002 OVC005 16/16 Q0993 TEMPO 3000 -RA BR RMK A2933");
simple_pass!(test_metar_4, "UUDD 061930Z 36003MPS 330V030 CAVOK 16/08 Q1016 R32L/CLRD60 NOSIG");
simple_pass!(test_metar_5, "ZGSZ 061900Z 13005MPS 9999 -TSRA FEW015 FEW026CB BKN030 26/25 Q1002 RESHRA BECMG AT1920 -SHRA BECMG AT1950 21005MPS 2500 +TSRA");
simple_pass!(test_metar_6, "RJAA 070900Z 03010KT 350V050 9999 -SHRA FEW010 BKN017 16/14 Q1004 WS R34L NOSIG RMK 1CU010 7CU017 A2967");
simple_pass!(test_metar_7, "RJAA 071000Z 02008KT 9999 -SHRA FEW010 BKN016 16/14 Q1006 TEMPO FEW008 BKN012 RMK 1CU010 7CU016 A2971");
simple_pass!(test_metar_8, "EGGD 071320Z 19009KT 9999 4500NW -SHRA FEW015TCU SCT020 BKN040 17/15 Q1011");
simple_pass!(test_metar_9, "KLAX 131253Z 08005KT 2SM -DZ BR FEW005 OVC008 18/17 A2986 RMK AO2 SLP108 VIS N-NE 1 1/4 VIS S 2 1/2 DZB10 P0000 T01780167 $");
simple_pass!(test_metar_10, "EDDK 150850Z AUTO VRB03KT 9999 R24/1000U NCD 23/11 Q1018");
simple_pass!(test_metar_11, "RJAA 171537Z AUTO 16002KT 3700 BR NCD 19/19 Q1010 RMK A2984");
