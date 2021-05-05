use std::io::Write;
use std::process::{Command, Stdio};

pub fn set_root_password(password: &str) {
    let passwd_input = format!("{p}\n{p}", p = password);

    let mut process = match Command::new("arch-chroot")
        .arg("/mnt")
        .arg("passwd")
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
