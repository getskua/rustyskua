use std::convert::Into;
use crate::markdown::MarkdownPage;

pub struct Collection<T> {
    pub members: Vec<T>,
}

