use crate::setup::Config;

mod disk;
mod swap;
mod packages;
mod fstab;
mod grub;
// mod time;
mod user;
mod network;

pub fn install(config: &Config) {
    let device_swap_path = format!("{}{}", config.answers.disk.path, "4");

    disk::parted(&*config.answers.disk.path);

    swap::init(&device_swap_path);
    swap::on(&device_swap_path);

    disk::format(&*config.answers.disk.path);
    disk::mount(&*config.answers.disk.path);

    packages::install();
    fstab::generate();
    grub::install(&*config.answers.disk.path, &*config.facts.firmware);

    user::set_root_password(&*config.answers.user.password);

    network::setup(&*config.answers.hostname);
}