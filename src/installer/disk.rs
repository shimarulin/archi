use crate::utils::cmd;

pub fn parted(disk_path: &str) {
    cmd::exec(
        "parted",
        vec![
            vec!["--script", "--", disk_path],
            vec!["mklabel", "gpt"],
            vec!["mkpart", "\"BIOS boot\" fat32 1MiB 2MiB"],
            vec!["set", "1", "bios_grub", "on"],
            vec!["mkpart", "\"EFI system\" fat32 2MiB 258MiB"],
            vec!["set", "2", "boot", "on"],
            vec!["mkpart", "\"Linux\" btrfs 258MiB -4001MiB"],
            vec!["mkpart", "\"Linux swap\" linux-swap -4001MiB -1MiB"],
        ]
        .concat()
        .as_slice(),
    );

    println!("Partitioning of {} is done.", disk_path);
}

fn format_efi_partition(disk_partition_path: &str) {
    cmd::exec("mkfs.fat", &["-F32", disk_partition_path]);

    println!("fat32 filesystem created on {}", disk_partition_path);
}

fn format_system_partition(disk_partition_path: &str) {
    cmd::exec(
        "mkfs.btrfs",
        &["--label", "System", "--force", disk_partition_path],
    );

    println!("btrfs filesystem created on {}", disk_partition_path);
}

fn mount_system_partition(disk_partition_path: &str) {
    cmd::exec("mount", &[disk_partition_path, "/mnt"]);

    println!("{} is mounted to /mnt", disk_partition_path);
}

fn umount_system_partition() {
    cmd::exec("umount", &["/mnt"]);

    println!("filesystem on /mnt is unmounted");
}

fn create_subvolume(name: &str) {
    let subvolume_path: &str = &*format!("{}{}", "/mnt/", name);
    cmd::exec("btrfs", &["subvolume", "create", subvolume_path]);

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
    cmd::exec(
        "mount",
        &["-o", "subvol=@,compress=zstd", disk_partition_path, "/mnt"],
    );

    println!(
        "btrfs subvolume @ on disk {} mounted to /mnt",
        disk_partition_path
    );
}

fn mount_home_subvolume(disk_partition_path: &str) {
    cmd::exec("mkdir", &["-p", "/mnt/home"]);
    cmd::exec(
        "mount",
        &[
            "-o",
            "subvol=@home,compress=zstd",
            disk_partition_path,
            "/mnt/home",
        ],
    );

    println!(
        "btrfs subvolume @home on disk {} mounted to /mnt/home",
        disk_partition_path
    );
}

fn mount_efi_partition(disk_partition_path: &str) {
    let boot_efi_path = "/mnt/boot/efi";

    cmd::exec("mkdir", &["-p", boot_efi_path]);
    cmd::exec("mount", &[disk_partition_path, boot_efi_path]);

    println!("{} is mounted to {}", disk_partition_path, boot_efi_path);
}

pub fn mount(disk_path: &str) {
    let device_efi_path = format!("{}{}", disk_path, "2");
    let device_system_path = format!("{}{}", disk_path, "3");
    mount_root_subvolume(&device_system_path);
    mount_home_subvolume(&device_system_path);
    mount_efi_partition(&device_efi_path);
}
