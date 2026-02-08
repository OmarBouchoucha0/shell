use crate::builtin::{BuiltinCommand, check_builtin_existance};
use crate::external::NonBuiltinCommand;
use crate::history::History;

pub trait Execute {
    fn execute(&self, args: Vec<String>, history: &mut History) -> Result<(), String>;
}

pub type CmdFn = Box<dyn Fn(Vec<String>, &History) -> Result<(), String>>;

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
    fn execute(&self, args: Vec<String>, history: &mut History) -> Result<(), String> {
        if check_builtin_existance(&self.name) {
            let cmd = BuiltinCommand::new(&self.name)?;
            cmd.execute(args, history)?;
        } else {
            let cmd = NonBuiltinCommand::new(&self.name)?;
            cmd.execute(args, history)?;
        }
        Ok(())
    }
}
