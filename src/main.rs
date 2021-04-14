use dialoguer::console::style;

mod setup;

fn main() {
    let config = setup::setup();

    if config.answers.confirm == false {
        println!("{}", style("\n  ╔══════════════════╗\n  ║ Canceled by user ║\n  ╚══════════════════╝").cyan());
        std::process::exit(0)
    }

    println!("firmware is {}", config.facts.firmware);
    println!("disk_path is {}", config.answers.disk.path);
    println!("username is {}", config.answers.user.username);
    println!("password is {}", config.answers.user.password);
}
