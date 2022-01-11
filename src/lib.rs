mod ast;
mod interpreter;
mod parser;
mod utils;

use interpreter::Compile;
use interpreter::Interpreter;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsValue;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
pub fn parse(input: &str) -> JsValue {
    let result = Interpreter::from_source(input);

    match result {
        Ok(result) => result.into(),
        Err(_) => "errror :(".into(),
    }
}
