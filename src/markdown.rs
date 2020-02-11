use super::raw::{RawInput, RawOutput};
use std::collections::HashMap;
use std::str::Chars;
use regex::Regex;
use crate::collection::Collection;
use std::thread;

pub struct Loc {
    line: u32,
    col: u32,
}

impl Loc {
    pub fn new(line: u32, col: u32) -> Self {
        Self {
            line,
            col,
        }
    }
}

impl Frontmatter {
    pub fn new(string: &mut String) -> Self {
        let mut loc = Loc::new(1, 0);
        let mut finished = false;
        while !finished {
            next = string.pop().unwrap();
            match next {
                '-' => {
                    let n_1 = string.pop().unwrap();
                    let n_2 = string.pop().unwrap();
                    if (n_1 != '-') | (n_2 != '-') {
                        assert!("")
                    }
                }
            }
        }
    }
}

fn parse(markdown_string: &str) -> () {
    let matches = FRONTMATTER_REGEX.find(markdown_string);
    if matches < 2 {
        panic!("The markdown file {} has no frontmatter.")
    }
}


pub struct MarkdownPage {
    pub content: String,
}