use std::process::{Command, Stdio};

pub fn install() {
    println!("Install packages");
    Command::new("pacstrap")
        .arg("/mnt")
        .args(&[
            "base",
            "btrfs-progs",
            "grub",
            "efibootmgr",
            "dosfstools",
            "linux",
            "linux-firmware",
            "nano",
            "networkmanager",
        ])
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .output()
        .expect("failed to execute pacstrap");
}
