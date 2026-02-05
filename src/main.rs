mod builtin;
mod command;
mod external;

use builtin::{build_dispatch_table, BuiltinCommand};
use command::Execute;
use external::NonBuiltinCommand;
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

fn handle_command(input: &str) -> Result<(), String> {
    if let Some((cmd, args)) = parse_input(input) {
        let dispatch_table = build_dispatch_table();
        let args = parse_args(args);
        if dispatch_table.contains_key(cmd) {
            let cmd = BuiltinCommand::new(cmd);
            cmd.execute(args)?;
        } else {
            let cmd = NonBuiltinCommand::new(cmd);
            cmd.execute(args)?;
        }
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
        if !trimmed.is_empty()
            && let Err(e) = handle_command(trimmed)
        {
            eprintln!("Error : {e}");
            continue;
        }
    }
}

fn main() {
    run();
}
