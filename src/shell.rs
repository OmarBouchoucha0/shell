use crate::cmd::{Cmd, Execute};
use crate::history::ShellHistory;
use rustyline::{Config, Editor, error::ReadlineError};
use std::env;

pub struct Shell {
    history: ShellHistory,
}

impl Shell {
    pub fn new() -> Self {
        Shell {
            history: ShellHistory::new(),
        }
    }

    pub fn history(&self) -> &ShellHistory {
        &self.history
    }

    pub fn history_mut(&mut self) -> &mut ShellHistory {
        &mut self.history
    }

    pub fn run(&mut self) {
        let config = Config::default();
        let history = ShellHistory::new();
        let mut rl: Editor<(), ShellHistory> = Editor::with_history(config, history).unwrap();

        loop {
            let prompt = match env::current_dir() {
                Ok(dir) => format!("{}$ ", dir.display()),
                Err(_) => "$ ".to_string(),
            };

            match rl.readline(&prompt) {
                Ok(line) => {
                    let trimmed = line.trim();
                    if !trimmed.is_empty()
                        && let Err(e) = self.handle_command(trimmed) {
                            eprintln!("Error: {e}");
                        }
                }

                Err(ReadlineError::Interrupted) => continue,
                Err(ReadlineError::Eof) => break,
                Err(err) => {
                    eprintln!("Error: {:?}", err);
                    break;
                }
            }
        }
    }
    fn parse_args(&self, args: &str) -> Vec<String> {
        args.split_whitespace().map(|s| s.to_string()).collect()
    }

    fn parse_input<'a>(&self, input: &'a str) -> (&'a str, &'a str) {
        if let Some((cmd, args)) = input.split_once(" ") {
            (cmd, args)
        } else {
            let args = "";
            (input, args)
        }
    }

    fn handle_command(&mut self, input: &str) -> Result<(), String> {
        let (cmd_name, args) = self.parse_input(input);
        let args = self.parse_args(args);
        let cmd = Cmd::new(cmd_name);
        cmd.execute(&args, self)?;
        Ok(())
    }
}

impl Default for Shell {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_shell_new() {
        let shell = Shell::new();
        assert!(shell.history.is_empty());
    }

    #[test]
    fn test_parse_args_single() {
        let shell = Shell::new();
        let result = shell.parse_args("hello");
        assert_eq!(result, vec!["hello"]);
    }

    #[test]
    fn test_parse_args_multiple() {
        let shell = Shell::new();
        let result = shell.parse_args("hello world test");
        assert_eq!(result, vec!["hello", "world", "test"]);
    }

    #[test]
    fn test_parse_args_empty() {
        let shell = Shell::new();
        let result = shell.parse_args("");
        assert!(result.is_empty());
    }

    #[test]
    fn test_parse_input_with_args() {
        let shell = Shell::new();
        let (cmd, args) = shell.parse_input("echo hello world");
        assert_eq!(cmd, "echo");
        assert_eq!(args, "hello world");
    }

    #[test]
    fn test_parse_input_no_args() {
        let shell = Shell::new();
        let (cmd, args) = shell.parse_input("exit");
        assert_eq!(cmd, "exit");
        assert_eq!(args, "");
    }

    #[test]
    fn test_handle_command_builtin_echo() {
        let mut shell = Shell::new();
        let result = shell.handle_command("echo hello world");
        assert!(result.is_ok());
        assert_eq!(shell.history().len(), 1);
    }

    #[test]
    fn test_handle_command_unknown() {
        let mut shell = Shell::new();
        let result = shell.handle_command("nonexistentcommand123");
        assert!(result.is_err());
    }
}
