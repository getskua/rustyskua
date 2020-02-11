use std::convert::From;
use super::markdown::MarkdownPage;
use comrak::{markdown_to_html, ComrakOptions};

pub struct HTMLPage {
    content: String,
    save_location: String,
}