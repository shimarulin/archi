use crate::setup::facts::{disks, Facts};

mod confirm;
mod disk;
mod hostname;
mod timezone;
mod user;
mod swap_size;

pub struct Answers {
    pub disk: disks::BlockDevice,
    pub user: user::User,
    pub confirm: bool,
    pub hostname: String,
    pub timezone: String,
    pub swap_size: i32,
}

pub fn ask_questions(facts_: &Facts) -> Answers {
    let disk_ = disk::select_disk(&facts_.disks);
    let user_ = user::setup_user();
    let hostname_ = hostname::input_hostname();
    let timezone_ = timezone::select_timezone(&facts_.timezone);
    let confirm_ = confirm::ask_confirm(&disk_);
    let swap_size_ = swap_size::input_swap_size().parse::<i32>().unwrap();

    Answers {
        disk: disk_,
        user: user_,
        confirm: confirm_,
        hostname: hostname_,
        timezone: timezone_,
        swap_size: swap_size_,
    }
}
