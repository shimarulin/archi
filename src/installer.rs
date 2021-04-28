use crate::setup::Config;

mod disk;
mod swap;

pub fn install(config: &Config) {
    let device_swap_path = format!("{}{}", config.answers.disk.path, "4");

    disk::parted(&*config.answers.disk.path);

    swap::init(&device_swap_path);
    swap::on(&device_swap_path);

    disk::format(&*config.answers.disk.path);
    disk::mount(&*config.answers.disk.path);
}