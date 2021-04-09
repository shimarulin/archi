mod firmware_interface;

pub struct Facts {
    pub firmware: String
}

pub fn gathering_facts() -> Facts {
    Facts {
        firmware: firmware_interface::get_firmware_interface()
    }
}