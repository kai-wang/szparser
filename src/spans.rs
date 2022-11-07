#[allow(unused_imports)]
use std::{error, fmt};

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Span(pub usize);

pub type Spanned<T> = (T, Span);

