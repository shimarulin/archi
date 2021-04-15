use crate::setup::Config;

mod partitioning;

pub fn install(config: &Config) {
    partitioning::create_partitions(&*config.answers.disk.path, &*config.facts.firmware);
}