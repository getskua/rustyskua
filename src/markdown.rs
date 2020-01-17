use super::raw::{RawInput, RawOutput};
use std::collections::HashMap;
use std::str::Chars;
use regex::Regex;
use crate::collection::Collection;
use std::thread;
use lazy_static::lazy_static;

lazy_static! {
    static ref FRONTMATTER_REGEX: Regex = Regex::new(r"[:;]").unwrap();
}


fn parse(markdown_string: &str) -> (HashMap<String, String>, String) {
    let split_file: Vec<&str> = markdown_string.split("---").collect();

    let markdown = split_file[3];
    let frontmatter_string = split_file[1];

    let mut frontmatter: HashMap<String, String> = HashMap::new();
    let frontmatter_keys: Vec<&str> = FRONTMATTER_REGEX.split(frontmatter_string).collect();
    for i in (1..frontmatter_keys.len()).filter(|x| x % 2 == 0) {
        frontmatter.insert(String::from(frontmatter_keys[i]), String::from(frontmatter_keys[i + 1]));
    };
    return (frontmatter, String::from(markdown));
}


pub struct MarkdownPage {
    pub content: String,
    pub frontmatter: HashMap<String, String>,
}

impl From<RawInput> for MarkdownPage {
    fn from(raw: RawInput) -> Self {
        let (frontmatter, markdown) = parse(&raw.content);
        return MarkdownPage {
            content: markdown,
            frontmatter,
        };
    }
}

impl Into<Collection<RawOutput>> for Collection<MarkdownPage> {
    fn into(self) -> Collection<RawOutput> {
        return Collection {
            members: vec![RawOutput {
                content: String::from("Not implemented."),
                save_location: String::from("index.html"),
            }]
        };
    }
}