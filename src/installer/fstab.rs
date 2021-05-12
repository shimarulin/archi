use std::process::Command;
use crate::utils::file;

pub fn generate() {
    let output = Command::new("genfstab")
        .args(&["-U", "-p", "/mnt"])
        .output()
        .expect("failed to execute genfstab");

    file::create("/mnt/etc/hostname", &*String::from_utf8(output.stdout).unwrap());
}
