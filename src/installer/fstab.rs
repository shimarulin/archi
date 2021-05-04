use std::fs::OpenOptions;
use std::io::Write;
use std::process::Command;

pub fn generate() {
    let mut fstab_file = OpenOptions::new()
        .read(true)
        .write(true)
        // .append(true)
        .open("/mnt/etc/fstab")
        .ok()
        .expect("Couldn't open /mnt/etc/fstab file.");

    let output = Command::new("genfstab")
        .args(&["-U", "-p", "/mnt"])
        .output()
        .expect("failed to execute genfstab");

    fstab_file
        .write_all(&output.stdout)
        .ok()
        .expect("Couldn't write /mnt/etc/fstab file.");

    println!("fstab created");
}
