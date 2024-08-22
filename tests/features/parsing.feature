Feature: METAR Parsing
    METARs are current weather reports from the world of aviation.
    This library's purpose is to parse them, so we test that here.

    Scenario Outline: Parse a METAR
        When "<metar>" is parsed
        Then it parses successfully
        And the station is "<station>"
        And the date is <date> <hour> <minute>
        And the wind direction is <wind_dir>
        And the wind speed is <wind_spd>
        And the wind is varying between <wind_vryng>
        And the wind is gusting to <wind_gusts>
        And the visibility is <visibility>
        And the temperature is <temp>
        And the dewpoint is <dewp>
        And the pressure is <pressure>

        # Currently missing checks for:
        #  - weather
        #  - cloud layers
        #  - no significant cloud
        #  - vertical visibility
        #  - windshear
        #  - RVR

        Examples:
            | station | date | hour | minute | wind_dir | wind_spd | wind_vryng | wind_gusts | visibility | temp | dewp | pressure   | metar                                                                                                                          |
            | EGPC    | 24   | 19   | 50     | unknown  | unknown  | none       | none       | unknown    | unk  | unk  | unknown    | EGPC 241950Z AUTO /////KT //// ///////// ///// Q////                                                                           |
            | EGHI    | 28   | 21   | 20     | 190      | 15 kt    | 140 220    | none       | 6000 m     | 16   | 14   | 1006 hPa   | EGHI 282120Z 19015KT 140V220 6000 RA SCT006 BKN009 16/14 Q1006                                                                 |
            | EGHI    | 06   | 20   | 50     | 310      | 6 kt     | 270 340    | none       | CAVOK      | 13   | 07   | 1017 hPa   | EGHI 062050Z 31006KT 270V340 CAVOK 13/07 Q1017                                                                                 |
            | EGHI    | 07   | 15   | 20     | 190      | 13 kt    | 160 220    | none       | 3000 m     | 15   | 14   | 1012 hPa   | EGHI 071520Z 19013KT 160V220 3000 -RADZ BR BKN006 15/14 Q1012                                                                  |
            | EGHI    | 07   | 17   | 50     | 210      | 10 kt    | none       | none       | 3500 m     | 16   | 15   | 1011 hPa   | EGHI 071750Z 21010KT 3500 -RADZ BR BKN004 16/15 Q1011                                                                          |
            | EGHI    | 08   | 06   | 50     | variable | 3 kt     | none       | none       | CAVOK      | 12   | 10   | 1009 hPa   | EGHI 080650Z VRB03KT CAVOK 12/10 Q1009                                                                                         |
            | EGHI    | 08   | 16   | 50     | 230      | 10 kt    | none       | none       | 9999 m     | 15   | 11   | 1006 hPa   | EGHI 081650Z 23010KT 9999 VCSH FEW018 FEW025TCU 15/11 Q1006                                                                    |
            | EGHI    | 11   | 07   | 50     | 220      | 17 kt    | 190 250    | 28 kt      | 6000 m     | 15   | 14   | 1008 hPa   | EGHI 110750Z 22017G28KT 190V250 6000 -RA FEW007 BKN010 15/14 Q1008 RERA                                                        |
            | EGHI    | 13   | 19   | 50     | 060      | 1 kt     | none       | none       | 9999 m     | 9    | 8    | 1010 hPa   | EGHI 131950Z 06001KT 9999 MIFG NSC 09/08 Q1010                                                                                 |
            | EGHI    | 15   | 06   | 50     | 060      | 1 kt     | none       | none       | 500 m      | 11   | 10   | 1003 hPa   | EGHI 150650Z 06001KT 0500 R20/1000 FG VV/// 11/10 Q1003                                                                        |
            | KEEN    | 06   | 13   | 56     | 0        | 0 kt     | none       | none       | 10 mi      | 6    | -3   | 30.29 inHg | KEEN 061356Z AUTO 00000KT 10SM CLR 06/M03 A3029 RMK AO2 SLP264 T00611028 $                                                     |
            | KLAX    | 06   | 18   | 53     | 260      | 7 kt     | none       | none       | 5 mi       | 19   | 13   | 30.00 inHg | KLAX 061853Z 26007KT 5SM BR SCT006 BKN013 19/13 A3000 RMK AO2 SLP158 T01890133 $                                               |
            | EGGD    | 06   | 19   | 20     | 140      | 7 kt     | none       | none       | 9999 m     | 7    | 6    | 997 hPa    | EGGD 061920Z AUTO 14007KT 9999 SCT035/// //////CB 07/06 Q0997                                                                  |
            | EGSS    | 08   | 17   | 50     | 310      | 6 kt     | 280 360    | none       | 7000 m     | 6    | 5    | 1009 hPa   | EGSS 081750Z AUTO 31006KT 280V360 7000 -RA BKN007 BKN012 BKN019 06/05 Q1009                                                    |
            | KLAX    | 10   | 13   | 35     | 100      | 8 kt     | none       | none       | 0.25 mi    | 16   | 15   | 29.99 inHg | KLAX 101335Z 10008KT 1/4SM R25L/1800V3000FT FG VV001 16/15 A2999 RMK AO2 VIS 1/8V1/2 T01610150                                 |
            | KLAX    | 10   | 17   | 53     | variable | 4 kt     | none       | none       | 5 mi       | 19   | 14   | 30.02 inHg | KLAX 101753Z COR VRB04KT 5SM HZ FEW009 19/14 A3002 RMK AO2 SLP165 T01940139 10194 20156 51006                                  |
            | KLAX    | 04   | 18   | 28     | 20       | 4 kt     | none       | none       | 2.5 mi     | 14   | 12   | 29.96 inHg | KLAX 041828Z 02004KT 2 1/2SM -RA BR BKN007 OVC013 14/12 A2996 RMK AO2 VIS 1 1/2V3 P0002 T01390122 $                            |
            | ESSA    | 08   | 19   | 50     | 220      | 21 kt    | none       | none       | 9999 m     | 6    | 3    | 973 hPa    | ESSA 081950Z 22021KT 9999 OVC025 06/03 Q0973 R01L/29//95 R08/29//95 R01R/29//95 NOSIG                                          |
            | EDDK    | 06   | 19   | 50     | 280      | 8 kt     | none       | none       | CAVOK      | 18   | 10   | 1015 hPa   | EDDK 061950Z AUTO 28008KT CAVOK 18/10 Q1015 BECMG 20006KT                                                                      |
            | EGPC    | 06   | 19   | 50     | 30       | 4 kt     | none       | none       | 9999 m     | 11   | 9    | 1017 hPa   | EGPC 061950Z AUTO 03004KT 9999 NCD 11/09 Q1017                                                                                 |
            | RJAA    | 06   | 19   | 30     | 310      | 10 kt    | none       | none       | 9999 m     | 16   | 16   | 993 hPa    | RJAA 061930Z AUTO 31010KT 9999 SCT002 OVC005 16/16 Q0993 TEMPO 3000 -RA BR RMK A2933                                           |
            | UUDD    | 06   | 19   | 30     | 360      | 3 mps    | 330 030    | none       | CAVOK      | 16   | 8    | 1016 hPa   | UUDD 061930Z 36003MPS 330V030 CAVOK 16/08 Q1016 R32L/CLRD60 NOSIG                                                              |
            | ZGSZ    | 06   | 19   | 00     | 130      | 5 mps    | none       | none       | 9999 m     | 26   | 25   | 1002 hPa   | ZGSZ 061900Z 13005MPS 9999 -TSRA FEW015 FEW026CB BKN030 26/25 Q1002 RESHRA BECMG AT1920 -SHRA BECMG AT1950 21005MPS 2500 +TSRA |
            | RJAA    | 07   | 09   | 00     | 30       | 10 kt    | 350 050    | none       | 9999 m     | 16   | 14   | 1004 hPa   | RJAA 070900Z 03010KT 350V050 9999 -SHRA FEW010 BKN017 16/14 Q1004 WS R34L NOSIG RMK 1CU010 7CU017 A2967                        |
            | RJAA    | 07   | 10   | 00     | 20       | 8 kt     | none       | none       | 9999 m     | 16   | 14   | 1006 hPa   | RJAA 071000Z 02008KT 9999 -SHRA FEW010 BKN016 16/14 Q1006 TEMPO FEW008 BKN012 RMK 1CU010 7CU016 A2971                          |
            | EGGD    | 07   | 13   | 20     | 190      | 9 kt     | none       | none       | 9999 m     | 17   | 15   | 1011 hPa   | EGGD 071320Z 19009KT 9999 4500NW -SHRA FEW015TCU SCT020 BKN040 17/15 Q1011                                                     |
            | KLAX    | 13   | 12   | 53     | 80       | 5 kt     | none       | none       | 2 mi       | 18   | 17   | 29.86 inHg | KLAX 131253Z 08005KT 2SM -DZ BR FEW005 OVC008 18/17 A2986 RMK AO2 SLP108 VIS N-NE 1 1/4 VIS S 2 1/2 DZB10 P0000 T01780167 $    |
            | EDDK    | 15   | 08   | 50     | variable | 3 kt     | none       | none       | 9999 m     | 23   | 11   | 1018 hPa   | EDDK 150850Z AUTO VRB03KT 9999 R24/1000U NCD 23/11 Q1018                                                                       |
            | RJAA    | 17   | 15   | 37     | 160      | 2 kt     | none       | none       | 3700 m     | 19   | 19   | 1010 hPa   | RJAA 171537Z AUTO 16002KT 3700 BR NCD 19/19 Q1010 RMK A2984                                                                    |
            | RJAA    | 24   | 09   | 00     | 220      | 13 kt    | 180 260    | 26 kt      | 9999 m     | 28   | 23   | 1003 hPa   | RJAA 240900Z 22013G26KT 180V260 9999 FEW020 BKN/// 28/23 Q1003 WS R16R WS R16L NOSIG RMK 1CU020 A2962                          |
            | EDDK    | 24   | 08   | 50     | 240      | 8 kt     | 220 280    | none       | 9000 m     | 18   | 17   | 1008 hPa   | EDDK 240850Z AUTO 24008KT 220V280 9000 SHRA BKN009 FEW///TCU 18/17 Q1008 BECMG NSW SCT015 BKN030                               |
            | UUDD    | 27   | 09   | 00     | 310      | 5 mps    | 260 350    | none       | CAVOK      | 31   | 15   | 1020 hPa   | UUDD 270900Z 31005MPS 260V350 CAVOK 31/15 Q1020 R88/CLRD60 NOSIG                                                               |
            | KLAX    | 28   | 12   | 53     | 240      | 5 kt     | none       | none       | 0.125 mi   | 17   | 16   | 29.99 inHg | KLAX 281253Z 24005KT 1/8SM R25L/2600VP6000FT FG VV002 17/16 A2999 RMK AO2 SLP152 VIS E 1/4 T01720161                           |
            | UUDD    | 15   | 12   | 30     | 260      | 7 mps    | none       | none       | CAVOK      | 23   | 09   | 1008 hPa   | UUDD 151230Z 26007MPS CAVOK 23/09 Q1008 WS R32L R32L/CLRD60 NOSIG                                                              |
            | KLAX    | 28   | 12   | 53     | 240      | 5 kt     | none       | none       | 0.125 mi   | 17   | 16   | 29.99 inHg | KLAX 281253Z 24005KT 1/8SM R25L/2600VP6000FT FG VV002 17/16 A2999 RMK AO2 SLP152 VIS E 1/4 T01720161                           |
            | KMHT    | 02   | 18   | 53     | 050      | 6 kt     | none       | none       | 10 mi      | 33   | 13   | 29.81 inHg | KMHT 021853Z COR 05006KT 10SM TS SCT075CB BKN150 33/13 A2981 RMK                                                               |
            | KPVG    | 18   | 19   | 56     | unknown  | unknown  | none       | none       | 10 mi      | unk  | unk  | 29.82 inHg | KPVG 181956Z AUTO 10SM FEW030 SCT035 BKN050 A2982 RMK AO2 SLPNO FZRANO PNO $                                                   |
            | KPVG    | 18   | 18   | 55     | unknown  | unknown  | none       | none       | 10 mi      | unk  | unk  | 29.83 inHg | KPVG 181855Z 10SM FEW025 OVC034 A2983 RMK AO2 PWINO PNO FZRANO RVRNO                                                           |
            | KGWW    | 19   | 11   | 50     | 000      | 0 kt     | none       | none       | unknown    | 14   | 14   | 30.07 inHg | KGWW 191150Z AUTO 00000KT 14/14 A3007 RMK AO2 70001 T01410140 10145 20122                                                      |
            | CYWG    | 19   | 05   | 00     | 160      | 14 kt    | none       | none       | 10 mi      | 19   | 11   | 29.59 inHg | CYWG 190500Z 16014KT 10SM SKC 19/11 A2959 RMK SLP022 DENSITY ALT 1800FT                                                        |
            | CYWG    | 19   | 04   | 00     | 150      | 11 kt    | none       | none       | 9 mi       | 18   | 12   | 29.60 inHg | CYWG 190400Z 15011KT 9SM SKC 18/12 A2960 RMK SLP027 DENSITY ALT 1700FT                                                         |
            | KFCI    | 20   | 03   | 56     | 000      | 0 kt     | none       | none       | unknown    | 15   | 15   | unknown    | KFCI 200356Z AUTO 00000KT SCT070 15/15 RMK AO2 SLPNO T01500150 402610111 PWINO $                                               |
            | KFCI    | 20   | 02   | 56     | 000      | 0 kt     | none       | none       | unknown    | 16   | 16   | unknown    | KFCI 200256Z AUTO 00000KT BKN070 16/16 RMK AO2 SLPNO T01610156 PWINO $                                                         |
            | KPHF    | 19   | 10   | 54     | 000      | 0 kt     | none       | none       | 5 mi       | 11   | 11   | 30.03 inHg | KPHF 191054Z 00000KT 5SM BR CLR 11/11 A3003 RMK AO2 SLP168 T01110111                                                           |
            | MDSD    | 19   | 20   | 00     | 080      | 8 kt     | none       | none       | 9999 m     | 32   | 25   | 1012 hPa   | MDSD 192000Z 08008KT 9999 FEW020CB FEW022 BKN300 32/25 Q1012 CB/NE/E/W                                                         |
            | K2R2    | 22   | 10   | 55     | 020      | 3 kt     | none       | none       | 9 mi       | 10   | 10   | 30.30 inHg | K2R2 221055Z AUTO 02003KT 9SM CLR 10/10 A3030 RMK AO2 T01030103 $                                                              |
            #| UUDD    | 25   | 08   | 30     | 030      | 4 mps    | 350 150    | none       | CAVOK      | 25   | 11   | 1019 hPa   | UUDD 250830Z 03004MPS 350V150 CAVOK 25/11 Q1019 R88/60D NOSIG                                                                  |
            #| EDDK    | 07   | 13   | 50     | 250      | 4 kt     | 220 280    | none       | 9999 m     | 17   | 14   | 1013 hPa   | EDDK 071350Z AUTO 25004KT 220V280 9999 3100 SHRA BKN036 BKN046 SCT///TCU 17/14 Q1013 BECMG NSW                                 |
            # | UUDD    | 29   | 10   | 00     | 120      | 2 mps    | none       | none       | CAVOK      | 22   | 4    | 1022 hPa   | UUDD 291000Z 12002MPS CAVOK 22/04 Q1022 R88/60D NOSIG                                                                          |

    Scenario Outline: Parsing broken METARs fails, but does not panic
        When "<metar>" is parsed
        Then it cleanly fails

        Examples:
        | metar                                                         |
        | EGHI 282120Z 19015KT 140V220 6000 RA SCT006 BKN009 16/14 1006 |
        | EGHI 282120Z 19015KT 140V220 6000 RA SCT006 BKN009 16/14 Q10  |
        | EGPC 211650Z 33026G37KT 9999 FEW021 12/7 Q1026                |
        | EGPC 211650Z 33026G37KT 9999 FEW021 1/70 Q1026                |
        | EGPC 211650Z 33026G37KT 1 FEW021 12/7 Q1026                   |
        | EGPC 211650Z 33026G37KT 100SM FEW021 1/70 Q1026               |
        | EGPC 211650Z 3026KT 9999 FEW021 12/7 Q1026                    |
