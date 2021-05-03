use crate::setup::facts::{Facts, disks};

mod disk;
mod user;
mod confirm;
mod hostname;

pub struct Answers {
    pub disk: disks::BlockDevice,
    pub user: user::User,
    pub confirm: bool,
    pub hostname: String,
}

pub fn ask_questions(facts_: &Facts) -> Answers {
    let disk_ = disk::select_disk(&facts_.disks);
    let user_ = user::setup_user();
    let confirm_ = confirm::ask_confirm(&disk_);
    let hostname_ = hostname::input_hostname();

    Answers {
        disk: disk_,
        user: user_,
        confirm: confirm_,
        hostname: hostname_,
    }
}
