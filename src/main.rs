use std::io::{self, Write};

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

fn parse_command(cmd: &str) -> Option<(&str, &str)> {
    if cmd.contains(" ") {
        let (cmd, args) = cmd.split_once(" ")?;
        return Some((cmd, args));
    }
    let args = "";
    Some((cmd, args))
}

fn handle_command(cmd: &str) -> Result<(), &str> {
    if let Some((cmd, args)) = parse_command(cmd) {
        match cmd {
            "echo" => builtin_echo(args),
            "exit" => builtin_exit(),
            "type" => builtin_type(args),
            _ => return Err("Command: not found"),
        }
    } else {
        return Err("Command: not found");
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
