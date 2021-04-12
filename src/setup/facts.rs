pub(crate) mod disks;
mod firmware_interface;

// #[derive(Clone)]
pub struct Facts {
    pub firmware: String,
    pub disks: Vec<disks::BlockDevice>,
}

pub fn gathering_facts() -> Facts {
    Facts {
        firmware: firmware_interface::get_firmware_interface(),
        disks: disks::get_disk_info(),
    }
}
