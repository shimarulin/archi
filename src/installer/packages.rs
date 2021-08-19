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
                "networkmanager",
                "sudo",
                "neovim",
                // curl package will be installed as 'networkmanager' dependency, just mention
                "curl",
            ],
        ]
        .concat()
        .as_slice(),
    );
}
