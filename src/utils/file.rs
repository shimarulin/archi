use std::path::Path;
use std::fs::File;
use std::io::Write;

pub fn create(path: &str, content: &str) {
    let path_ = Path::new(path);
    let display = path_.display();

    let mut file = match File::create(&path_) {
        Err(why) => panic!("couldn't create {}: {}", display, why),
        Ok(file) => file,
    };

    match file.write_all(content.as_bytes()) {
        Err(why) => panic!("couldn't write to {}: {}", display, why),
        Ok(_) => println!("successfully wrote {}", display),
    };
}