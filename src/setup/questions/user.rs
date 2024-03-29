use crate::utils::input::answer_string_handler;
use crate::utils::message::format_message;
use crate::utils::render_config::get_render_config;
use inquire::{required, Password, Text};

pub struct User {
    pub username: String,
    pub password: String,
}

pub fn setup_user() -> User {
    let render_config = get_render_config();

    let _username = answer_string_handler(
        Text::new(&*format_message("Username"))
            .with_render_config(render_config)
            .with_validator(required!("The username is required"))
            .prompt(),
    );

    let _password = answer_string_handler(
        Password::new(&*format_message("Password"))
            .with_render_config(render_config)
            .with_display_toggle_enabled()
            .with_help_message("Ctrl+R to reveal password")
            .with_validator(required!("The password is required"))
            .prompt(),
    );

    User {
        username: _username,
        password: _password,
    }
}
