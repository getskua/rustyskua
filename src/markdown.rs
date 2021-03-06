use super::raw::RawInput;
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

pub struct Frontmatter {
    values: HashMap<String, String>
}

impl Frontmatter {
    pub fn new(string: &mut String) -> Self {
        let mut frontmatter = Frontmatter {
            values: HashMap::new()
        };
        let mut loc = Loc::new(1, 0);
        let mut finished = false;
        while !finished {
            let next = string.pop().unwrap();
            match next {
                '-' => {
                    let n_1 = string.pop().unwrap();
                    let n_2 = string.pop().unwrap();
                    if (n_1 != '-') | (n_2 != '-') {
                        panic!("The '-' on line {}, column {} is invalid.", loc.line, loc.col);
                    }
                }
                'a'..='z' | 'A'..='Z' => {
                    let mut colon = false;
                    let mut key = String::new();
                    let mut value = String::new();
                    while !colon {
                        let n_char = string.pop().unwrap();
                        match n_char {
                            'a'..='z' | 'A'..='Z' | '_' => {
                                key.push(n_char);
                            }
                            ':' => {
                                colon = true;
                            }
                            _ => {
                                panic!("Invalid token on line {}, column {}.", loc.line, loc.col);
                            }
                        }
                    }
                    let mut eol = false;
                    while !eol {
                        let n_char = string.pop().unwrap();
                        match n_char {
                            'a'..='z' | 'A'..='Z' | '_' => {
                                key.push(n_char);
                            }
                            '\n' => {
                                eol = true;
                            }
                            _ => {
                                panic!("Invalid token on line {}, column {}.", loc.line, loc.col);
                            }
                        }
                    }
                    frontmatter.values.insert(key, value);
                }
                _ => {
                    panic!("Unexpected token on line {}, column {}.", loc.line, loc.col)
                }
            };
        };
        frontmatter
    }
}

pub struct MarkdownPage {
    pub frontmatter: Frontmatter,
    pub content: String,
}