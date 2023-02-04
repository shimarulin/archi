use crate::utils::cmd;

fn grub_mbr_install(disk_path: &str) {
    cmd::exec(
        "arch-chroot",
        &[
            "/mnt",
            "grub-install",
            "--target=i386-pc",
            "--recheck",
            "--boot-directory=/esp/GRUB",
            &disk_path,
        ],
    );
}

fn grub_efi_install(disk_path: &str) {
    cmd::exec(
        "arch-chroot",
        &[
            "/mnt",
            "grub-install",
            "--target=x86_64-efi",
            "--recheck",
            "--removable",
            "--boot-directory=/esp/GRUB",
            "--efi-directory=/esp",
            &disk_path,
        ],
    );
}

fn create_efi_option(disk_path: &str) {
    cmd::exec(
        "arch-chroot",
        &[
            "/mnt",
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
        ],
    );

    println!("UEFI boot option created");
}

fn grub_mkconfig() {
    cmd::exec(
        "arch-chroot",
        &["/mnt", "grub-mkconfig", "-o", "/esp/GRUB/grub/grub.cfg"],
    );
}

pub fn install(disk_path: &str, firmware: &str) {
    grub_mbr_install(&disk_path);
    grub_efi_install(&disk_path);

    grub_mkconfig();

    if firmware == "UEFI" {
        create_efi_option(&disk_path);
    }

    println!("GRUB bootloader installed");
}
