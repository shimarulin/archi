use std::process::{Command};

pub fn enable_swap(disk_path: &str) {
    let part_number = "4";
    let part_path = format!("{}{}", disk_path, part_number);
    Command::new("swapon")
        .arg(&part_path)
        .output()
        .expect("failed to execute swapon");

    println!("swap on {} enabled", part_path);
}

pub fn mount_root(disk_path: &str) {
    let part_number = "3";
    let part_path = format!("{}{}", disk_path, part_number);
    Command::new("mount")
        .arg(&part_path)
        .arg("/mnt")
        .output()
        .expect("failed to execute mkfs.btrfs");

    println!("btrfs filesystem on {} is mounted to /mnt", part_path);
}

pub fn create_subvolume(name: &str) {
    let subvolume_path: &str = &*format!("{}{}", "/mnt/", name);
    Command::new("btrfs")
        .args(&["subvolume", "create", subvolume_path])
        .output()
        .expect("failed to execute btrfs");

    println!("btrfs subvolume {} created", subvolume_path);
}