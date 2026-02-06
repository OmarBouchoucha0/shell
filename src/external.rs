use crate::command::Execute;
use std::process::Command;
use std::{env, path::Path};

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
    fn execute(&self, args: Vec<String>, history: &mut Vec<String>) -> Result<(), String> {
        match Command::new(&self.name).args(args).output() {
            Ok(output) => {
                history.push(self.name.clone());
                print!("{}", String::from_utf8_lossy(&output.stdout));
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
        let cmd = NonBuiltinCommand::new("ls");
        assert_eq!(cmd.name, "ls");
    }

    #[test]
    fn test_non_builtin_command_empty_name() {
        let cmd = NonBuiltinCommand::new("");
        assert_eq!(cmd.name, "");
    }

    #[test]
    fn test_non_builtin_command_execute_valid() {
        let cmd = NonBuiltinCommand::new("echo");
        let result = cmd.execute(vec!["hello".to_string()]);
        assert!(result.is_ok());
    }

    #[test]
    fn test_non_builtin_command_execute_unknown() {
        let cmd = NonBuiltinCommand::new("nonexistent_command_xyz");
        let result = cmd.execute(vec![]);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Unknown command"));
    }

    #[test]
    fn test_non_builtin_command_execute_with_args() {
        let cmd = NonBuiltinCommand::new("ls");
        let result = cmd.execute(vec!["-la".to_string()]);
        assert!(result.is_ok());
    }

    #[test]
    fn test_non_builtin_command_special_chars_in_name() {
        let cmd = NonBuiltinCommand::new("cmd-with_dots");
        assert_eq!(cmd.name, "cmd-with_dots");
    }
}
