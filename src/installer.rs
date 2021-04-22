use crate::setup::Config;

mod partitioning;
mod format;

pub fn install(config: &Config) {
    partitioning::create_partitions(&*config.answers.disk.path);
    format::swap_init(&*config.answers.disk.path);
}