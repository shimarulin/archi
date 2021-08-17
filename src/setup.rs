mod facts;
mod questions;

pub struct Config {
    pub facts: facts::Facts,
    pub answers: questions::Answers,
}

pub fn setup() -> Config {
    let facts_ = facts::gathering_facts();
    let answers_ = questions::ask_questions(&facts_);

    Config {
        facts: facts_,
        answers: answers_,
    }
}
