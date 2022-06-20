use js_sys::Array;
use output::Output;
use wasm_bindgen::prelude::*;
mod konsh;
mod output;
pub use konsh::Konsh;

#[wasm_bindgen]
extern "C" {
    pub fn alert(s: &str);
}

#[wasm_bindgen]
pub fn add() -> Array {
    // let mut map = Map::new();
    // map = map.set(&JsValue::from_str("hello"), &JsValue::from_str("red"));
    // map = map.set("こんちは", "red");
    // return map;
    let outputs = vec!["hello", "world"];
    let colors = vec!["red", ""];
    let output = Output::from_vec(outputs, colors);

    output.to_jsarray()
}

#[test]
fn add_test() {}
