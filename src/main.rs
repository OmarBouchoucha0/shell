use std::collections::HashMap;
use std::io::{self, Write};

trait Execute {
    fn execute(&self, args: &str) -> Result<(), String>;
}

type CommandFn = Box<dyn Fn(&str) -> Result<(), String>>;

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

fn build_dispatch_table() -> HashMap<&'static str, CommandFn> {
    let mut map: HashMap<&'static str, CommandFn> = HashMap::new();

    map.insert("echo", Box::new(|args| builtin_echo(args)));
    map.insert("exit", Box::new(|args| builtin_exit(args)));
    map.insert("type", Box::new(|args| builtin_type(args)));

    map
}

impl Execute for BuiltinCommand {
    fn execute(&self, args: &str) -> Result<(), String> {
        let dispatch_table = build_dispatch_table();
        if dispatch_table.contains_key(self.name.as_str()) {
            if let Some(func) = dispatch_table.get(self.name.as_str()) {
                return func(args);
            } else {
                return Err(format!("Unknown builtin command: {}", self.name));
            }
        } else {
            return Err(format!("Unknown builtin command: {}", self.name));
        }
    }
}

fn builtin_echo(args: &str) -> Result<(), String> {
    println!("{args}");
    Ok(())
}

//we use the args to match the func signature in cmdFn
fn builtin_exit(_args: &str) -> Result<(), String> {
    println!("Exiting the Program");
    std::process::exit(0);
}

fn builtin_type(args: &str) -> Result<(), String> {
    if args.chars().all(char::is_whitespace) {
        return Ok(());
    }
    let dispatch_table = build_dispatch_table();
    for arg in args.split(" ") {
        if dispatch_table.contains_key(arg) {
            println!("{arg} : BUILTIN");
        } else {
            println!("{arg} : EXTERNAL OR UNKNOW COMMAND");
        }
    }
    Ok(())
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
