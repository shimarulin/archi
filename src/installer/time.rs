use crate::utils::cmd;

pub fn set_timezone(timezone: &str) {
    cmd::exec(
        "arch-chroot",
        &["/mnt", "ln", "-sf", &*format!("/usr/share/zoneinfo/{}", timezone), "/etc/localtime"],
    );
}

pub fn enable_time_sync() {
    cmd::exec(
        "arch-chroot",
        &["/mnt", "systemctl", "enable", "systemd-timesyncd.service"],
    );
}

pub fn setup(timezone: &str) {
    set_timezone(&timezone);
    enable_time_sync();
}