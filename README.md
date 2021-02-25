# METAR parsing library for Rust

## Quick usage

This simple usage will print out the parsed data from the METAR.

```rust
use metar::Metar;

fn main() {
  let metar = "EGHI 282120Z 19015KT 140V220 6000 RA SCT006 BKN009 16/14 Q1006";
  let r = Metar::parse(metar).unwrap();
  println!("{:#?}", r);
}
```

## Issues

METARs are complicated structures. If you come across a METAR that doesn't parse
correctly, please open an issue and include the METAR. This will aid in debugging
the issue significantly.
