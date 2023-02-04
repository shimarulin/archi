use crate::utils::cmd;
use crate::utils::disk;

const SUBVOLUME_NAMES: [&str; 8] = [
    "@/",
    "@/home",
    "@/opt",
    "@/root",
    "@/srv",
    "@/var/log",
    "@/var/opt",
    "@/.snapshots",
];

pub fn parted(disk_path: &str, swap_size: &i32) {
    let bios_start = "1MiB";
    let efi_start = "2MiB";
    let system_start = "258MiB";
    let swap_start = format!("-{}MiB", swap_size);

    let bios_partition = format!("\"BIOS boot\" fat32 {} {}", bios_start, efi_start);
    let efi_partition = format!("\"EFI system\" fat32 {} {}", efi_start, system_start);
    let system_partition = format!("\"Linux\" btrfs {} {}", system_start, swap_start);
    let swap_partition = format!("\"Linux swap\" linux-swap {} -1MiB", swap_start);

    cmd::exec(
        "parted",
        vec![
            vec!["--script", "--", disk_path],
            vec!["mklabel", "gpt"],
            vec!["mkpart", &bios_partition],
            vec!["set", "1", "bios_grub", "on"],
            vec!["mkpart", &efi_partition],
            vec!["set", "2", "boot", "on"],
            vec!["mkpart", &system_partition],
            vec!["mkpart", &swap_partition],
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
    let mut path_vec: Vec<&str> = name.split("/").collect();

    if path_vec.len() > 2 {
        path_vec.pop();
        let internal_path = &*format!("/mnt/{}", path_vec.join("/"));
        cmd::exec("mkdir", &["-p", internal_path]);
    }

    let subvolume_path: &str = &*format!("{}{}", "/mnt/", name);
    cmd::exec("btrfs", &["subvolume", "create", subvolume_path]);

    println!("btrfs subvolume {} created", subvolume_path);
}

pub fn format(disk_path: &str) {
    let device_efi_path = disk::get_partition_path(disk_path, "2");
    let device_system_path = disk::get_partition_path(disk_path, "3");

    format_efi_partition(&device_efi_path);
    format_system_partition(&device_system_path);
    mount_system_partition(&device_system_path);
    for subvolume_name in &SUBVOLUME_NAMES {
        create_subvolume(subvolume_name);
    }
    umount_system_partition();
}

fn mount_subvolume(disk_partition_path: &str, subvolume_name: &str, mount_path: &str) {
    let internal_mount_path = &*format!("/mnt{}", mount_path);
    cmd::exec("mkdir", &["-p", internal_mount_path]);
    cmd::exec(
        "mount",
        &[
            "-o",
            format!("subvol={},compress=zstd", subvolume_name).as_str(),
            disk_partition_path,
            internal_mount_path,
        ],
    );

    println!(
        "btrfs subvolume {} on disk {} mounted to /mnt{}",
        subvolume_name, disk_partition_path, mount_path
    );
}

fn mount_efi_partition(disk_partition_path: &str) {
    let boot_efi_path = "/mnt/esp";

    cmd::exec("mkdir", &["-p", boot_efi_path]);
    cmd::exec("mount", &[disk_partition_path, boot_efi_path]);

    println!("{} is mounted to {}", disk_partition_path, boot_efi_path);
}

pub fn mount(disk_path: &str) {
    let device_efi_path = disk::get_partition_path(disk_path, "2");
    let device_system_path = disk::get_partition_path(disk_path, "3");
    for subvolume_name in &SUBVOLUME_NAMES {
        mount_subvolume(
            &device_system_path,
            subvolume_name,
            subvolume_name.clone().replace("@", "").as_str(),
        );
    }
    mount_efi_partition(&device_efi_path);
}
