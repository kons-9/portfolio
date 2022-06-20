use crate::konsh::file::FileTree;
use crate::konsh::Parser;
use crate::output::Output;

use js_sys::Array;
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
        write!(f, "{}: {}", self.name, self.error)
    }
}
type CommandOut = Result<Array, CommandErr>;

/// 一つずつディレクトリを変えていく
/// 今は親から子にしか行けない
pub fn cd(parser: &Parser, ft: &mut FileTree) -> CommandOut {
    if parser.sub_command.len() != 1 {
        return Err(CommandErr::new("cd", "invalid sub command"));
    }
    let to_dir = parser.sub_command[0];
    if let Some(dir_info) = ft.find_current_dir(to_dir) {
        ft.pwd = dir_info;
        Ok(Output::new().to_jsarray())
    } else {
        Err(CommandErr::new(
            "cd",
            format!("no such file of directory: {}", to_dir),
        ))
    }
}
pub fn hello_konsh(_parser: &Parser) -> CommandOut {
    let outputs = vec![
        "Welcome to kons's portfolio:)",
        "Application made by kons are displayed in this site.",
        "ゆっくりしていってね!!!!",
        "",
        "(The list of command can be found by typing 'help command')",
    ];
    let colors = vec![""; outputs.len()];
    Ok(Output::from_vec(outputs, colors).to_jsarray())
}
pub fn ls(_parser: &Parser, _ft: &FileTree) -> CommandOut {
    Ok(Output::new().to_jsarray())
}
pub fn help(_parser: &Parser) -> CommandOut {
    let outputs = vec!["hello", "world"];
    let colors = vec!["red", ""];
    let output = Output::from_vec(outputs, colors);
    Ok(output.to_jsarray())
}
