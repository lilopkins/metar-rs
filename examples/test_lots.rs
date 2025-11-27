use std::{env, fs};

use metar::Metar;

fn main() {
    eprintln!("This tool tests many METARs from a file containing one per line. It will only output METARs that fail.");
    for path in env::args().skip(1) {
        let data = fs::read_to_string(path).unwrap();
        for metar in data.lines() {
            let res = Metar::parse(metar);
            if res.is_err() {
                println!("{metar}");
                if env::var("OUTPUT_ERRORS").is_ok_and(|v| v.eq_ignore_ascii_case("yes")) {
                    for e in res.unwrap_err() {
                        println!("{e}");
                    }
                }
            }
        }
    }
}
