use crate::utils::cmd;

pub fn upgrade_archlinux_keyring() {
    println!("Upgrade archlinux-keyring");
    cmd::exec(
        "pacman",
        vec![vec!["-Sy"], vec!["archlinux-keyring"], vec!["--noconfirm"]]
            .concat()
            .as_slice(),
    );
}

pub fn install() {
    println!("Install packages");
    cmd::exec(
        "pacstrap",
        vec![
            vec!["/mnt"],
            vec![
                // Required
                "base",
                "btrfs-progs",
                "grub",
                "efibootmgr",
                "dosfstools",
                "linux",
                "linux-firmware",
                "sudo",
                "python",
                // Network management (TODO: select alternatives)
                "networkmanager",
                // Text editor (TODO: select alternatives)
                "neovim",
                // curl package will be installed as 'networkmanager' dependency
                "curl",
            ],
        ]
        .concat()
        .as_slice(),
    );
}
