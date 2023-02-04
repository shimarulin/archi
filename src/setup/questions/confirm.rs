use crate::setup::facts::disks;
use crate::utils::input::answer_boolean_handler;
use crate::utils::message::format_message;
use crate::utils::render_config::get_render_config;
use inquire::Confirm;

pub fn ask_confirm(block_device: &disks::BlockDevice) -> bool {
    let disk_info = format!(
        "WARNING: This will ERASE ALL DATA on the ENTIRE DISK {path}",
        path = block_device.path,
    );

    let render_config = get_render_config();

    answer_boolean_handler(
        Confirm::new(&*format_message("Continue?"))
            .with_render_config(render_config)
            .with_help_message(&*disk_info)
            .with_default(false)
            .prompt(),
    )
}
