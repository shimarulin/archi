use crate::setup::questions_theme::get_questions_theme;
use dialoguer::console::style;
use dialoguer::Confirm;
use crate::utils::cmd;

pub fn confirm_reboot() {
    if Confirm::with_theme(&get_questions_theme())
        .with_prompt("Reboot now?")
        .interact()
        .unwrap()
    {

        cmd::run("reboot");
    } else {
        std::process::exit(0);
    }
}
