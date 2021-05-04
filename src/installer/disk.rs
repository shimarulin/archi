use std::process::{Command, Stdio};

pub fn parted(disk_path: &str) {
    Command::new("parted")
        .arg("--script")
        .arg("--")
        .arg(disk_path)
        .args(&["mklabel", "gpt"])
        .args(&["mkpart", "\"BIOS boot\" fat32 1MiB 2MiB"])
        .args(&["set", "1", "bios_grub", "on"])
        .args(&["mkpart", "\"EFI system\" fat32 2MiB 258MiB"])
        .args(&["set", "2", "boot", "on"])
        .args(&["mkpart", "\"Linux\" btrfs 258MiB -4001MiB"])
        .args(&["mkpart", "\"Linux swap\" linux-swap -4001MiB -1MiB"])
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .output()
        .expect("ERR");

    println!("Partitioning of {} is done.", disk_path);
}

fn format_efi_partition(disk_partition_path: &str) {
    Command::new("mkfs.fat")
        .arg("-F32")
        .arg(&disk_partition_path)
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .output()
        .expect("ERR");

    println!(
        "fat32 filesystem created on {}",
        disk_partition_path
    );
}

fn format_system_partition(disk_partition_path: &str) {
    Command::new("mkfs.btrfs")
        .args(&["--label", "System"])
        .arg("--force")
        .arg(&disk_partition_path)
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .output()
        .expect("ERR");

    println!(
        "btrfs filesystem created on {}",
        disk_partition_path
    );
}

fn mount_system_partition(disk_partition_path: &str) {
    Command::new("mount")
        .arg(&disk_partition_path)
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .arg("/mnt")
        .output()
        .expect("ERR");

    println!("{} is mounted to /mnt", disk_partition_path);
}

fn umount_system_partition() {
    Command::new("umount")
        .arg("/mnt")
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .output()
        .expect("ERR");

    println!("filesystem on /mnt is unmounted");
}

fn create_subvolume(name: &str) {
    let subvolume_path: &str = &*format!("{}{}", "/mnt/", name);
    Command::new("btrfs")
        .args(&["subvolume", "create", subvolume_path])
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .output()
        .expect("ERR");

    println!("btrfs subvolume {} created", subvolume_path);
}

pub fn format(disk_path: &str) {
    let device_efi_path = format!("{}{}", disk_path, "2");
    let device_system_path = format!("{}{}", disk_path, "3");
    let subvolume_names = ["@", "@home"];

    format_efi_partition(&device_efi_path);
    format_system_partition(&device_system_path);
    mount_system_partition(&device_system_path);
    for subvolume_name in &subvolume_names {
        create_subvolume(subvolume_name);
    }
    umount_system_partition();
}

fn mount_root_subvolume(disk_partition_path: &str) {
    Command::new("mount")
        .args(&["-o", "subvol=@,compress=zstd"])
        .arg(&disk_partition_path)
        .arg("/mnt")
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .output()
        .expect("ERR");

    println!("btrfs subvolume @ on disk {} mounted to /mnt", disk_partition_path);
}

fn mount_home_subvolume(disk_partition_path: &str) {
    Command::new("mkdir")
        .arg("-p")
        .arg("/mnt/home")
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .output()
        .expect("ERR");

    Command::new("mount")
        .args(&["-o", "subvol=@home,compress=zstd"])
        .arg(&disk_partition_path)
        .arg("/mnt/home")
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .output()
        .expect("ERR");

    println!("btrfs subvolume @home on disk {} mounted to /mnt/home", disk_partition_path);
}

fn mount_efi_partition(disk_partition_path: &str) {
    let boot_efi_path = "/mnt/boot";

    Command::new("mkdir")
        .arg("-p")
        .arg(&boot_efi_path)
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .output()
        .expect("ERR");

    Command::new("mount")
        .arg(&disk_partition_path)
        .arg(&boot_efi_path)
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .output()
        .expect("ERR");

    println!("{} is mounted to {}", disk_partition_path, boot_efi_path);
}

pub fn mount(disk_path: &str) {
    let device_efi_path = format!("{}{}", disk_path, "2");
    let device_system_path = format!("{}{}", disk_path, "3");
    mount_root_subvolume(&device_system_path);
    mount_home_subvolume(&device_system_path);
    mount_efi_partition(&device_efi_path);
}
