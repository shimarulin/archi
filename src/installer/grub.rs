use std::fs::{File, OpenOptions};
use std::io::Write;
use std::path::Path;
use std::process::{Command, Stdio};

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

fn grub_mkconfig() {
    // TODO: replace to custom config
    Command::new("arch-chroot")
        .arg("/mnt")
        .args(&["grub-mkconfig", "-o", "/boot/grub/grub.cfg"])
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .output()
        .expect("ERR");

    println!("GRUB2 mkconfig");
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

// fn create_grub_config() {
//     let cfg_file_path = "/mnt/boot/grub/config.cfg";
//     let mut cfg_file = OpenOptions::new()
//         .read(true)
//         .write(true)
//         .create(true)
//         .open(&cfg_file_path)
//         .ok()
//         .expect(&*format!("Couldn't open {} file.", cfg_file_path));
//
//     let content = "set timeout=5
// menuentry \"Arch Linux\" {
//   insmod btrfs
//   linux /@/boot/vmlinuz-linux root=LABEL=System ro rootflags=subvol=@
//   initrd /@/boot/initramfs-linux.img
// }";
//
//     cfg_file
//         .write_all(&content.as_ref())
//         .ok()
//         .expect(&*format!("Couldn't write {} file.", cfg_file_path));
//
//     println!("{} created", cfg_file_path);
// }

// fn create_grub_config() {
//     let path = Path::new("/mnt/boot/grub/config.cfg");
//     let display = path.display();
//
//     let mut file = match File::create(&path) {
//         Err(why) => panic!("couldn't create {}: {}", display, why),
//         Ok(file) => file,
//     };
//
//     let content = "set timeout=5
// menuentry \"Arch Linux\" {
//   insmod btrfs
//   linux /vmlinuz-linux root=LABEL=System ro rootflags=subvol=@
//   initrd /initramfs-linux.img
// }";
//
//     match file.write_all(hostname.as_bytes()) {
//         Err(why) => panic!("couldn't write to {}: {}", display, why),
//         Ok(_) => println!("successfully wrote hostname '{}' to {}", hostname, display),
//     };
// }

fn protect_grub_cfg() {
    Command::new("chattr")
        .args(&["+i", "/mnt/boot/grub/grub.cfg"])
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .output()
        .expect("ERR");

    println!("grub.cfg protected");
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
    grub_mkconfig();
    // create_grub_entrypoint();
    // create_grub_config();
    // protect_grub_cfg();
    if firmware == "UEFI" {
        create_efi_option(&disk_path);
    }

    println!("GRUB bootloader installed");
}
