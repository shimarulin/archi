use std::fs::OpenOptions;
use std::io::Write;
use std::process::Command;

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
        .output()
        .expect("failed to execute grub-install");

    println!("GRUB2 BIOS installed");
}

fn grub_efi_install(disk_path: &str) {
    let device_efi_path = format!("{}{}", disk_path, "2");
    let boot_efi_path = "/mnt/boot/EFI";

    Command::new("mkdir")
        .arg("-p")
        .arg(&boot_efi_path)
        .output()
        .expect("failed to execute mkdir");

    Command::new("mount")
        .arg(&device_efi_path)
        .arg(&boot_efi_path)
        .output()
        .expect("failed to execute mount");

    Command::new("arch-chroot")
        .arg("/mnt")
        .args(&[
            "grub-install",
            "--target=x86_64-efi",
            "--recheck",
            "--removable",
            "--boot-directory=/boot",
            "--efi-directory=/boot/EFI",
            "--bootloader-id=arch",
            // "--force",
        ])
        .output()
        .expect("failed to execute grub-install");

    println!("GRUB2 UEFI installed");
}

fn create_grub_entrypoint() {
    let cfg_file_path = "/mnt/boot/grub/grub.cfg";
    let mut cfg_file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open("/mnt/boot/grub/grub.cfg")
        .ok()
        .expect(&*format!("Couldn't open {} file.", cfg_file_path));

    cfg_file
        .write_all(". $prefix/config.cfg".as_ref())
        .ok()
        .expect(&*format!("Couldn't write {} file.", cfg_file_path));

    println!("{} created", cfg_file_path);
}

fn create_grub_config() {
    let cfg_file_path = "/mnt/boot/grub/config.cfg";
    let mut cfg_file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(&cfg_file_path)
        .ok()
        .expect(&*format!("Couldn't open {} file.", cfg_file_path));

    let content = "set timeout=5
menuentry \"Arch Linux\" {
  insmod btrfs
  linux /@/boot/vmlinuz-linux root=LABEL=System ro rootflags=subvol=@
  initrd /@/boot/initramfs-linux.img
}";

    cfg_file
        .write_all(&content.as_ref())
        .ok()
        .expect(&*format!("Couldn't write {} file.", cfg_file_path));

    println!("{} created", cfg_file_path);
}

fn protect_grub_cfg() {
    Command::new("chattr")
        .args(&["+i", "/mnt/boot/grub/grub.cfg"])
        .output()
        .expect("failed to execute chattr");

    println!("grub.cfg protected");
}

pub fn install(disk_path: &str) {
    grub_mbr_install(&disk_path);
    grub_efi_install(&disk_path);
    create_grub_entrypoint();
    create_grub_config();
    protect_grub_cfg();

    println!("GRUB bootloader installed");
}
