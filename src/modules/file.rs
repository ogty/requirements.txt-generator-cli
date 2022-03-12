use std::fs::File;
use std::io::{self, prelude::*, BufReader, Lines};
use std::path::{Display, Path};

pub fn write(path: &str, content: String) {
    let path: &Path = Path::new(&path);
    let for_display: Display = path.display();

    let mut file: File = match File::create(&path) {
        Err(why) => panic!("couldn't create {}: {}", for_display, why),
        Ok(file) => file,
    };

    if let Err(why) = file.write_all(content.as_bytes()) {
        panic!("couldn't write to {}: {}", for_display, why)
    }
}

pub fn read_lines(path: &str) -> Vec<String> {
    let mut result: Vec<String> = Vec::new();
    let file_content: File = File::open(path).unwrap();
    let line_contents: Lines<BufReader<File>> = io::BufReader::new(file_content).lines();
    for line_content in line_contents {
        result.push(line_content.unwrap());
    }
    result
}
