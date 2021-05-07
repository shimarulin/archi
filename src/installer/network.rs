use std::fs::{OpenOptions, File};
use std::io::Write;
use std::path::Path;
use crate::utils::file;

fn set_hostname(hostname: &str) {
    file::create("/mnt/etc/hostname", &hostname);
}

fn update_hosts_file(hostname: &str) {
    let cfg_file_path = "/mnt/etc/hosts";
    let mut cfg_file = OpenOptions::new()
        .read(true)
        .append(true)
        .open(&cfg_file_path)
        .ok()
        .expect(&*format!("Couldn't open {} file.", cfg_file_path));

    let content = format!("\
127.0.0.1        localhost
::1              localhost
127.0.1.1        {hostname}.localdomain        {hostname}
", hostname = hostname);

    cfg_file
        .write_all(&content.as_ref())
        .ok()
        .expect(&*format!("Couldn't write {} file.", cfg_file_path));

    println!("{} updated", cfg_file_path);
}

pub fn setup (hostname: &str) {
    set_hostname(&hostname);
    update_hosts_file(&hostname);
}