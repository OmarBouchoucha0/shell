use crate::cmd::Execute;
use crate::history::History;
use std::process::Command;
use std::{env, path::Path};

pub struct NonBuiltinCommand {
    name: String,
}

impl NonBuiltinCommand {
    pub fn new(name: &str) -> Result<Self, String> {
        if external_command_exists(name) {
            Ok(NonBuiltinCommand {
                name: name.to_string(),
            })
        } else {
            Err(format!("{}: command not found", name))
        }
    }
}

impl Execute for NonBuiltinCommand {
    fn execute(&self, args: Vec<String>, history: &mut History) -> Result<(), String> {
        match Command::new(&self.name).args(args).output() {
            Ok(output) => {
                print!("{}", String::from_utf8_lossy(&output.stdout));
                history.push(self.name.clone());
                Ok(())
            }
            Err(_) => Err(format!("Unknown command: {}", self.name)),
        }
    }
}

pub fn external_command_exists(cmd: &str) -> bool {
    if cmd.contains('/') {
        return Path::new(cmd).exists();
    }

    if let Some(path_var) = env::var_os("PATH") {
        for dir in env::split_paths(&path_var) {
            let full = dir.join(cmd);
            if full.is_file() {
                return true;
            }
        }
    }

    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_non_builtin_command_new() {
        let cmd = NonBuiltinCommand::new("ls").unwrap();
        assert_eq!(cmd.name, "ls");
    }

    #[test]
    fn test_non_builtin_command_new_invalid() {
        let result = NonBuiltinCommand::new("nonexistentcommand123456");
        assert!(result.is_err());
    }

    #[test]
    fn test_non_builtin_command_empty_name() {
        let result = NonBuiltinCommand::new("");
        assert!(result.is_err());
    }

    #[test]
    fn test_non_builtin_command_execute_valid() {
        let cmd = NonBuiltinCommand::new("echo").unwrap();
        let mut history = History::new();
        let result = cmd.execute(vec!["hello".to_string()], &mut history);
        assert!(result.is_ok());
        assert_eq!(history.len(), 1);
    }

    #[test]
    fn test_non_builtin_command_execute_with_args() {
        let cmd = NonBuiltinCommand::new("ls").unwrap();
        let mut history = History::new();
        let result = cmd.execute(vec!["-la".to_string()], &mut history);
        assert!(result.is_ok());
        assert_eq!(history.len(), 1);
    }

    #[test]
    fn test_non_builtin_command_special_chars_in_name() {
        let result = NonBuiltinCommand::new("cmd-with_dots");
        // This will fail since command doesn't exist
        assert!(result.is_err());
    }
}
