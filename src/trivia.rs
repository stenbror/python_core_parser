
use std::fmt::{self, Write};

pub use crate::location::Location;

#[derive(Clone, Debug, PartialEq)]
pub struct TriviaBase<T> {
    pub location: Location,
    pub node: T
}

impl<T> TriviaBase<T> {
    pub fn new(location: Location, node: T) -> Self {
        TriviaBase {
            location,
            node
        }
    }
}

pub enum TriviaKind {
    WhiteSpace,
    Newline,
    Comment
}

pub type Trivia = TriviaBase<TriviaKind>;

impl fmt::Display for Trivia {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use TriviaKind::*;
        match &self.node {
            WhiteSpace => f.write_str("' '"),
            Newline => f.write_str("<NEWLINE>"),
            Comment => f.write_str("<COMMENT>")
        }
    }
}
