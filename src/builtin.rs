use crate::command::{CommandFn, Execute};
use std::collections::HashMap;

pub struct BuiltinCommand {
    name: String,
}

impl BuiltinCommand {
    pub fn new(name: &str) -> Self {
        BuiltinCommand {
            name: name.to_string(),
        }
    }
}

pub fn build_dispatch_table() -> HashMap<String, CommandFn> {
    let mut map: HashMap<String, CommandFn> = HashMap::new();

    map.insert("echo".to_string(), Box::new(echo));
    map.insert("exit".to_string(), Box::new(exit));
    map.insert("type".to_string(), Box::new(type_cmd));

    map
}

impl Execute for BuiltinCommand {
    fn execute(&self, args: Vec<String>) -> Result<(), String> {
        let dispatch_table = build_dispatch_table();
        if dispatch_table.contains_key(self.name.as_str()) {
            if let Some(func) = dispatch_table.get(self.name.as_str()) {
                func(args)
            } else {
                Err(format!("Unknown builtin command: {}", self.name))
            }
        } else {
            Err(format!("Unknown builtin command: {}", self.name))
        }
    }
}

fn echo(args: Vec<String>) -> Result<(), String> {
    println!("{:?}", args);
    Ok(())
}

fn exit(_args: Vec<String>) -> Result<(), String> {
    println!("Exiting the Program");
    std::process::exit(0);
}

fn type_cmd(args: Vec<String>) -> Result<(), String> {
    let dispatch_table = build_dispatch_table();
    for arg in args {
        if arg.chars().all(char::is_whitespace) {
            println!("");
        }
        if dispatch_table.contains_key(&arg) {
            println!("{arg} : BUILTIN");
        } else {
            println!("{arg} : EXTERNAL OR UNKNOW COMMAND");
        }
    }
    Ok(())
}
