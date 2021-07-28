use crate::utils::input::answer_option_handler;
use crate::utils::message::format_message;
use inquire::Select;
use std::process::Command;

pub fn select(timezone_default: &str) -> String {
    let output = Command::new("timedatectl")
        .arg("list-timezones")
        .output()
        .expect("ERR");

    let list_timezones_output_string = String::from_utf8(output.stdout).unwrap();

    let timezone_list = &list_timezones_output_string.split("\n").collect::<Vec<_>>();

    let index_default = timezone_list
        .iter()
        .position(|&x| x == timezone_default)
        .unwrap();

    answer_option_handler(
        Select::new(&*format_message("Timezone"), &timezone_list)
            .with_page_size(11)
            .with_starting_cursor(index_default)
            .prompt(),
    )
}
