//! Test suite for the Web and headless browsers.

#![cfg(target_arch = "wasm32")]

extern crate wasm_bindgen_test;
use std::assert_eq;

use cacher_wasm::{
    get_compressed_cacher_info, get_decompressed_data, CacherOptions, CacherReturnInfo,
};
use wasm_bindgen_test::*;

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
fn hello_world_test() {
    let options = CacherOptions::new();
    let data = String::from("Hello World");

    let compressed_info: Vec<u8> = get_compressed_cacher_info(&options, &data);
    let decompressed_data: CacherReturnInfo = get_decompressed_data(&compressed_info);

    assert_eq!(data, decompressed_data.data());
}
