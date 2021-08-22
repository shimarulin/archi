use crate::utils::input::answer_string_handler;
use crate::utils::message::format_message;
use inquire::{
    max_length, min_length, required,
    validator::{InquireLength, StringValidator},
    Text,
};

pub fn input_hostname() -> String {
    fn is_valid_hostname_characters(hostname: &str) -> bool {
        fn is_valid_char(byte: u8) -> bool {
            (byte >= b'a' && byte <= b'z')
                || (byte >= b'A' && byte <= b'Z')
                || (byte >= b'0' && byte <= b'9')
                || byte == b'-'
        }

        !(hostname.bytes().any(|byte| !is_valid_char(byte)))
    }

    fn is_valid_hostname_starts(hostname: &str) -> bool {
        !(hostname.starts_with('-'))
    }

    fn is_valid_hostname_ends(hostname: &str) -> bool {
        !(hostname.ends_with('-'))
    }

    let allowed_characters: StringValidator = &|input| {
        match is_valid_hostname_characters(input) {
        true => Ok(()),
        false => Err(String::from(
            "The hostname can contain ASCII(7) letters from a to z, the digits from 0 to 9, and the hyphen (-)",
        )),
    }
    };

    let allowed_starts: StringValidator = &|input| match is_valid_hostname_starts(input) {
        true => Ok(()),
        false => Err(String::from("The hostname may not start with a hyphen (-)")),
    };

    let allowed_ends: StringValidator = &|input| match is_valid_hostname_ends(input) {
        true => Ok(()),
        false => Err(String::from("The hostname may not end with a hyphen (-)")),
    };

    answer_string_handler(
        Text::new(&*format_message("Hostname"))
            .with_validator(required!("The hostname is required"))
            .with_validator(allowed_characters)
            .with_validator(allowed_starts)
            .with_validator(allowed_ends)
            .with_validator(min_length!(
                1,
                "The hostname must be from 1 to 63 characters long"
            ))
            .with_validator(max_length!(
                63,
                "The hostname must be from 1 to 63 characters long"
            ))
            .prompt(),
    )
}
