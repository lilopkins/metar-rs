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
