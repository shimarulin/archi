use crate::setup::facts::{Facts};

mod disk;
mod user;

pub struct Answers {
    pub user: user::User,
}

pub fn ask_questions(facts_: &Facts) -> Answers {
    disk::select_disk(&facts_.disks);
    Answers {
        user: user::setup_user()
    }
}