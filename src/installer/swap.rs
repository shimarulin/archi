use std::process::{Command};

pub fn init(disk_partition_path: &str) {
    Command::new("mkswap")
        .arg(&disk_partition_path)
        .output()
        .expect("failed to execute mkswap");

    println!("swap on {} initialized", disk_partition_path);
}

pub fn on(disk_partition_path: &str) {
    Command::new("swapon")
        .arg(&disk_partition_path)
        .output()
        .expect("failed to execute swapon");

    println!("swap on {} enabled", disk_partition_path);
}