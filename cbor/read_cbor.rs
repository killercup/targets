#![no_main]

#[macro_use]
extern crate libfuzzer_sys;
extern crate cbor;

fuzz_target!(|data| {
    let mut dec = cbor::Decoder::from_bytes(data);

    for _c in dec.items() { }
});
