use std::process::Command;

// curl "http://worldtimeapi.org/api/ip"
// curl "https://ipapi.co/json/"
// curl "https://ipapi.co/timezone/"
pub fn get_timezone() -> String {
    let output = Command::new("curl")
        .arg("https://ipapi.co/timezone/")
        .output()
        .expect("ERR");

    if output.status.success() {
        String::from_utf8(output.stdout).unwrap()
    } else {
        String::from("UTC")
    }
}
