# METAR parsing library for Rust

## Testing

[![metar-rs Automated Testing](https://github.com/lilopkins/metar-rs-autotest/actions/workflows/rust.yml/badge.svg)](https://github.com/lilopkins/metar-rs-autotest/actions/workflows/rust.yml)

This library is being testing regularly with real-world METARs to find failures. The badge above indicates whether these tests are currently successful.

## Quick usage

This simple usage will print out the parsed data from the METAR.

```rust
use metar::Metar;

fn main() {
  let metar = "EGHI 282120Z 19015KT 140V220 6000 RA SCT006 BKN009 16/14 Q1006";
  match Metar::parse(metar) {
    Ok(metar) => println!("{metar:#?}"),
    Err(es) => {
      for e in es {
        eprintln!("{e}");
      }
    }
  }
}
```

## Issues

METARs are complicated structures. If you come across a METAR that doesn't parse
correctly, please open an issue and include the METAR. This will aid in debugging
the issue significantly.
