use std::{fmt::format, fs::File};

use std::path::PathBuf;
use wasm_bindgen::prelude::*;
mod file;
use file::FileTree;
mod konsh;

#[wasm_bindgen]
extern "C" {
    pub fn alert(s: &str);
}

#[wasm_bindgen]
pub fn add() {
    let f = FileTree::new();
    alert(&PathBuf::from("/").is_dir().to_string());
    alert(&PathBuf::from("/User").is_file().to_string());
}

#[wasm_bindgen]
pub fn sample() {
    let a = FileTree::new();
    let b = FileTree::json_to_filetree();
}

#[test]
fn add_test() {}
