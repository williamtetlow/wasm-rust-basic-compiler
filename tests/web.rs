//! Test suite for the Web and headless browsers.

#![cfg(target_arch = "wasm32")]

extern crate wasm_bindgen_test;
use std::print;

use wasm_bindgen_test::*;

extern crate wasm_basic_compiler;
use wasm_basic_compiler::parse;

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
fn pass() {
    let result = parse("add 7 with 6");

    print!("{}", result);

    assert_eq!(1 + 1, 2);
}
