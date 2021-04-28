use std::process::Command;

pub fn parted(disk_path: &str) {
    Command::new("parted")
        .arg("--script")
        .arg("--")
        .arg(disk_path)
        .args(&["mklabel", "gpt"])
        .args(&["mkpart", "\"BIOS boot\" fat32 1MiB 2MiB"])
        .args(&["set", "1 bios_grub on"])
        .args(&["mkpart", "\"EFI system\" fat32 2MiB 258MiB"])
        .args(&["mkpart", "\"Linux\" btrfs 258MiB -4001MiB"])
        .args(&["mkpart", "\"Linux swap\" linux-swap -4001MiB -1MiB"])
        .output()
        .expect("failed to execute parted");

    println!("Partitioning of {} is done.", disk_path);
}

fn format_system_partition(disk_partition_path: &str) {
    let output = Command::new("mkfs.btrfs")
        .args(&["--label", "System"])
        .arg("--force")
        .arg(&disk_partition_path)
        .output()
        .expect("failed to execute mkfs.btrfs");

    let mut stdout = String::from_utf8(output.stdout).unwrap();
    stdout.pop();

    println!(
        "btrfs filesystem on {} is created:\n{}",
        disk_partition_path, stdout
    );
}

fn mount_system_partition(disk_partition_path: &str) {
    Command::new("mount")
        .arg(&disk_partition_path)
        .arg("/mnt")
        .output()
        .expect("failed to execute mkfs.btrfs");

    println!("btrfs filesystem on {} is mounted to /mnt", disk_partition_path);
}

fn umount_system_partition() {
    Command::new("umount")
        .arg("/mnt")
        .output()
        .expect("failed to execute umount");

    println!("filesystem on /mnt is unmounted");
}

fn create_subvolume(name: &str) {
    let subvolume_path: &str = &*format!("{}{}", "/mnt/", name);
    Command::new("btrfs")
        .args(&["subvolume", "create", subvolume_path])
        .output()
        .expect("failed to execute btrfs");

    println!("btrfs subvolume {} created", subvolume_path);
}

pub fn format(disk_path: &str) {
    let device_system_path = format!("{}{}", disk_path, "3");
    let subvolume_names = ["@", "@home"];

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
        .output()
        .expect("failed to execute mount");

    println!("btrfs subvolume @ on disk {} mounted to /mnt", disk_partition_path);
}

fn mount_home_subvolume(disk_partition_path: &str) {
    Command::new("mkdir")
        .arg("/mnt/home")
        .output()
        .expect("failed to execute mkdir");
    Command::new("mount")
        .args(&["-o", "subvol=@home,compress=zstd"])
        .arg(&disk_partition_path)
        .arg("/mnt/home")
        .output()
        .expect("failed to execute mount");

    println!("btrfs subvolume @home on disk {} mounted to /mnt/home", disk_partition_path);
}

pub fn mount(disk_path: &str) {
    let device_system_path = format!("{}{}", disk_path, "3");
    mount_root_subvolume(&device_system_path);
    mount_home_subvolume(&device_system_path);
}
