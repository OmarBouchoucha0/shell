use crate::builtin::{check_builtin_existance, BuiltinCommand};
use crate::external::NonBuiltinCommand;
use crate::shell::Shell;

pub trait Execute {
    fn execute(&self, args: &Vec<String>, shell: &mut Shell) -> Result<(), String>;
}

pub type CmdFn = Box<dyn Fn(&Vec<String>, &mut Shell) -> Result<(), String>>;

pub struct Cmd<'a> {
    name: &'a str,
}

impl<'a> Cmd<'a> {
    pub fn new(name: &'a str) -> Cmd<'a> {
        Cmd { name }
    }
}

impl<'a> Execute for Cmd<'a> {
    fn execute(&self, args: &Vec<String>, shell: &mut Shell) -> Result<(), String> {
        if check_builtin_existance(self.name) {
            let cmd = BuiltinCommand::new(self.name)?;
            cmd.execute(args, shell)?;
        } else {
            let cmd = NonBuiltinCommand::new(self.name)?;
            cmd.execute(args, shell)?;
        }
        Ok(())
    }
}
