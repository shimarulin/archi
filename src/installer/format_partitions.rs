use std::process::{Command};

pub fn swap_init(disk_path: &str) {
    let part_number = "4";
    let part_path = format!("{}{}", disk_path, part_number);
    Command::new("mkswap")
        .arg(&part_path)
        .output()
        .expect("failed to execute mkswap");

    println!("swap on {} initialized", part_path);
}

pub fn system_format(disk_path: &str) {
    let part_number = "3";
    let part_path = format!("{}{}", disk_path, part_number);
    let output = Command::new("mkfs.btrfs")
        .args(&["--label", "System"])
        .arg("--force")
        .arg(&part_path)
        .output()
        .expect("failed to execute mkfs.btrfs");

    let mut stdout = String::from_utf8(output.stdout).unwrap();
    stdout.pop();

    println!("btrfs filesystem on {} is created:\n{}", part_path, stdout);
}