mod builtin;
mod command;
mod external;

use builtin::{BuiltinCommand, build_dispatch_table};
use command::Execute;
use external::NonBuiltinCommand;
use std::env;
use std::io::{self, Write};

fn parse_args(args: &str) -> Vec<String> {
    let mut output: Vec<String> = Vec::new();
    for arg in args.split_whitespace() {
        output.push(arg.to_string());
    }
    output
}

fn parse_input(input: &str) -> Option<(&str, &str)> {
    if input.contains(" ") {
        let (cmd, args) = input.split_once(" ")?;
        return Some((cmd, args));
    }
    let args = "";
    Some((input, args))
}

fn handle_command(input: &str, history: &mut Vec<String>) -> Result<(), String> {
    if let Some((cmd, args)) = parse_input(input) {
        let dispatch_table = build_dispatch_table();
        let args = parse_args(args);
        if dispatch_table.contains_key(cmd) {
            let cmd = BuiltinCommand::new(cmd);
            cmd.execute(args, history)?;
        } else {
            let cmd = NonBuiltinCommand::new(cmd);
            cmd.execute(args, history)?;
        }
    } else {
        return Err(format!("Command {}: not found", input));
    }
    Ok(())
}

fn run() {
    let mut cmd = String::new();
    let mut history: Vec<String> = Vec::new();
    loop {
        cmd.clear();
        match env::current_dir() {
            Ok(dir) => {
                print!("{}$ ", dir.display());
            }
            Err(e) => eprintln!("Error: {e}"),
        }

        if let Err(e) = io::stdout().flush() {
            eprintln!("Error: {e}");
            continue;
        }

        if let Err(e) = io::stdin().read_line(&mut cmd) {
            eprintln!("Error: {e}");
            continue;
        }

        let trimmed = cmd.trim();
        if !trimmed.is_empty()
            && let Err(e) = handle_command(trimmed, &mut history)
        {
            eprintln!("Error : {e}");
            continue;
        }
    }
}

fn main() {
    run();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_args_single() {
        let result = parse_args("hello");
        assert_eq!(result, vec!["hello"]);
    }

    #[test]
    fn test_parse_args_multiple() {
        let result = parse_args("hello world test");
        assert_eq!(result, vec!["hello", "world", "test"]);
    }

    #[test]
    fn test_parse_args_empty() {
        let result = parse_args("");
        assert!(result.is_empty());
    }

    #[test]
    fn test_parse_args_whitespace_only() {
        let result = parse_args("   ");
        assert!(result.is_empty());
    }

    #[test]
    fn test_parse_args_multiple_spaces() {
        let result = parse_args("hello    world");
        assert_eq!(result, vec!["hello", "world"]);
    }

    #[test]
    fn test_parse_args_with_tabs() {
        let result = parse_args("hello\tworld");
        assert_eq!(result, vec!["hello", "world"]);
    }

    #[test]
    fn test_parse_args_leading_trailing_whitespace() {
        let result = parse_args("  hello world  ");
        assert_eq!(result, vec!["hello", "world"]);
    }

    #[test]
    fn test_parse_input_with_args() {
        let result = parse_input("echo hello world");
        assert!(result.is_some());
        let (cmd, args) = result.unwrap();
        assert_eq!(cmd, "echo");
        assert_eq!(args, "hello world");
    }

    #[test]
    fn test_parse_input_no_args() {
        let result = parse_input("exit");
        assert!(result.is_some());
        let (cmd, args) = result.unwrap();
        assert_eq!(cmd, "exit");
        assert_eq!(args, "");
    }

    #[test]
    fn test_parse_input_empty() {
        let result = parse_input("");
        assert!(result.is_some());
        let (cmd, args) = result.unwrap();
        assert_eq!(cmd, "");
        assert_eq!(args, "");
    }

    #[test]
    fn test_parse_input_single_space() {
        let result = parse_input(" ");
        assert!(result.is_some());
        let (cmd, args) = result.unwrap();
        assert_eq!(cmd, "");
        assert_eq!(args, "");
    }

    #[test]
    fn test_parse_input_multiple_spaces_between() {
        let result = parse_input("echo   hello");
        assert!(result.is_some());
        let (cmd, args) = result.unwrap();
        assert_eq!(cmd, "echo");
        assert_eq!(args, "  hello");
    }

    #[test]
    fn test_parse_input_leading_whitespace() {
        let result = parse_input("  echo hello");
        assert!(result.is_some());
        let (cmd, args) = result.unwrap();
        assert_eq!(cmd, "");
        assert_eq!(args, " echo hello");
    }

    #[test]
    fn test_handle_command_builtin_echo() {
        let mut history = Vec::new();
        let result = handle_command("echo hello world", &mut history);
        assert!(result.is_ok());
        assert_eq!(history.len(), 1);
        assert_eq!(history[0], "echo");
    }

    #[test]
    fn test_handle_command_builtin_type() {
        let mut history = Vec::new();
        let result = handle_command("type echo", &mut history);
        assert!(result.is_ok());
        assert_eq!(history.len(), 1);
        assert_eq!(history[0], "type");
    }

    #[test]
    fn test_handle_command_external() {
        let mut history = Vec::new();
        let result = handle_command("echo test", &mut history);
        assert!(result.is_ok());
        assert_eq!(history.len(), 1);
        assert_eq!(history[0], "echo");
    }

    #[test]
    fn test_handle_command_unknown_external() {
        let mut history = Vec::new();
        let result = handle_command("nonexistentcommand123", &mut history);
        assert!(result.is_err());
        assert!(history.is_empty());
    }

    #[test]
    fn test_handle_command_empty() {
        let mut history = Vec::new();
        let result = handle_command("", &mut history);
        assert!(result.is_err());
    }

    #[test]
    fn test_handle_command_only_whitespace() {
        let mut history = Vec::new();
        let result = handle_command("   ", &mut history);
        assert!(result.is_err());
    }

    #[test]
    fn test_handle_command_builtin_pwd() {
        let mut history = Vec::new();
        let result = handle_command("pwd", &mut history);
        assert!(result.is_ok());
        assert_eq!(history.len(), 1);
        assert_eq!(history[0], "pwd");
    }

    #[test]
    fn test_handle_command_builtin_history() {
        let mut history = Vec::new();
        let result = handle_command("echo hello", &mut history);
        assert!(result.is_ok());
        let result = handle_command("history", &mut history);
        assert!(result.is_ok());
        assert_eq!(history.len(), 2);
        assert_eq!(history[0], "echo");
        assert_eq!(history[1], "history");
    }

    #[test]
    fn test_build_dispatch_table_contents() {
        let table = build_dispatch_table();
        assert_eq!(table.len(), 6);
        assert!(table.contains_key("echo"));
        assert!(table.contains_key("exit"));
        assert!(table.contains_key("type"));
        assert!(table.contains_key("pwd"));
        assert!(table.contains_key("cd"));
        assert!(table.contains_key("history"));
    }

    #[test]
    fn test_execute_trait_builtin() {
        let cmd = BuiltinCommand::new("echo");
        let mut history = Vec::new();
        let result = cmd.execute(vec!["test".to_string()], &mut history);
        assert!(result.is_ok());
        assert_eq!(history.len(), 1);
        assert_eq!(history[0], "echo");
    }

    #[test]
    fn test_execute_trait_external() {
        let cmd = NonBuiltinCommand::new("echo");
        let mut history = Vec::new();
        let result = cmd.execute(vec!["test".to_string()], &mut history);
        assert!(result.is_ok());
        assert_eq!(history.len(), 1);
        assert_eq!(history[0], "echo");
    }
}
