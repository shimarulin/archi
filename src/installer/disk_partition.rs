use std::process::Command;

pub fn create_partitions(path: &str) {
    Command::new("parted")
        .arg("--script")
        .arg("--")
        .arg(path)
        .args(&["mklabel", "gpt"])
        .args(&["mkpart", "\"BIOS boot\" fat32 1MiB 2MiB"])
        .args(&["set", "1 bios_grub on"])
        .args(&["mkpart", "\"EFI system\" fat32 2MiB 258MiB"])
        .args(&["mkpart", "\"Linux\" btrfs 258MiB -4001MiB"])
        .args(&["mkpart", "\"Linux swap\" linux-swap -4001MiB -1MiB"])
        .output()
        .expect("failed to execute parted");

    println!("Partitioning of {} is done.",  path);
}