use std::convert::From;
use super::markdown::MarkdownPage;
use comrak::{markdown_to_html, ComrakOptions};

pub struct HTMLPage {
    content: String,
    save_location: String,
}

impl From<MarkdownPage> for HTMLPage {
    fn from(markdown: MarkdownPage) -> Self {
        let html = markdown_to_html(&markdown.content, &ComrakOptions);
        let save_location = match markdown.frontmatter.get("save_location") {
            Some(location) => String::from(location),
            None => {
                panic!("A markdown file is badly formatted.");
            }
        };
        return HTMLPage {
            content: html,
            save_location,
        };
    }
}