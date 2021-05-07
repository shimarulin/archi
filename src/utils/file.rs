use std::path::Path;
use std::fs::File;
use std::io::{Write, BufReader, Read};

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

pub fn replace_string(path: &str, from: &str, to: &str) {
    let path_ = Path::new(path);
    let display = path_.display();

    let mut file = match File::open(&path_) {
        Err(why) => panic!("couldn't open {}: {}", display, why),
        Ok(file) => file,
    };
    let mut buf_reader = BufReader::new(&file);
    let mut source_string = String::new();

    match buf_reader.read_to_string(&mut source_string) {
        Err(why) => panic!("couldn't read to buffer from file {}: {}", display, why),
        Ok(_) => {},
    };

    let target_string = source_string.replace(&*from, &*to);

    match file.write_all(target_string.as_bytes()) {
        Err(why) => panic!("couldn't write to {}: {}", display, why),
        Ok(_) => println!("successfully wrote {}", display),
    };
}

pub fn append(path: &str, content: &str) {
    let path_ = Path::new(path);
    let display = path_.display();

    let mut file = match File::open(&path_) {
        Err(why) => panic!("couldn't open {}: {}", display, why),
        Ok(file) => file,
    };
    let mut buf_reader = BufReader::new(&file);
    let mut source_string = String::new();

    match buf_reader.read_to_string(&mut source_string) {
        Err(why) => panic!("couldn't read to buffer from file {}: {}", display, why),
        Ok(_) => {},
    };

    let target_string = format!("{}{}", source_string, content);

    match file.write_all(target_string.as_bytes()) {
        Err(why) => panic!("couldn't write to {}: {}", display, why),
        Ok(_) => println!("successfully wrote {}", display),
    };
}
