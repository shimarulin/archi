use crate::setup::questions_theme::get_questions_theme;
use dialoguer::Select;
use std::process::Command;

pub fn select(timezone_default: &str) -> String {
    let output = Command::new("timedatectl")
        .arg("list-timezones")
        .output()
        .expect("ERR");

    let raw_timezone_list = String::from_utf8(output.stdout).unwrap();

    let selections = &raw_timezone_list.split("\n").collect::<Vec<_>>();

    let index_default = selections.iter().position(|&x| x == timezone_default);

    let selection = Select::with_theme(&get_questions_theme())
        .with_prompt("Select timezone")
        .items(&selections[..])
        .default(index_default.unwrap())
        .paged(true)
        .interact()
        .unwrap();

    (&selections[selection]).to_string()
}
