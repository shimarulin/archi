use crate::utils::input::answer_string_handler;
use crate::utils::message::format_message;
use inquire::{required, Text};

pub fn input_hostname() -> String {
    answer_string_handler(
        Text::new(&*format_message("Hostname"))
            .with_validator(required!("The hostname is required"))
            .prompt(),
    )
}
