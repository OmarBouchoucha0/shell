use crate::history::History;

pub trait Execute {
    fn execute(&self, args: Vec<String>, history: &mut History) -> Result<(), String>;
}

pub type CommandFn = Box<dyn Fn(Vec<String>, &History) -> Result<(), String>>;
