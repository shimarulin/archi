use crate::setup::facts::disks;
use crate::utils::input::answer_option_index_handler;
use crate::utils::message::format_message;
use inquire::Select;

pub fn select_disk(block_device_list: &Vec<disks::BlockDevice>) -> disks::BlockDevice {
    let selections = &block_device_list
        .iter()
        .map(|block_device| {
            format!(
                "{name:<14} {model:<24} {size:<8} {serial:<32}",
                name = block_device.path.to_string(),
                model = block_device.model.to_string(),
                size = block_device.size.to_string(),
                serial = block_device.serial.to_string()
            )
        })
        .collect::<Vec<_>>();

    let selections2 = &selections
        .iter()
        .map(|s| -> &str { &*s })
        .collect::<Vec<_>>();

    let selection = answer_option_index_handler(
        Select::new(&*format_message("Disk"), &selections2)
            .with_page_size(10)
            .prompt(),
    );

    disks::BlockDevice {
        name: (&block_device_list[selection].name).to_string(),
        path: (&block_device_list[selection].path).to_string(),
        model: (&block_device_list[selection].model).to_string(),
        serial: (&block_device_list[selection].serial).to_string(),
        size: (&block_device_list[selection].size).to_string(),
    }
}
