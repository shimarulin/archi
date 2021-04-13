use crate::setup::facts::disks;
use crate::setup::questions_theme::get_questions_theme;
use dialoguer::console::style;
use dialoguer::{theme::ColorfulTheme, Select};

pub fn select_disk(block_device_list: &Vec<disks::BlockDevice>) -> disks::BlockDevice {
    let theme = ColorfulTheme {
        prompt_suffix: style(
            "═════════════════════════════════════════════════════════════════════════════"
                .to_string(),
        )
        .for_stderr()
        .black()
        .bright(),
        success_suffix: style("".to_string()).for_stderr().black().bright(),
        ..get_questions_theme()
    };
    let selections = block_device_list
        .iter()
        .map(|block_device| {
            format!(
                "{name:<9} {model:<24} {size:<8} {serial:<32}",
                name = block_device.name.to_string(),
                model = block_device.model.to_string(),
                size = block_device.size.to_string(),
                serial = block_device.serial.to_string()
            )
        })
        .collect::<Vec<_>>();

    let prompt_table_header = style(format!(
        "{name:<9} {model:<24} {size:<8} {serial:<32}",
        name = "Name",
        model = "Model",
        size = "Size",
        serial = "Serial"
    ))
    .yellow();

    let prompt = format!("Select disk to install\n  {}\n", prompt_table_header);

    let selection = Select::with_theme(&theme)
        .with_prompt(prompt)
        .default(0)
        .items(&selections[..])
        .interact()
        .unwrap();

    disks::BlockDevice {
        name: (&block_device_list[selection].name).to_string(),
        path: (&block_device_list[selection].path).to_string(),
        model: (&block_device_list[selection].model).to_string(),
        serial: (&block_device_list[selection].serial).to_string(),
        size: (&block_device_list[selection].size).to_string(),
        hotplug: *(&block_device_list[selection].hotplug),
    }
}
