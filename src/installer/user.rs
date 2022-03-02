use crate::utils::cmd;
use crate::utils::file;
use std::io::Write;
use std::process::{Command, Stdio};

pub fn fill_skel() {
    cmd::exec("mkdir", &["-p", "/mnt/etc/skel/.local/bin"]);
}

pub fn set_user_password(username: &str, password: &str) {
    let passwd_input = format!("{p}\n{p}", p = password);

    let mut process = match Command::new("arch-chroot")
        .arg("/mnt")
        .arg("passwd")
        .arg(&username)
        .stdin(Stdio::piped())
        .spawn()
    {
        Err(why) => panic!("couldn't spawn passwd: {}", why),
        Ok(process) => process,
    };

    match process
        .stdin
        .take()
        .unwrap()
        .write_all(passwd_input.as_bytes())
    {
        Err(why) => panic!("couldn't write to passwd stdin: {}", why),
        Ok(_) => println!("sent password to passwd"),
    }

    let output = process.wait_with_output().expect("ERR");

    if output.status.success() {
        println!("{}", String::from_utf8(output.stdout).unwrap());
    }
}

pub fn create_user(username: &str) {
    cmd::exec("arch-chroot", &["/mnt", "useradd", "-m", &username]);
}

pub fn add_user_to_groups(username: &str, groups: &[&str]) {
    cmd::exec(
        "arch-chroot",
        &[
            &["/mnt", "usermod", "--append", "--groups"],
            groups,
            &[username],
        ]
        .concat(),
    );
}

pub fn lock_login_as_root() {
    cmd::exec("arch-chroot", &["/mnt", "passwd", "-l", "root"]);
}

pub fn enable_wheel_group() {
    file::replace_string(
        "/mnt/etc/sudoers",
        "# %wheel ALL=(ALL:ALL) ALL",
        "%wheel ALL=(ALL:ALL) ALL",
    )
}

pub fn setup(username: &str, password: &str) {
    fill_skel();
    create_user(&username);
    set_user_password(&username, &password);
    enable_wheel_group();
    add_user_to_groups(&username, &["wheel"]);
    lock_login_as_root();
}
