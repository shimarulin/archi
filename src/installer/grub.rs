use std::process::{Command, Stdio};
use crate::utils::file;

fn grub_mbr_install(disk_path: &str) {
    Command::new("arch-chroot")
        .arg("/mnt")
        .args(&[
            "grub-install",
            "--target=i386-pc",
            "--recheck",
            "--boot-directory=/boot",
            // "--force",
            &disk_path,
        ])
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .output()
        .expect("ERR");

    println!("GRUB2 BIOS installed");
}

fn grub_efi_install(disk_path: &str) {
    Command::new("arch-chroot")
        .arg("/mnt")
        .args(&[
            "grub-install",
            "--target=x86_64-efi",
            "--recheck",
            "--removable",
            "--boot-directory=/boot",
            "--efi-directory=/boot/efi",
            &disk_path,
        ])
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .output()
        .expect("ERR");
}

fn create_grub_cfg() {
    file::create("/mnt/boot/grub/grub.cfg", ". $prefix/config.cfg");
}

fn create_grub_config_cfg() {
    let content = "set default=0
set timeout=5
menuentry \"Arch Linux\" {
  probe --set=UUID --fs-uuid $root
  linux /@/boot/vmlinuz-linux root=UUID=$UUID ro rootflags=subvol=@
  initrd /@/boot/initramfs-linux.img
}";
    file::create("/mnt/boot/grub/config.cfg", content);
}

fn protect_grub_cfg() {
    Command::new("chattr")
        .args(&["+i", "/mnt/boot/grub/grub.cfg"])
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .output()
        .expect("ERR");

    println!("grub.cfg protected");
}

fn add_grub_cfg_to_pacman_ignore() {
    file::replace_string("/mnt/etc/pacman.conf", "#NoUpgrade   =", "NoUpgrade   = boot/grub/grub.cfg");
}

fn create_efi_option(disk_path: &str) {
    Command::new("arch-chroot")
        .arg("/mnt")
        .args(&[
            "efibootmgr",
            "-c",
            "-d",
            &disk_path,
            "-p",
            "2", // EFI System partition
            "-L",
            "Arch Linux",
            "-l",
            "\\EFI\\BOOT\\BOOTX64.EFI",
        ])
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .output()
        .expect("ERR");

    println!("UEFI boot option created");
}

pub fn install(disk_path: &str, firmware: &str) {
    grub_mbr_install(&disk_path);
    grub_efi_install(&disk_path);

    create_grub_cfg();
    create_grub_config_cfg();
    protect_grub_cfg();
    add_grub_cfg_to_pacman_ignore();

    if firmware == "UEFI" {
        create_efi_option(&disk_path);
    }

    println!("GRUB bootloader installed");
}
