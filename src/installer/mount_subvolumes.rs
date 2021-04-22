use std::process::{Command};

pub fn mount_root_subvolume(disk_path: &str) {
    let part_number = "3";
    let part_path = format!("{}{}", disk_path, part_number);
    Command::new("mount")
        .args(&["-o", "subvol=@,compress=zstd"])
        .arg(&part_path)
        .arg("/mnt")
        .output()
        .expect("failed to execute mount");

    println!("btrfs subvolume @ on disk {} mounted to /mnt", part_path);
}

pub fn mount_home_subvolume(disk_path: &str) {
    let part_number = "3";
    let part_path = format!("{}{}", disk_path, part_number);
    Command::new("mkdir")
        .arg("/mnt/home")
        .output()
        .expect("failed to execute mkdir");
    Command::new("mount")
        .args(&["-o", "subvol=@home,compress=zstd"])
        .arg(&part_path)
        .arg("/mnt/home")
        .output()
        .expect("failed to execute mount");

    println!("btrfs subvolume @home on disk {} mounted to /mnt/home", part_path);
}
