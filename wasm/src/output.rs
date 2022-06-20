use std::iter;

use js_sys::Array;
use wasm_bindgen::JsValue;

pub struct Output<'a> {
    outputs: Vec<&'a str>,
    colors: Vec<&'a str>,
}

impl<'a> Output<'a> {
    pub fn new() -> Self {
        Output {
            outputs: Vec::new(),
            colors: Vec::new(),
        }
    }
    pub fn from_vec(outputs: Vec<&'a str>, colors: Vec<&'a str>) -> Self {
        Output { outputs, colors }
    }
    pub fn push(&mut self, out: &'a str, col: &'a str) {
        self.outputs.push(out);
        self.colors.push(col);
    }
    pub fn to_jsarray(&self) -> Array {
        let array = Array::new();
        if self.outputs.len() == 0 {
            array.push(&JsValue::from_str("\n"));
        }
        for (c, o) in iter::zip(&self.colors, &self.outputs) {
            let val = format!("{}\n{}", o, c);
            array.push(&JsValue::from_str(&val));
        }
        array
    }
}
