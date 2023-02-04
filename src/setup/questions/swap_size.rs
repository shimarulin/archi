use crate::utils::input::answer_string_handler;
use crate::utils::message::format_message;
use crate::utils::render_config::get_render_config;
use inquire::{required, validator::Validation, Text};

pub fn input_swap_size() -> String {
    fn is_valid_number(input: &str) -> bool {
        fn is_valid_char(byte: u8) -> bool {
            byte >= b'0' && byte <= b'9'
        }

        !(input.bytes().any(|byte| !is_valid_char(byte)))
    }

    let swap_size_validator = |input: &str| match is_valid_number(input) {
        true => Ok(Validation::Valid),
        false => Ok(Validation::Invalid(
            "The swap size should be a number".into(),
        )),
    };

    let render_config = get_render_config();

    answer_string_handler(
        Text::new(&*format_message("Swap size"))
            .with_render_config(render_config)
            .with_default("4096")
            .with_help_message("Enter swap partition size in MiB")
            .with_validator(required!("The swap size is required"))
            .with_validator(swap_size_validator)
            .prompt(),
    )
}
