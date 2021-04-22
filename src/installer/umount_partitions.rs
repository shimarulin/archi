use std::process::{Command};

pub fn umount_root() {
    Command::new("umount")
        .arg("/mnt")
        .output()
        .expect("failed to execute umount");

    println!("filesystem on /mnt is unmounted");
}
