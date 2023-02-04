use crate::setup::Config;
use crate::utils;

mod disk;
mod env;
mod fstab;
mod grub;
mod mkinitcpio;
mod network;
mod packages;
mod swap;
mod time;
mod user;

pub fn install(config: &Config) {
    swap::off();
    disk::parted(&*config.answers.disk.path, &config.answers.swap_size);

    let device_swap_path = utils::disk::get_partition_path(&config.answers.disk.path, "4");
    swap::init(&device_swap_path);
    swap::on(&device_swap_path);

    disk::format(&*config.answers.disk.path);
    disk::mount(&*config.answers.disk.path);

    packages::upgrade_archlinux_keyring();
    packages::install();
    fstab::generate();
    grub::install(&*config.answers.disk.path, &*config.facts.firmware);
    mkinitcpio::process();

    user::setup(
        &*config.answers.user.username,
        &*config.answers.user.password,
    );

    time::setup(&*config.answers.timezone);
    network::setup(&*config.answers.hostname);

    env::setup();
}
