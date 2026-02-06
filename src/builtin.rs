use crate::command::{CommandFn, Execute};
use crate::external::external_command_exists;
use std::collections::HashMap;
use std::env;
use std::path::Path;

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
    map.insert("pwd".to_string(), Box::new(pwd));
    map.insert("cd".to_string(), Box::new(cd));
    map.insert("history".to_string(), Box::new(history));
    map.insert("type".to_string(), Box::new(type_cmd));

    map
}

impl Execute for BuiltinCommand {
    fn execute(&self, args: Vec<String>, history: &mut Vec<String>) -> Result<(), String> {
        let dispatch_table = build_dispatch_table();
        if dispatch_table.contains_key(self.name.as_str()) {
            if let Some(func) = dispatch_table.get(self.name.as_str()) {
                let result = func(args, history);
                if result.is_ok() {
                    history.push(self.name.clone());
                }
                result
            } else {
                Err(format!("Erreur Executing Command: {}", self.name))
            }
        } else {
            Err(format!("Unknown builtin command: {}", self.name))
        }
    }
}

fn echo(args: Vec<String>, _history: &Vec<String>) -> Result<(), String> {
    let output = args.join(" ");
    println!("{output}");
    Ok(())
}

fn exit(_args: Vec<String>, _history: &Vec<String>) -> Result<(), String> {
    std::process::exit(0);
}

fn pwd(_args: Vec<String>, _history: &Vec<String>) -> Result<(), String> {
    let current_dir = env::current_dir().map_err(|e| e.to_string())?;
    println!("The current directory is: {}", current_dir.display());
    Ok(())
}

fn cd(args: Vec<String>, _history: &Vec<String>) -> Result<(), String> {
    if args.len() > 1 {
        return Err("CD only 1 takes 1 Path".to_string());
    }
    let path_str = args.join("");
    let path = Path::new(&path_str);
    env::set_current_dir(path).map_err(|e| e.to_string())?;
    Ok(())
}

fn history(_args: Vec<String>, history: &Vec<String>) -> Result<(), String> {
    let mut i: u32 = 1;
    for line in history {
        println!("{i} {line}");
        i += 1;
    }
    Ok(())
}

fn type_cmd(args: Vec<String>, _history: &Vec<String>) -> Result<(), String> {
    let dispatch_table = build_dispatch_table();
    if let Some(arg) = args.into_iter().next() {
        if arg.chars().all(char::is_whitespace) {
            println!();
            return Ok(());
        }
        if dispatch_table.contains_key(&arg) {
            println!("{arg} : BUILTIN");
            return Ok(());
        }
        if external_command_exists(&arg) {
            println!("{arg} : EXTERNAL");
            return Ok(());
        } else {
            println!("{arg} : UNKNOW COMMAND");
            return Ok(());
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_builtin_command_new() {
        let cmd = BuiltinCommand::new("echo");
        assert_eq!(cmd.name, "echo");
    }

    #[test]
    fn test_build_dispatch_table() {
        let table = build_dispatch_table();
        assert!(table.contains_key("echo"));
        assert!(table.contains_key("exit"));
        assert!(table.contains_key("pwd"));
        assert!(table.contains_key("cd"));
        assert!(table.contains_key("history"));
        assert!(table.contains_key("type"));
        assert_eq!(table.len(), 6);
    }

    #[test]
    fn test_builtin_command_execute_echo() {
        let cmd = BuiltinCommand::new("echo");
        let mut history = Vec::new();
        let result = cmd.execute(vec!["hello".to_string(), "world".to_string()], &mut history);
        assert!(result.is_ok());
        assert_eq!(history.len(), 1);
        assert_eq!(history[0], "echo");
    }

    #[test]
    fn test_builtin_command_execute_type_builtin() {
        let cmd = BuiltinCommand::new("type");
        let mut history = Vec::new();
        let result = cmd.execute(vec!["echo".to_string()], &mut history);
        assert!(result.is_ok());
        assert_eq!(history.len(), 1);
        assert_eq!(history[0], "type");
    }

    #[test]
    fn test_builtin_command_execute_type_external() {
        let cmd = BuiltinCommand::new("type");
        let mut history = Vec::new();
        let result = cmd.execute(vec!["ls".to_string()], &mut history);
        assert!(result.is_ok());
        assert_eq!(history.len(), 1);
        assert_eq!(history[0], "type");
    }

    #[test]
    fn test_builtin_command_execute_unknown() {
        let cmd = BuiltinCommand::new("unknown");
        let mut history = Vec::new();
        let result = cmd.execute(vec![], &mut history);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Unknown builtin command"));
        assert!(history.is_empty());
    }

    #[test]
    fn test_builtin_command_empty_name() {
        let cmd = BuiltinCommand::new("");
        assert_eq!(cmd.name, "");
    }

    #[test]
    fn test_echo_empty_args() {
        let history = Vec::new();
        let result = echo(vec![], &history);
        assert!(result.is_ok());
    }

    #[test]
    fn test_echo_single_arg() {
        let history = Vec::new();
        let result = echo(vec!["hello".to_string()], &history);
        assert!(result.is_ok());
    }

    #[test]
    fn test_echo_multiple_args() {
        let history = Vec::new();
        let result = echo(
            vec!["hello".to_string(), "world".to_string(), "test".to_string()],
            &history,
        );
        assert!(result.is_ok());
    }

    #[test]
    fn test_echo_with_spaces() {
        let history = Vec::new();
        let result = echo(vec!["hello world".to_string()], &history);
        assert!(result.is_ok());
    }

    #[test]
    fn test_type_empty_args() {
        let history = Vec::new();
        let result = type_cmd(vec![], &history);
        assert!(result.is_ok());
    }

    #[test]
    fn test_type_whitespace_arg() {
        let history = Vec::new();
        let result = type_cmd(vec!["   ".to_string()], &history);
        assert!(result.is_ok());
    }

    #[test]
    fn test_type_multiple_args_mixed() {
        let history = Vec::new();
        let result = type_cmd(
            vec!["echo".to_string(), "ls".to_string(), "exit".to_string()],
            &history,
        );
        assert!(result.is_ok());
    }

    #[test]
    fn test_pwd() {
        let history = Vec::new();
        let result = pwd(vec![], &history);
        assert!(result.is_ok());
    }

    #[test]
    fn test_cd() {
        let history = Vec::new();
        let result = cd(vec![".".to_string()], &history);
        assert!(result.is_ok());
    }

    #[test]
    fn test_cd_no_args() {
        let history = Vec::new();
        let result = cd(vec![], &history);
        assert!(result.is_err());
    }

    #[test]
    fn test_history() {
        let hist = vec!["echo".to_string(), "ls".to_string()];
        let result = history(vec![], &hist);
        assert!(result.is_ok());
    }

    #[test]
    fn test_history_empty() {
        let hist = Vec::new();
        let result = history(vec![], &hist);
        assert!(result.is_ok());
    }
}
