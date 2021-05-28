use crate::setup::questions_theme::get_questions_theme;
use dialoguer::Input;

pub fn input_hostname() -> String {
    Input::with_theme(&get_questions_theme())
        .with_prompt("Enter hostname")
        .interact_text()
        .unwrap()
}
