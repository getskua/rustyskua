use std::fs::read_to_string;
use std::io::Read;

pub struct RawInput {
    pub content: String,
    pub load_location: String,
}

impl RawInput {
    pub fn new(file: String) -> Self {
        let contents = read_to_string(&file).expect("Error reading the file.");
        RawInput {
            content: contents,
            load_location: (&file).clone(),
        }
    }
}

pub struct RawOutput {
    pub content: String,
    pub save_location: String,
}

impl RawOutput {
    pub fn new(content: String, save_location: String) -> RawOutput {
        RawOutput {
            content,
            save_location,
        }
    }
}