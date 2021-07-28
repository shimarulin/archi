use console::style;
use inquire::error::InquireResult;
use inquire::OptionAnswer;

pub fn exit_by_user() {
    let top_line = style("\n  ╔══════════════════════════════════════════════════════════╗").cyan();
    let bottom_line =
        style("\n  ╚══════════════════════════════════════════════════════════╝").cyan();
    let cancel_line =
        style("\n  ║               Installation canceled by user              ║").cyan();

    println!("{}{}{}", top_line, cancel_line, bottom_line);
    std::process::exit(0);
}

pub fn answer_string_handler(answer: InquireResult<String>) -> String {
    match answer {
        Ok(_) => answer.unwrap(),
        Err(_) => {
            exit_by_user();
            String::from("")
        }
    }
}

pub fn answer_option_handler(answer: InquireResult<OptionAnswer>) -> String {
    match answer {
        Ok(_) => String::from(answer.unwrap().value),
        Err(_) => {
            exit_by_user();
            String::from("")
        }
    }
}

pub fn answer_boolean_handler(answer: InquireResult<bool>) -> bool {
    match answer {
        Ok(_) => answer.unwrap(),
        Err(_) => {
            exit_by_user();
            false
        }
    }
}
