# METAR parsing library for Rust

## Quick usage

This simple usage will print out the parsed data from the METAR.

```rust
extern crate metar;

fn main() {
  let metar = "EGHI 282120Z 19015KT 140V220 6000 RA SCT006 BKN009 16/14 Q1006".to_string();
  let r = metar::Metar::parse(metar).unwrap();
  println!("{:#?}", r);
}
```

## Issues?

METARs are complicated structures. If you come across a METAR that doesn't parse
correctly, please open an issue and include the METAR. This will aid in debugging
the issue significantly.

## Definition of a METAR

A METAR can be defined with the following EBNF description:

```
letter = "A" | "B" | "C" | "D" | "E" | "F" | "G" | "H" | "I" | "J" | "K" | "L" | "M" | "N" | "O"
	| "P" | "Q" | "R" | "S" | "T" | "U" | "V" | "W" | "X" | "Y" | "Z".
digit = "0" | "1" | "2" | "3" | "4" | "5" | "6" | "7" | "8" | "9".
digit6 = "0" | "1" | "2" | "3" | "4" | "5" | "6".
digit2 = "0" | "1" | "2".
heading = ( "3" digit6 digit | digit2 digit digit ).
rwheading = ( "3" digit6 | digit2 digit ).
wxtype = "DZ" | "RA" | "SN" | "SG" | "PL" | "IC" | "GR" | "GS" | "UP"
	| "FG" | "BR" | "SA" | "DU" | "HZ" | "FU" | "VA"
	| "PO" | "SQ" | "FC" | "DS" | "SS".
wxcharacteristic = "TS" | "SH" | "FZ" | "BL" | "DR" | "MI" | "BC" | "PR".
wxintensity = "-" | "+" | "VC".
cloudamount = "FEW" | "SCT" | "BKN" | "OVC".
cloudtype = [ "CB" | "TCU" ].
temp = [ "M" ] digit digit.

station = letter letter letter letter.
time = digit digit digit digit digit digit "Z".
wind = ( heading | "VRB" | "ABV" ) digit digit [ "G" digit digit ] ( "KT" | "MPS" ) " " [ heading "V" heading ].
visibility = [ "M" ] ( digit digit digit digit ) | ( digit digit "SM" ) | "CAVOK" | "NSC" | "SKC".
rvr = { "R" rwheading [ "R" | "L" | "C" ] "/" [ "P" | "M" ] digit digit digit digit [ "V" digit digit digit digit ] [ "D" | "U" | "N" ] " " }.
weather = { [ wxintensity ] [ wxcharacteristic ] wxtype " " }.
clouds = "NCD" | { cloudamount digit digit digit cloudtype }.
vertvisibility = "VV" digit digit digit.
temperatures = temp "/" temp.
pressure = ( "Q" | "A" ) digit digit digit digit.

metar = station " " time " " ( "NIL" | ( [ "AUTO " ] wind " " visibility " " rvr weather clouds [ vertvisibility " " ] temperatures pressure "..." ) ).
```

A (Perl-compatible) Regular expression reading a METAR could look like this:

`(?P<station>[A-Z0-9]{4}) (?P<time>[0-9]{6}Z) (?P<data>NIL|(?:AUTO )?(?P<wind_dir>[0-9]{3}|VRB|ABV)(?P<wind_speed>[0-9]{2})(?:G(?P<wind_gusts>[0-9]{2}))?(?P<wind_unit>KT|MPS) (?:(?P<wind_varying_from>[0-9]{3})V(?P<wind_varying_to>[0-9]{3}))? (?P<visibility>CAVOK|NSC|SKC|M?[0-9]{2}SM|M?[0-9]{4}) (?P<rvr>(?:R[0-9]{2}[LCR]?\/[PM]?[0-9]{4}(?:V[0-9]{4})?[DUN]? )*)(?P<wx>(?:(?:VC|\-|\+)?(?:TS|SH|FZ|BL|DR|MI|BC|PR)?(?:DZ|RA|SN|SG|PL|IC|GR|GS|UP|FG|BR|SA|DU|HZ|FU|VA|PO|SQ|FC|DS|SS) )*)(?P<cloud>NCD|(?:(?:FEW|SCT|BKN|OVC)[0-9]{3}(?:CB|TCU)? )*)(?:VV(?P<vert_visibility>[0-9]{3}) )?(?P<temperature>M?[0-9]{2})\/(?P<dewpoint>M?[0-9]{2}) (?P<pressure>(?:Q|A)[0-9]{4}))(?: RMK (?P<remarks>.*))?$`
