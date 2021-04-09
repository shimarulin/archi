mod facts;
mod questions;

pub struct Config {
    pub facts: facts::Facts,
    pub answers: questions::Answers
}

pub fn setup() -> Config {
    let facts = facts::gathering_facts();

    Config {
        facts,
        answers: questions::ask_questions(),
    }
}