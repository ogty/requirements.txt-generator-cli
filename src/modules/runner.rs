use std::process::{Command, Output};


pub trait Runner {
    fn run(&mut self) -> Output;
}

pub struct CommandRunner {
    pub command: &'static str,
    pub argument: &'static str,
}


impl Default for CommandRunner {
    fn default() -> CommandRunner {
        CommandRunner {
            command: "",
            argument: "",
        }
    }
}

impl Runner for CommandRunner {
    fn run(&mut self) -> Output {
        let output: Output = Command::new(self.command)
            .args(&[self.argument])
            .output()
            .expect("failed to execute process");

        return output
    }
}
