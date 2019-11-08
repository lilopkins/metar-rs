# METAR parsing library for Rust

## Quick usage

This simple usage will print out the parsed data from the METAR.

```rust
extern crate metar;

fn main() {
  let metar = "EGHI 282120Z 19015KT 140V220 6000 RA SCT006 BKN009 16/14 Q1006";
  let r = metar::Metar::parse(metar).unwrap();
  println!("{:#?}", r);
}
```

## Issues?

METARs are complicated structures. If you come across a METAR that doesn't parse
correctly, please open an issue and include the METAR. This will aid in debugging
the issue significantly.

## Definition of a METAR

A METAR can be defined with the following Backus-Naur Form description:

```bnf
<metar> ::= <station> ' ' <observationtime> ' ' <method> ' ' <wind> ' ' <wind_varying> <cloudsvis> ' ' <temps> ' ' <pressure> <remark>

<station> ::= <letter><letter><letter><letter>

<method> ::= '' | 'AUTO'

<observationtime> ::= <obs_day><obs_hour><obs_minute> 'Z'
<obs_day> ::= <obs_day_1><digit> | '3' <obs_day_2>
<obs_day_1> ::= '0' | '1' | '2'
<obs_day_2> ::= '0' | '1'
<obs_hour> ::= <obs_hour_1><digit> | '2' <obs_hour_2>
<obs_hour_1> ::= '0' | '1'
<obs_hour_2> ::= '0' | '1' | '2' | '3'
<obs_minute> ::= <obs_minute_1><digit>
<obs_minute_1> ::= '0' | '1' | '2' | '3' | '4' | '5'

<wind> ::= <wind_dir><digit><digit><wind_gusts> 'KT'
	     | <wind_dir><digit><digit><wind_gusts> 'MPS'
<wind_dir> ::= <angle> | 'VRB' | ''
<wind_gusts> ::= '' | 'G' <digit><digit>

<wind_varying> ::= '' | <angle> 'V' <angle> ' '

<angle> ::= <angle_1><digit><digit> | '3' <angle_2><digit>
<angle_1> ::= '0' | '1' | '2'
<angle_2> ::= '0' | '1' | '2' | '3' | '4' | '5'

<cloudsvis> ::= 'CAVOK' | <visibility> <rvr> <weather> <clouds>
<visibility> ::= <digit><digit><digit><digit> | <digit><digit> 'SM'
			   | 'M' <digit><digit><digit><digit> | 'M' <digit><digit> 'SM'
<clouds> ::= 'CLR' | 'SKC' | 'NCD' | 'NSC' | <cloud_description_list> | <vertical_visibility>
<rvr> ::= <rvr_entry> | <rvr_entry><rvr>
<rvr_entry> ::= 'R' <runway_number> '/' <rvr_vis> <rvr_trend>
<runway_number> ::= <angle_1><digit><runway_modifier> | '3' <angle_2><runway_modifier>
<runway_modifier> ::= '' | 'L' | 'C' | 'R'
<rvr_vis> ::= 'P' <digit><digit><digit><digit> | 'M' <digit><digit><digit><digit>
<rvr_trend> ::= 'D' | 'U' | 'N'

<cloud_description_list> ::= <cloud_description> | <cloud_description> <cloud_description_list>
<cloud_description> ::= <cloud_density> <cloud_floor> <cloud_type>
<cloud_density> ::= 'FEW' | 'SCT' | 'BKN' | 'OVC' | '///'
<cloud_floor> ::= <digit><digit><digit> | '///'
<cloud_type> ::= '' | 'CB' | 'TCU' | '///'

<vertical_visibility> ::= 'VV' <vertical_visibility_distance>
<vertical_visibility_distance> ::= '///' | <digit><digit>

<weather> ::= '' | <weather_cond> | <weather_cond> <weather>
<weather_cond> ::= <weather_intesity><weather_descriptor><weather_preceipitation>
				 | <weather_obscuration>
				 | <weather_other>
				 | <weather_preceipitation><weather_timing>
<weather_intesity> ::= '' | '+' | '-' | 'VC'
<weather_descriptor> ::= '' | 'MI' | 'PR' | 'BC' | 'DR' | 'BL' | 'SH' | 'TS' | 'FZ'
<weather_preceipitation> ::= 'RA' | 'DZ' | 'SN' | 'SG' | 'IC' | 'PL' | 'GR' | 'GS' | 'UP'
<weather_obscuration> ::= 'FG' | 'VA' | 'BR' | 'HZ' | 'DU' | 'FU' | 'SA' | 'PY'
<weather_other> ::= 'SQ' | 'PO' | 'DS' | 'SS' | 'FC'
<weather_timing> ::= 'B' <weather_timing_time> 'E' <weather_timing_time>
				   | 'B' <weather_timing_time>
				   | 'E' <weather_timing_time>
<weather_timing_time> ::= <digit><digit> | <digit><digit><digit><digit>


<temps> ::= <temperature> '/' <temperature>
<temperature> ::= 'M' <digit><digit>
				| <digit><digit>

<pressure> ::= 'Q' <digit><digit><digit><digit>
			 | 'A' <digit><digit><digit><digit>

<remark> ::= ' RMK' ...

<digit> ::= '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9'
```
