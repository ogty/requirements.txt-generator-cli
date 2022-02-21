use crate::modules::runner;
use crate::modules::runner::Runner;


pub fn whoami() -> String {
    let mut runner: runner::CommandRunner = runner::CommandRunner::default();
    runner.command = "whoami";
    let output = runner.run();
    
    if cfg!(windows) {
        let info: String = String::from_utf8(output.stdout).unwrap();
        let username: &str = info.split("\\").collect::<Vec<&str>>()[1];
        return String::from(username.replace("\r\n", ""));
    } else if cfg!(linux) {
        let username: String = String::from_utf8(output.stdout).unwrap();
        return username.replace("\r\n", "");
    } else if cfg!(macos) {
        let username: String = String::from_utf8(output.stdout).unwrap();
        return username.replace("\r\n", "");
    } else {
        panic!("Error");
    }
}
