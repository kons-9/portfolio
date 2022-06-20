use std::collections::HashMap;

pub struct Parser<'a> {
    pub command: &'a str,
    pub sub_command: Vec<&'a str>,
    pub options: HashMap<&'a str, Option<&'a str>>,
}

impl<'a> Parser<'a> {
    pub fn new(command_str: &'a str) -> Self {
        let mut iter = command_str.split_ascii_whitespace();
        let command;
        let mut options = HashMap::<&str, Option<&str>>::new();
        let mut sub_command = Vec::<&str>::new();
        let mut pre_option: Option<&str> = None;
        if let Some(name) = iter.next() {
            command = name;
        } else {
            panic!("invalid command");
        }
        for val in iter {
            if val.starts_with("--") {
                // long options
                let val = val.strip_prefix("--").unwrap();
                if let Some(s) = pre_option {
                    options.insert(s, None);
                }
                pre_option = Some(val);
            } else if val.starts_with("-") {
                // short options
                let val = val.strip_prefix("-").unwrap();
                if let Some(s) = pre_option {
                    options.insert(s, None);
                }
                pre_option = Some(val);
            } else {
                if let Some(s) = pre_option {
                    options.insert(s, Some(val));
                } else {
                    sub_command.push(val);
                }
                pre_option = None;
            }
        }
        if let Some(s) = pre_option {
            options.insert(s, None);
        }
        Parser {
            command,
            options,
            sub_command,
        }
    }
}
