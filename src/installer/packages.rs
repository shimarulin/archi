use crate::utils::cmd;

pub fn install() {
    println!("Install packages");
    cmd::exec(
        "pacstrap",
        vec![
            vec!["/mnt"],
            vec![
                "base",
                "btrfs-progs",
                "grub",
                "efibootmgr",
                "dosfstools",
                "linux",
                "linux-firmware",
                "nano",
                "networkmanager",
                "sudo",
            ],
        ]
        .concat()
        .as_slice(),
    );
}
