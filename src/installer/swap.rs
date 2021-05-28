use crate::utils::cmd;

pub fn init(disk_partition_path: &str) {
    cmd::exec("mkswap", &[disk_partition_path]);

    println!("swap on {} initialized", disk_partition_path);
}

pub fn on(disk_partition_path: &str) {
    cmd::exec("swapon", &[disk_partition_path]);

    println!("swap on {} enabled", disk_partition_path);
}

pub fn off() {
    cmd::exec("swapoff", &["--all"]);

    println!("All swap disabled");
}
