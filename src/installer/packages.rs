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

pub fn install(kernel: &str, editor: &str) {
    let mut packages = vec![
        "base",
        "btrfs-progs",
        "grub",
        "efibootmgr",
        "dosfstools",
        "linux-firmware",
        "sudo",
        "python",
        // Network management (TODO: select alternatives)
        "networkmanager",
        // curl package will be installed as 'networkmanager' dependency
        "curl",
    ];

    packages.push(kernel);
    packages.push(editor);

    println!("Install packages");
    cmd::exec("pacstrap", vec![vec!["/mnt"], packages].concat().as_slice());
}
