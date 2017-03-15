#![no_main]

#[macro_use] extern crate libfuzzer_sys;

extern crate bincode;

use bincode::{serialize, deserialize, SizeLimit};

fuzz_target!(|data| {
    let _decoded = deserialize::<Vec<(f64, f64)>>(data);
});
