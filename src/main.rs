mod builtin;
mod cmd;
mod external;
mod history;
mod shell;

use shell::Shell;

fn main() {
    let mut shell = Shell::new();
    shell.run();
}
