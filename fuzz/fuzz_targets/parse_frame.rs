#![no_main]

extern crate libwifi;

use libfuzzer_sys::fuzz_target;
use libwifi::parse_frame;

fuzz_target!(|data: &[u8]| {
    let _ = parse_frame(&data, false);
});
