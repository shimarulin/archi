use crate::setup::facts::disks;
use dialoguer::{theme::ColorfulTheme, Select};
use dialoguer::console::style;

pub fn select_disk(block_device_list: &Vec<disks::BlockDevice>) -> String {
    let theme = ColorfulTheme {
        success_suffix: style("".to_string()).for_stderr().black().bright(),
        ..ColorfulTheme::default()
    };
    let selections = block_device_list
        .iter()
        .map(|block_device| {
            format!(
                "{name:<9} {model:<20} {size:<8} {serial:<18}",
                name = block_device.name.to_string(),
                model = block_device.model.to_string(),
                size = block_device.size.to_string(),
                serial = block_device.serial.to_string()
            )
        })
        .collect::<Vec<_>>();

    let prompt_table_header = style(format!(
                "{name:<9} {model:<20} {size:<8} {serial:<18}",
                name = "Name",
                model = "Model",
                size = "Size",
                serial = "Serial"
            )).yellow();

    let prompt = format!(
                "Select disk to install\n  {}\n",
                prompt_table_header
            );

    let selection = Select::with_theme(&theme)
        .with_prompt(prompt)
        .default(0)
        .items(&selections[..])
        .interact()
        .unwrap();

    (&block_device_list[selection].path).to_string()
}
