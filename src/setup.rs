mod questions;

pub struct Config {
    pub answers: questions::Answers
}

pub fn setup() -> Config {
    Config {
        answers: questions::ask_questions(),
    }
}