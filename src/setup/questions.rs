use crate::setup::facts::{Facts};

mod disk;
mod user;

pub struct Answers {
    pub disk_path: String,
    pub user: user::User,
}

pub fn ask_questions(facts_: &Facts) -> Answers {
    Answers {
        disk_path: disk::select_disk(&facts_.disks),
        user: user::setup_user()
    }
}