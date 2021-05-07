use crate::utils::file;

fn set_hostname(hostname: &str) {
    file::create("/mnt/etc/hostname", &hostname);
}

fn update_hosts_file(hostname: &str) {
    let content = format!("\
127.0.0.1        localhost
::1              localhost
127.0.1.1        {hostname}.localdomain        {hostname}
", hostname = hostname);

    file::append("/mnt/etc/hosts", &*content);
}

pub fn setup (hostname: &str) {
    set_hostname(&hostname);
    update_hosts_file(&hostname);
}