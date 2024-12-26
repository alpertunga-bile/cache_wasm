mod utils;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn greet() -> String {
    let mut greeting: String = String::new();
    greeting.push_str("Hello World");

    return greeting;
}
