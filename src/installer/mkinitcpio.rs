use crate::utils::cmd;
use crate::utils::file;

// TODO: can be removed later due to archlinux/mkinitcpio update
//   See https://github.com/archlinux/mkinitcpio/pull/60

fn configure() {
    // Manjaro: MODULES="crc32c-intel"
    // Arch: MODULES=()
    file::replace_string("/mnt/etc/mkinitcpio.conf", "MODULES=()", "MODULES=(vmd)");
    // Manjaro: HOOKS="base udev autodetect modconf block keyboard keymap consolefont resume filesystems"
    // Arch: HOOKS=(base udev autodetect modconf block filesystems keyboard fsck)
    file::replace_string(
        "/mnt/etc/mkinitcpio.conf",
        "HOOKS=(base udev autodetect modconf block filesystems keyboard fsck)",
        "HOOKS=(base udev block autodetect modconf filesystems keyboard)",
    );
}

fn build() {
    cmd::exec("arch-chroot", &["/mnt", "mkinitcpio", "-P"]);
}

pub fn process() {
    configure();
    build();
}
