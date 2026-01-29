use std::io::{self, Write};

trait Execute {
    fn execute(&self, args: &str) -> Result<(), String>;
}

struct BuiltinCommand {
    name: String,
}

impl BuiltinCommand {
    fn new(name: &str) -> Self {
        BuiltinCommand {
            name: name.to_string(),
        }
    }
}

impl Execute for BuiltinCommand {
    fn execute(&self, args: &str) -> Result<(), String> {
        match self.name.as_str() {
            "echo" => Ok(builtin_echo(args)),
            "exit" => Ok(builtin_exit()),
            "type" => Ok(builtin_type(args)),
            _ => Err(format!("Unknown builtin command: {}", self.name)),
        }
    }
}

fn builtin_echo(args: &str) {
    println!("{args}");
}

fn builtin_exit() {
    println!("Exiting the Program");
    std::process::exit(0);
}

fn builtin_type(args: &str) {
    println!("ARGS : {args}");
}

fn parse_input(input: &str) -> Option<(&str, &str)> {
    if input.contains(" ") {
        let (cmd, args) = input.split_once(" ")?;
        return Some((cmd, args));
    }
    let args = "";
    Some((input, args))
}

fn handle_command(input: &str) -> Result<(), String> {
    if let Some((cmd, args)) = parse_input(input) {
        let cmd = BuiltinCommand::new(cmd);
        cmd.execute(args)?;
    } else {
        return Err(format!("Command {}: not found", input));
    }
    Ok(())
}

fn run() {
    let mut cmd = String::new();
    loop {
        cmd.clear();
        print!("$ ");

        if let Err(e) = io::stdout().flush() {
            eprintln!("Error: {e}");
            continue;
        }

        if let Err(e) = io::stdin().read_line(&mut cmd) {
            eprintln!("Error: {e}");
            continue;
        }

        let trimmed = cmd.trim();
        if !trimmed.is_empty() {
            if let Err(e) = handle_command(trimmed) {
                eprintln!("Error : {e}");
                continue;
            }
        }
    }
}

fn main() {
    run();
}
