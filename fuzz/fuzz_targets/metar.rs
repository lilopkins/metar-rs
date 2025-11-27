#![no_main]

use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: &[u8]| {
    // fuzzed code goes here
    let s = String::from_utf8_lossy(data);
    let _ = metar::Metar::parse(&s);
});
