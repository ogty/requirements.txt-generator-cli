use std::fs::File;
use std::io::{prelude::*, self};
use std::path::{Path, Display};


pub fn read(path: &str) -> String {
    let path: &Path = Path::new(&path);
    let display: Display = path.display();

    let mut file: File = match File::open(&path) {
        Err(why) => panic!("couldn't open {}: {}", display, why),
        Ok(file) => file,
    };

    let mut s: String = String::new();
    match file.read_to_string(&mut s) {
        Err(why) => panic!("couldn't read {}: {}", display, why),
        Ok(_) => s
    }
}


pub fn write(path: &str, content: String) {
    let path: &Path = Path::new(&path);
    let display: Display = path.display();

    let mut file: File = match File::create(&path) {
        Err(why) => panic!("couldn't create {}: {}", display, why),
        Ok(file) => file,
    };

    match file.write_all(content.as_bytes()) {
        Err(why) => panic!("couldn't write to {}: {}", display, why),
        Ok(_) => (),
    }
}


pub fn read_lines(path: &str) -> Vec<String> {
    let mut result: Vec<String> = Vec::new();
    let file: File = File::open(path).unwrap();
    let tmp = io::BufReader::new(file).lines();
    for line in tmp {
        result.push(line.unwrap());
    }
    return result;
}
