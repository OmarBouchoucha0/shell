pub trait Execute {
    fn execute(&self, args: Vec<String>) -> Result<(), String>;
}

pub type CommandFn = Box<dyn Fn(Vec<String>) -> Result<(), String>>;
