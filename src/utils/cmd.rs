use std::process::{Command, Output, Stdio};

pub fn exec(command: &str, args: &[&str]) -> Output {
    Command::new(command)
        .args(args)
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .output()
        .expect("ERR")
}

pub fn run(command: &str) -> Output {
    Command::new(command)
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .output()
        .expect("ERR")
}
