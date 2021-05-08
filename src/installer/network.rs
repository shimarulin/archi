use crate::utils::cmd;
use crate::utils::file;

fn set_hostname(hostname: &str) {
    file::create("/mnt/etc/hostname", &hostname);
}

fn update_hosts_file(hostname: &str) {
    let content = format!(
        "\
127.0.0.1        localhost
::1              localhost
127.0.1.1        {hostname}.localdomain        {hostname}
",
        hostname = hostname
    );

    file::append("/mnt/etc/hosts", &*content);
}

fn nm_enable() {
    // enable NetworkManager
    cmd::exec(
        "arch-chroot",
        &["/mnt", "systemctl", "enable", "NetworkManager.service"],
    );
    // enable Wi-Fi
    cmd::exec(
        "arch-chroot",
        &["/mnt", "systemctl", "enable", "wpa_supplicant.service"],
    );
}

pub fn setup(hostname: &str) {
    set_hostname(&hostname);
    update_hosts_file(&hostname);
    nm_enable();
}
