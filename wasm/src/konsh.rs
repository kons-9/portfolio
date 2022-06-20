pub mod file;
pub mod parser;

mod built_in_command;

use std::path::PathBuf;

use crate::konsh::file::FileTree;
use built_in_command::CommandErr;
use built_in_command::{cd, hello_konsh, help, ls};
use parser::Parser;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct Konsh {
    ft: FileTree,
}

impl Konsh {
    pub fn new() -> Self {
        Konsh {
            ft: FileTree::new(),
        }
    }

    pub fn decode<'a>(&mut self, input: &'a str) -> Result<String, CommandErr> {
        let ft = &mut self.ft;
        let parser = Parser::<'a>::new(input);
        let output = match parser.command {
            "cd" => cd(&parser, ft)?,
            "ls" => ls(&parser, ft)?,
            "help" => help(&parser)?,
            "hello_konsh" => hello_konsh(&parser)?,
            _ => Err(CommandErr::new(
                "konsh",
                format!("command not found: {}", parser.command),
            ))?,
        };
        Ok(output)
    }

    pub fn pwd(&self) -> PathBuf {
        return self.ft.pwd.path.clone();
    }
}
