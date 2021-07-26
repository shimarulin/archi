use crate::setup::questions_theme::get_questions_theme;
use crate::utils::cmd;
use dialoguer::Confirm;

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
