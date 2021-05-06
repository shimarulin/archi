use std::fs::{OpenOptions, File};
use std::io::Write;
use std::path::Path;

fn set_hostname(hostname: &str) {
    let path = Path::new("/mnt/etc/hostname");
    let display = path.display();

    let mut file = match File::create(&path) {
        Err(why) => panic!("couldn't create {}: {}", display, why),
        Ok(file) => file,
    };

    match file.write_all(hostname.as_bytes()) {
        Err(why) => panic!("couldn't write to {}: {}", display, why),
        Ok(_) => println!("successfully wrote hostname '{}' to {}", hostname, display),
    };
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