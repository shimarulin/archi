use crate::utils::input::exit_by_user;
use console::{style, Term};

mod actions;
mod installer;
mod setup;
mod utils;

fn main() {
    let term_width = usize::from(Term::stdout().size().1) - 3;

    let welcome_message_r0 = style("Welcome to").cyan().bright();

    let logo_r0 = style("    _             _     _ ").cyan().bright();
    let logo_r1 = style("   / \\   _ __ ___| |__ (_)").cyan().bright();
    let logo_r2 = style("  / _ \\ | '__/ __| '_ \\| |").cyan().bright();
    let logo_r3 = style(" / ___ \\| | | (__| | | | |").cyan().bright();
    let logo_r4 = style("/_/   \\_\\_|  \\___|_| |_|_|").cyan().bright();
    let logo = format!(
        "{:^width$}\n{:^width$}\n{:^width$}\n{:^width$}\n{:^width$}",
        logo_r0,
        logo_r1,
        logo_r2,
        logo_r3,
        logo_r4,
        width = term_width
    );

    let welcome_message_r1 = style("Simple and minimal Arch Linux installer")
        .cyan()
        .bright();

    let pre_question_message_r2 =
        style("Press ESC at any time to quit before starting the installation process")
            .cyan()
            .bright();

    println!(
        "{:^width$}\n{}\n\n{:^width$}\n\n\n{}",
        welcome_message_r0,
        logo,
        welcome_message_r1,
        pre_question_message_r2,
        width = term_width
    );

    let config = setup::setup();

    let top_line = style("\n  ╔══════════════════════════════════════════════════════════╗").cyan();
    let bottom_line =
        style("\n  ╚══════════════════════════════════════════════════════════╝").cyan();
    let empty_line =
        style("\n  ║                                                          ║").cyan();
    let success_line =
        style("\n  ║ Arch Linux installed successfully                        ║").cyan();
    let disk_line =
        style("\n  ║ Disk:                                                    ║").cyan();
    let user_line =
        style("\n  ║ User:                                                    ║").cyan();
    let next_step_line0 =
        style("\n  ║ You Arch Linux installation still mounted to /mnt        ║").cyan();
    let next_step_line1 =
        style("\n  ║ You can reboot now or continue setup                     ║").cyan();
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
        exit_by_user();
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
