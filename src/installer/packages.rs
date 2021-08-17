use crate::utils::{cmd, file};

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
                // curl package will be installed as 'networkmanager' or 'git' dependency, just mention
                "curl",
                "wget",
                // OpenSSH client and server
                "openssh",
                "git",
                "python",
                "python-pip",
                "ansible",
            ],
        ]
        .concat()
        .as_slice(),
    );
}

fn set_default_editor() {
    let content = format!(
        "
EDITOR=/usr/bin/nvim
VISUAL=/usr/bin/nvim
"
    );

    file::append("/mnt/etc/environment", &*content);
}

pub fn setup() {
    set_default_editor();
}
