use crate::setup::facts::disks;
use crate::utils::input::answer_boolean_handler;
use crate::utils::message::format_message;
use inquire::Confirm;

pub fn ask_confirm(block_device: &disks::BlockDevice) -> bool {
    let disk_info = format!(
        "WARNING: This will ERASE ALL DATA on the ENTIRE DISK {path}",
        path = block_device.path,
    );

    answer_boolean_handler(
        Confirm::new(&*format_message("Confirm the installation?"))
            .with_help_message(&*disk_info)
            .with_default(false)
            .prompt(),
    )
}
