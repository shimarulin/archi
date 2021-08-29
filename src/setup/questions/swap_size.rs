use crate::utils::input::answer_string_handler;
use crate::utils::message::format_message;
use inquire::{required, validator::StringValidator, Text};

pub fn input_swap_size() -> String {
    fn is_valid_hostname_characters(hostname: &str) -> bool {
        fn is_valid_char(byte: u8) -> bool {
            byte >= b'0' && byte <= b'9'
        }

        !(hostname.bytes().any(|byte| !is_valid_char(byte)))
    }

    let allowed_characters: StringValidator = &|input| match is_valid_hostname_characters(input) {
        true => Ok(()),
        false => Err(String::from("The swap size should be a number")),
    };

    answer_string_handler(
        Text::new(&*format_message("Swap size"))
            .with_default("4096")
            .with_help_message("Enter swap partition size in MiB")
            .with_validator(required!("The swap size is required"))
            .with_validator(allowed_characters)
            .prompt(),
    )
}
