use crate::cmd::{CmdFn, Execute};
use crate::external::external_command_exists;
use crate::shell::Shell;
use std::collections::HashMap;
use std::env;
use std::path::Path;

pub struct BuiltinCommand<'a> {
    name: &'a str,
}

impl<'a> BuiltinCommand<'a> {
    pub fn new(name: &'a str) -> Result<Self, String> {
        if check_builtin_existance(name) {
            Ok(BuiltinCommand { name })
        } else {
            Err(format!("{}: not a builtin command", name))
        }
    }
}

fn echo(args: &[String], _shell: &mut Shell) -> Result<(), String> {
    let output = args.join(" ");
    println!("{output}");
    Ok(())
}

fn exit(_args: &[String], _shell: &mut Shell) -> Result<(), String> {
    std::process::exit(0);
}

fn pwd(_args: &[String], _shell: &mut Shell) -> Result<(), String> {
    let current_dir = env::current_dir().map_err(|e| e.to_string())?;
    println!("The current directory is: {}", current_dir.display());
    Ok(())
}

fn cd(args: &[String], _shell: &mut Shell) -> Result<(), String> {
    if args.is_empty() {
        match env::home_dir() {
            Some(path) => {
                env::set_current_dir(path).map_err(|e| e.to_string())?;
                return Ok(());
            }
            _ => return Err("Home Directory not Found".to_string()),
        }
    }
    if args.len() > 1 {
        return Err("CD only 1 takes 1 Path".to_string());
    }
    let path_str = args.join("");
    let path = Path::new(&path_str);
    env::set_current_dir(path).map_err(|e| e.to_string())?;
    Ok(())
}

fn history_cmd(_args: &[String], shell: &mut Shell) -> Result<(), String> {
    let mut i: u32 = 1;
    for line in shell.history().iter() {
        println!("{i} {line}");
        i += 1;
    }
    Ok(())
}

fn type_cmd(args: &[String], _shell: &mut Shell) -> Result<(), String> {
    let dispatch_table = build_dispatch_table();
    if let Some(arg) = args.first() {
        if arg.chars().all(char::is_whitespace) {
            println!();
            return Ok(());
        }
        if dispatch_table.contains_key(arg) {
            println!("{arg} : BUILTIN");
            return Ok(());
        }
        if external_command_exists(arg) {
            println!("{arg} : EXTERNAL");
            return Ok(());
        } else {
            println!("{arg} : UNKNOW COMMAND");
            return Ok(());
        }
    }
    Ok(())
}

pub fn build_dispatch_table() -> HashMap<String, CmdFn> {
    let mut map: HashMap<String, CmdFn> = HashMap::new();

    map.insert("echo".to_string(), Box::new(echo));
    map.insert("exit".to_string(), Box::new(exit));
    map.insert("pwd".to_string(), Box::new(pwd));
    map.insert("cd".to_string(), Box::new(cd));
    map.insert("history".to_string(), Box::new(history_cmd));
    map.insert("type".to_string(), Box::new(type_cmd));

    map
}

pub fn check_builtin_existance(name: &str) -> bool {
    let builtins: Vec<&str> = vec!["echo", "exit", "pwd", "cd", "history", "type"];
    builtins.contains(&name)
}

impl<'a> Execute for BuiltinCommand<'a> {
    fn execute(&self, args: &[String], shell: &mut Shell) -> Result<(), String> {
        let dispatch_table = build_dispatch_table();
        if let Some(func) = dispatch_table.get(self.name) {
            let result = func(args, shell);
            if result.is_ok() {
                shell
                    .history_mut()
                    .push(format!("{} {}", self.name, args.join(" ")));
            }
            result
        } else {
            Err(format!("Erreur Executing Command: {}", self.name))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_builtin_command_new() {
        let cmd = BuiltinCommand::new("echo").unwrap();
        assert_eq!(cmd.name, "echo");
    }

    #[test]
    fn test_builtin_command_new_invalid() {
        let result = BuiltinCommand::new("invalid");
        assert!(result.is_err());
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
        let cmd = BuiltinCommand::new("echo").unwrap();
        let mut shell = Shell::new();
        let args = vec!["hello".to_string(), "world".to_string()];
        let result = cmd.execute(&args, &mut shell);
        assert!(result.is_ok());
        assert_eq!(shell.history().len(), 1);
    }

    #[test]
    fn test_builtin_command_execute_type_builtin() {
        let cmd = BuiltinCommand::new("type").unwrap();
        let mut shell = Shell::new();
        let args = vec!["echo".to_string()];
        let result = cmd.execute(&args, &mut shell);
        assert!(result.is_ok());
        assert_eq!(shell.history().len(), 1);
    }

    #[test]
    fn test_builtin_command_execute_type_external() {
        let cmd = BuiltinCommand::new("type").unwrap();
        let mut shell = Shell::new();
        let args = vec!["ls".to_string()];
        let result = cmd.execute(&args, &mut shell);
        assert!(result.is_ok());
        assert_eq!(shell.history().len(), 1);
    }

    #[test]
    fn test_builtin_command_execute_unknown() {
        let _cmd = BuiltinCommand::new("echo").unwrap();
        let mut shell = Shell::new();
        // Test with wrong command name (won't happen in practice since new() validates)
        let cmd_unknown = BuiltinCommand { name: "unknown" };
        let args: Vec<String> = vec![];
        let result = cmd_unknown.execute(&args, &mut shell);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Erreur Executing Command"));
        assert!(shell.history().is_empty());
    }

    #[test]
    fn test_builtin_command_empty_name() {
        let result = BuiltinCommand::new("");
        assert!(result.is_err());
    }

    #[test]
    fn test_echo_empty_args() {
        let mut shell = Shell::new();
        let args: Vec<String> = vec![];
        let result = echo(&args, &mut shell);
        assert!(result.is_ok());
    }

    #[test]
    fn test_echo_single_arg() {
        let mut shell = Shell::new();
        let args = vec!["hello".to_string()];
        let result = echo(&args, &mut shell);
        assert!(result.is_ok());
    }

    #[test]
    fn test_echo_multiple_args() {
        let mut shell = Shell::new();
        let args = vec!["hello".to_string(), "world".to_string(), "test".to_string()];
        let result = echo(&args, &mut shell);
        assert!(result.is_ok());
    }

    #[test]
    fn test_echo_with_spaces() {
        let mut shell = Shell::new();
        let args = vec!["hello world".to_string()];
        let result = echo(&args, &mut shell);
        assert!(result.is_ok());
    }

    #[test]
    fn test_type_empty_args() {
        let mut shell = Shell::new();
        let args: Vec<String> = vec![];
        let result = type_cmd(&args, &mut shell);
        assert!(result.is_ok());
    }

    #[test]
    fn test_type_whitespace_arg() {
        let mut shell = Shell::new();
        let args = vec!["   ".to_string()];
        let result = type_cmd(&args, &mut shell);
        assert!(result.is_ok());
    }

    #[test]
    fn test_type_multiple_args_mixed() {
        let mut shell = Shell::new();
        let args = vec!["echo".to_string(), "ls".to_string(), "exit".to_string()];
        let result = type_cmd(&args, &mut shell);
        assert!(result.is_ok());
    }

    #[test]
    fn test_pwd() {
        let mut shell = Shell::new();
        let args: Vec<String> = vec![];
        let result = pwd(&args, &mut shell);
        assert!(result.is_ok());
    }

    #[test]
    fn test_cd() {
        let mut shell = Shell::new();
        let args = vec![".".to_string()];
        let result = cd(&args, &mut shell);
        assert!(result.is_ok());
    }

    #[test]
    fn test_cd_no_args() {
        let mut shell = Shell::new();
        let args: Vec<String> = vec![];
        let result = cd(&args, &mut shell);
        assert!(result.is_ok());
    }

    #[test]
    fn test_history_command() {
        let mut shell = Shell::new();
        let args: Vec<String> = vec![];
        let result = history_cmd(&args, &mut shell);
        assert!(result.is_ok());
    }
}
