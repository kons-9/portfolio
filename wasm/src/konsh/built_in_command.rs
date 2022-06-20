use crate::konsh::file::FileTree;
use crate::konsh::Parser;
use crate::style::Style;

use std::error::Error;
use std::fmt;

#[derive(Clone, Debug)]
pub struct CommandErr {
    name: String,
    error: String,
}

impl CommandErr {
    pub fn new(name: impl Into<String>, error: impl Into<String>) -> Self {
        CommandErr {
            name: name.into(),
            error: error.into(),
        }
    }
}
impl fmt::Display for CommandErr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} :{}", self.name, self.error)
    }
}
type CommandOut = Result<String, CommandErr>;

/// 一つずつディレクトリを変えていく
/// 今は親から子にしか行けない
pub fn cd(parser: &Parser, ft: &mut FileTree) -> CommandOut {
    if parser.sub_command.len() != 1 {
        return Err(CommandErr::new("cd", "invalid sub command"));
    }
    let to_dir = parser.sub_command[0];
    if let Some(dir_info) = ft.find_current_dir(to_dir) {
        ft.pwd = dir_info;
        Ok("".to_string())
    } else {
        Err(CommandErr::new(
            "cd",
            format!("no such file of directory: {}", to_dir),
        ))
    }
}
pub fn hello_konsh(parser: &Parser) -> CommandOut {
    Ok("hello".to_string())
}
pub fn ls(parser: &Parser, ft: &FileTree) -> CommandOut {
    Ok("hello".to_string())
}
pub fn help(parser: &Parser) -> CommandOut {
    Ok("hello".to_string())
}
