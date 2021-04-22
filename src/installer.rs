use crate::setup::Config;

mod disk_partition;
mod format_partitions;

pub fn install(config: &Config) {
    disk_partition::create_partitions(&*config.answers.disk.path);
    format_partitions::system_format(&*config.answers.disk.path);
    format_partitions::swap_init(&*config.answers.disk.path);
}