use std::io::{self, Write};

fn builtin_echo() {
    println!("echo is a builtin");
}

fn builtin_exit() {
    println!("Exiting the Program");
    std::process::exit(0);
}

fn handle_command(cmd: &str) {
    match cmd {
        "echo" => builtin_echo(),
        "exit" => builtin_exit(),
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
