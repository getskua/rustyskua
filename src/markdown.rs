use comrak::{ComrakOptions, markdown_to_html};

use super::Convert;
use super::html::HTMLPage;

pub struct MarkdownPage {
    content: String,
    location: String,
}

impl Convert<HTMLPage> for MarkdownPage {
    fn convert(&self) -> HTMLPage {
        HTMLPage {
            body: markdown_to_html(&self.content, &ComrakOptions::default())
        }
    }
}
