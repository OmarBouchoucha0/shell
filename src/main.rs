use std::io::{self, Write};

fn builtin_echo() {
    println!("echo is a builtin");
}

fn handle_command(cmd: &str) {
    match cmd {
        "echo" => builtin_echo(),
        _ => println!("{cmd}: not found"),
    }
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
            handle_command(trimmed);
        }
    }
}

fn main() {
    run();
}

