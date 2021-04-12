use crate::setup::facts::disks;
use dialoguer::{theme::ColorfulTheme, Select};

pub fn select_disk(block_device_list: &Vec<disks::BlockDevice>) {
    let selections = block_device_list
        .iter()
        .map(|block_device| {
            format!("{path:<20} test", path=block_device.path.to_string())
        })
        .collect::<Vec<_>>();

    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Pick your flavor")
        .default(0)
        .items(&selections[..])
        .interact()
        .unwrap();

    println!("Enjoy your {:?}!", selection);
}
