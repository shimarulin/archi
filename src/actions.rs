use crate::utils::cmd;
use crate::utils::input::answer_boolean_handler;
use crate::utils::message::format_message;
use inquire::Confirm;

pub fn confirm_reboot() {
    if answer_boolean_handler(
        Confirm::new(&*format_message("Reboot now?"))
            .with_help_message(
                "You can reboot to installed system now or continue and reboot later",
            )
            .with_default(false)
            .prompt(),
    ) {
        cmd::run("reboot");
    } else {
        std::process::exit(0);
    }
}
