use std::fs::read_to_string;
use std::io::Read;

pub struct Raw {
    pub content: String
}

impl Raw {
    pub fn new(file: String) -> Self {
        let mut contents = read_to_string(file).expect("Error reading the file.");
        Raw {
            content: contents
        }
    }
}