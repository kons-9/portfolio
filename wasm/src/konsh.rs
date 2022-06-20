mod built_in_command;
pub mod file;
pub mod parser;

use crate::konsh::file::FileTree;
use crate::output::Output;
use built_in_command::{cd, hello_konsh, help, ls};
use js_sys::Array;
use parser::Parser;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct Konsh {
    ft: FileTree,
}

#[wasm_bindgen]
impl Konsh {
    pub fn new() -> Self {
        Konsh {
            ft: FileTree::new(),
        }
    }

    pub fn decode(&mut self, input: &str) -> Array {
        let ft = &mut self.ft;
        let parser = Parser::new(input);
        let output = match parser.command {
            "cd" => cd(&parser, ft),
            "ls" => ls(&parser, ft),
            "help" => help(&parser),
            "hello_konsh" => hello_konsh(&parser),
            x => Ok(Output::from_vec(
                vec![&format!("{}: command not found: {}", "konsh", x)],
                vec!["red"],
            )
            .to_jsarray()),
        };
        match output {
            Ok(x) => x,
            Err(e) => Output::from_vec(vec![&format!("{}", e)], vec!["red"]).to_jsarray(),
        }
    }

    // pub fn pwd(&self) -> PathBuf {
    //     return self.ft.pwd.path.clone();
    // }
}
