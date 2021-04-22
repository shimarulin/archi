use crate::setup::Config;

mod disk_partition;
mod format_partitions;
mod mount_partitions;
mod umount_partitions;

pub fn install(config: &Config) {
    let subvolume_names = ["@", "@home"];

    disk_partition::create_partitions(&*config.answers.disk.path);

    format_partitions::swap_init(&*config.answers.disk.path);
    format_partitions::system_format(&*config.answers.disk.path);

    mount_partitions::enable_swap(&*config.answers.disk.path);
    mount_partitions::mount_root(&*config.answers.disk.path);
    for subvolume_name in &subvolume_names {
        mount_partitions::create_subvolume(subvolume_name);
    }

    umount_partitions::umount_root();
}