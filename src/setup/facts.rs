pub(crate) mod disks;
mod firmware_interface;
mod timezone;

pub struct Facts {
    pub firmware: String,
    pub disks: Vec<disks::BlockDevice>,
    pub timezone: String,
}

pub fn gathering_facts() -> Facts {
    Facts {
        firmware: firmware_interface::get_firmware_interface(),
        disks: disks::get_disk_info(),
        timezone: timezone::get_timezone(),
    }
}
