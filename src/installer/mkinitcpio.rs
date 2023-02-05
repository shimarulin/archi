use crate::utils::cmd;
use crate::utils::file;

fn configure() {
    file::replace_string(
        "/mnt/etc/mkinitcpio.conf",
        "HOOKS=(base udev autodetect modconf kms keyboard keymap consolefont block filesystems fsck)",
        "HOOKS=(base udev autodetect modconf kms keyboard keymap consolefont block filesystems)",
    );
}

fn build() {
    cmd::exec("arch-chroot", &["/mnt", "mkinitcpio", "-P"]);
}

pub fn process() {
    configure();
    build();
}
