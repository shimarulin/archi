use std::process::{Command};

pub fn swap_init(disk_path: &str) {
    let part_number = "4";
    let part_path = format!("{}{}", disk_path, part_number);
    let output = Command::new("mkswap")
        .arg(&part_path)
        .output()
        .expect("failed to execute mkswap");

    let mut stdout = String::from_utf8(output.stdout).unwrap();
    stdout.pop();


    println!("Partition {}:\n{}", part_path, stdout);
}