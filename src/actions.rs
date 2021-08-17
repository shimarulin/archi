use crate::utils::cmd;
use crate::utils::input::answer_boolean_handler;
use crate::utils::message::format_message;
use inquire::Confirm;

pub fn confirm_reboot() {
    if answer_boolean_handler(
        Confirm::new(&*format_message("Reboot now?"))
            .with_default(false)
            .prompt(),
    ) {
        cmd::run("reboot");
    } else {
        std::process::exit(0);
    }
}
