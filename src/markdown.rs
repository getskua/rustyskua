use super::raw::Raw;
use std::collections::HashMap;
use comrak::markdown_to_html;
use std::str::Chars;
use regex::Regex;

const FRONTMATTER_REGEX: Regex = Regex::new(r"[:;]").unwrap();

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

impl From<Raw> for MarkdownPage {
    fn from(raw: Raw) -> Self {
        let (frontmatter, markdown) = parse(&raw.content);
        return MarkdownPage {
            content: markdown,
            frontmatter,
        };
    }
}