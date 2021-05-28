use crate::utils::file;
use std::process::Command;

pub fn generate() {
    let output = Command::new("genfstab")
        .args(&["-U", "-p", "/mnt"])
        .output()
        .expect("failed to execute genfstab");

    file::create(
        "/mnt/etc/fstab",
        &*String::from_utf8(output.stdout).unwrap(),
    );
}
