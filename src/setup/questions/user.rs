use dialoguer::{Input, Password};
use crate::setup::questions_theme::get_questions_theme;

pub struct User {
    pub username: String,
    pub password: String,
}

pub fn setup_user() -> User {
    User {
        username: Input::with_theme(&get_questions_theme())
            .with_prompt("Enter your username")
            .interact_text()
            .unwrap(),
        password: Password::with_theme(&get_questions_theme())
            .with_prompt("Enter your password")
            .with_confirmation("Repeat password", "Error: the passwords don't match.")
            .interact()
            .unwrap(),
    }
}
