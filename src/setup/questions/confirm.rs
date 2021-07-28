use crate::setup::facts::disks;
use crate::utils::input::answer_boolean_handler;
use console::style;
use inquire::Confirm;

pub fn ask_confirm(block_device: &disks::BlockDevice) -> bool {
    let disk_info = format!(
        "({model} {size}, serial: {serial})",
        model = block_device.model,
        size = block_device.size,
        serial = block_device.serial,
    );

    let warning_note = format!(
        "{left_border}{note_start} {path}\n{left_border}{disk_info}\n{left_border}{note_danger} {note_end}",
        left_border = style("║ ").yellow(),
        note_start = style("Please note that all data on the").yellow(),
        path = style(&block_device.path).red(),
        disk_info = style(disk_info).yellow(),
        note_danger = style("will be erased").red(),
        note_end = style("prior to installation.").yellow(),
    );

    println!("{}", warning_note);

    answer_boolean_handler(
        Confirm::new("Do you confirm the installation?")
            .with_default(false)
            .prompt(),
    )
}
