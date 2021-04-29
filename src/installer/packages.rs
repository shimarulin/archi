use std::process::{Command, Stdio};

pub fn install() {
    println!("Install packages");
    Command::new("pacstrap")
        .arg("/mnt")
        .args(&["base", "btrfs-progs", "grub", "linux", "linux-firmware", "nano"])
        .stdout(Stdio::inherit())
        .output()
        .expect("failed to execute pacstrap");
}