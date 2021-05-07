use std::process::{Command, Stdio};

pub fn exec(command: &str, args: &[&str]) {
    Command::new(command)
        .args(args)
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .output()
        .expect("ERR");
}
