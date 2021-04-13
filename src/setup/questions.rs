use crate::setup::facts::{Facts, disks};

mod disk;
mod user;

pub struct Answers {
    pub disk: disks::BlockDevice,
    pub user: user::User,
}

pub fn ask_questions(facts_: &Facts) -> Answers {
    Answers {
        disk: disk::select_disk(&facts_.disks),
        user: user::setup_user(),
    }
}
