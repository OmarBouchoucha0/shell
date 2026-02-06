pub trait Execute {
    fn execute(&self, args: Vec<String>, history: &mut Vec<String>) -> Result<(), String>;
}

pub type CommandFn = Box<dyn Fn(Vec<String>, &Vec<String>) -> Result<(), String>>;
