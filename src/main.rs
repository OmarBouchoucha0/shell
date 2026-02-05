use std::collections::HashMap;
use std::io::{self, Write};
use std::process::Command;

trait Execute {
    fn execute(&self, args: Vec<String>) -> Result<(), String>;
}

type CommandFn = Box<dyn Fn(Vec<String>) -> Result<(), String>>;

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

struct NonBuiltinCommand {
    name: String,
}

impl NonBuiltinCommand {
    fn new(name: &str) -> Self {
        NonBuiltinCommand {
            name: name.to_string(),
        }
    }
}

impl Execute for NonBuiltinCommand {
    fn execute(&self, args: Vec<String>) -> Result<(), String> {
        match Command::new(&self.name).args(args).output() {
            Ok(output) => {
                println!("{}", String::from_utf8_lossy(&output.stdout));
                Ok(())
            }
            Err(_) => Err(format!("Unknown command: {}", self.name)),
        }
    }
}

fn build_dispatch_table_builtin_command() -> HashMap<String, CommandFn> {
    let mut map: HashMap<String, CommandFn> = HashMap::new();

    map.insert("echo".to_string(), Box::new(builtin_echo));
    map.insert("exit".to_string(), Box::new(builtin_exit));
    map.insert("type".to_string(), Box::new(builtin_type));

    map
}

impl Execute for BuiltinCommand {
    fn execute(&self, args: Vec<String>) -> Result<(), String> {
        let dispatch_table = build_dispatch_table_builtin_command();
        if dispatch_table.contains_key(self.name.as_str()) {
            if let Some(func) = dispatch_table.get(self.name.as_str()) {
                func(args)
            } else {
                Err(format!("Unknown builtin command: {}", self.name))
            }
        } else {
            Err(format!("Unknown builtin command: {}", self.name))
        }
    }
}

fn builtin_echo(args: Vec<String>) -> Result<(), String> {
    println!("{:?}", args);
    Ok(())
}

fn builtin_exit(_args: Vec<String>) -> Result<(), String> {
    println!("Exiting the Program");
    std::process::exit(0);
}

fn builtin_type(args: Vec<String>) -> Result<(), String> {
    let dispatch_table = build_dispatch_table_builtin_command();
    for arg in args {
        if arg.chars().all(char::is_whitespace) {
            println!("");
        }
        if dispatch_table.contains_key(&arg) {
            println!("{arg} : BUILTIN");
        } else {
            println!("{arg} : EXTERNAL OR UNKNOW COMMAND");
        }
    }
    Ok(())
}

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
        let dispatch_table = build_dispatch_table_builtin_command();
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
