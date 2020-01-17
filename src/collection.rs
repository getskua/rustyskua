use std::convert::Into;
use crate::raw::RawOutput;
use crate::markdown::MarkdownPage;

pub struct Collection<T> {
    pub members: Vec<T>,
}

