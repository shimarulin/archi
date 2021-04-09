pub fn get_firmware_interface() -> String {
    if std::path::Path::new("/sys/firmware/efi").exists() {
        String::from("UEFI")
    } else {
        String::from("BIOS")
    }
}
