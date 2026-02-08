use crate::builtin::{check_builtin_existance, BuiltinCommand};
use crate::external::NonBuiltinCommand;
use crate::shell::Shell;

pub trait Execute {
    fn execute(&self, args: Vec<String>, shell: &mut Shell) -> Result<(), String>;
}

pub type CmdFn = Box<dyn Fn(Vec<String>, &mut Shell) -> Result<(), String>>;

pub struct Cmd {
    name: String,
}

impl Cmd {
    pub fn new(name: &str) -> Cmd {
        Cmd {
            name: name.to_string(),
        }
    }
}

impl Execute for Cmd {
    fn execute(&self, args: Vec<String>, shell: &mut Shell) -> Result<(), String> {
        if check_builtin_existance(&self.name) {
            let cmd = BuiltinCommand::new(&self.name)?;
            cmd.execute(args, shell)?;
        } else {
            let cmd = NonBuiltinCommand::new(&self.name)?;
            cmd.execute(args, shell)?;
        }
        Ok(())
    }
}
