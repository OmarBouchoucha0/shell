use crate::command::Execute;
use std::process::Command;

pub struct NonBuiltinCommand {
    name: String,
}

impl NonBuiltinCommand {
    pub fn new(name: &str) -> Self {
        NonBuiltinCommand {
            name: name.to_string(),
        }
    }
}

impl Execute for NonBuiltinCommand {
    fn execute(&self, args: Vec<String>) -> Result<(), String> {
        match Command::new(&self.name).args(args).output() {
            Ok(output) => {
                println!("{}", String::from_utf8_lossy(&output.stdout));
                Ok(())
            }
            Err(_) => Err(format!("Unknown command: {}", self.name)),
        }
    }
}
