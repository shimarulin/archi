use serde::{Deserialize, Serialize};
use serde_json;
use std::process::Command;

#[derive(Serialize, Deserialize)]
pub struct BlockDevice {
    pub name: String,
    pub path: String,
    pub model: String,
    pub serial: String,
    pub size: String,
}

#[derive(Serialize, Deserialize)]
pub struct LsblkData {
    pub blockdevices: Vec<BlockDevice>,
}

pub fn get_disk_info() -> Vec<BlockDevice> {
    let lsblk_output = Command::new("lsblk")
        .arg("--json")
        // 8 - SATA and USB devices
        // 259 - NVME devices
        .args(&["--include", "8,259"])
        .args(&["--output", "NAME,PATH,SIZE,MODEL,SERIAL"])
        .output()
        .expect("failed to execute lsblk");

    let lsblk_stdout = String::from_utf8(lsblk_output.stdout).unwrap();

    let lsblk_data: LsblkData = serde_json::from_str(lsblk_stdout.as_str()).unwrap();

    lsblk_data.blockdevices
}
