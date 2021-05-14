use crate::setup::facts::{Facts, disks};

mod disk;
mod user;
mod confirm;
mod hostname;
mod timezone;

pub struct Answers {
    pub disk: disks::BlockDevice,
    pub user: user::User,
    pub confirm: bool,
    pub hostname: String,
    pub timezone: String,
}

pub fn ask_questions(facts_: &Facts) -> Answers {
    let disk_ = disk::select_disk(&facts_.disks);
    let user_ = user::setup_user();
    let hostname_ = hostname::input_hostname();
    let timezone_ = timezone::select(&facts_.timezone);
    let confirm_ = confirm::ask_confirm(&disk_);

    Answers {
        disk: disk_,
        user: user_,
        confirm: confirm_,
        hostname: hostname_,
        timezone: timezone_,
    }
}
