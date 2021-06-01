use dialoguer::console::style;

mod installer;
mod setup;
mod utils;
mod actions;

fn main() {
    let config = setup::setup();

    let top_line          = style("\n  ╔══════════════════════════════════════════════════════════╗").cyan();
    let bottom_line       = style("\n  ╚══════════════════════════════════════════════════════════╝").cyan();
    let empty_line        = style("\n  ║                                                          ║").cyan();
    let cancel_line       = style("\n  ║               Installation canceled by user              ║").cyan();
    let success_line      = style("\n  ║ Arch Linux installed successfully                        ║").cyan();
    let disk_line         = style("\n  ║ Disk:                                                    ║").cyan();
    let user_line         = style("\n  ║ User:                                                    ║").cyan();
    let next_step_line0   = style("\n  ║ You Arch Linux installation still mounted to /mnt        ║").cyan();
    let next_step_line1   = style("\n  ║ You can reboot now or continue setup                     ║").cyan();
    let disk_path_line = format!(
        "{start} {key:<8}: {value:<46} {end}",
        start = style("\n  ║").cyan(),
        end = style("║").cyan(),
        key = style("  Path").cyan(),
        value = style(&config.answers.disk.path).yellow(),
    );
    let disk_serial_line = format!(
        "{start} {key:<8}: {value:<46} {end}",
        start = style("\n  ║").cyan(),
        end = style("║").cyan(),
        key = style("  Serial").cyan(),
        value = style(&config.answers.disk.serial).yellow(),
    );
    let disk_model_line = format!(
        "{start} {key:<8}: {value:<46} {end}",
        start = style("\n  ║").cyan(),
        end = style("║").cyan(),
        key = style("  Model").cyan(),
        value = style(&config.answers.disk.model).yellow(),
    );
    let username_line = format!(
        "{start} {key:<8}: {value:<46} {end}",
        start = style("\n  ║").cyan(),
        end = style("║").cyan(),
        key = style("  Name").cyan(),
        value = style(&config.answers.user.username).yellow(),
    );
    let userpass_line = format!(
        "{start} {key:<8}: {value:<46} {end}",
        start = style("\n  ║").cyan(),
        end = style("║").cyan(),
        key = style("  Pass").cyan(),
        value = style(&config.answers.user.password).yellow(),
    );
    let hostname_line = format!(
        "{start} {key:<8}: {value:<46} {end}",
        start = style("\n  ║").cyan(),
        end = style("║").cyan(),
        key = style("Hostname").cyan(),
        value = style(&config.answers.hostname).yellow(),
    );
    let timezone_line = format!(
        "{start} {key:<8}: {value:<46} {end}",
        start = style("\n  ║").cyan(),
        end = style("║").cyan(),
        key = style("Timezone").cyan(),
        value = style(&config.answers.timezone).yellow(),
    );

    if config.answers.confirm == false {
        println!("{}{}{}", top_line, cancel_line, bottom_line);
        std::process::exit(0)
    }

    installer::install(&config);

    println!(
        "{start}{success}{empty}{disk}{model}{serial}{path}{user}{username}{pass}{host}{timezone}{empty}{next0}{next1}{end}\n",
        start = top_line,
        end = bottom_line,
        success = success_line,
        disk = disk_line,
        path = disk_path_line,
        serial = disk_serial_line,
        model = disk_model_line,
        user = user_line,
        username = username_line,
        pass = userpass_line,
        host = hostname_line,
        timezone = timezone_line,
        empty = empty_line,
        next0 = next_step_line0,
        next1 = next_step_line1,
    );

    actions::confirm_reboot();
}
