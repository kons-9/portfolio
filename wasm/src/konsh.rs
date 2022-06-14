use std::collections::HashMap;
const builtin_commands: [&str; 1] = ["cd"];

pub struct Parser {
    command: String,
    options: HashMap<String, String>,
}

impl Parser {
    pub fn new(command: &str) -> Self {}
}
