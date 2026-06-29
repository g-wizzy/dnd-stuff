use std::{error::Error, fmt};

#[derive(Debug)]
pub struct ParseError(pub String);

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<&str> for ParseError {
    fn from(value: &str) -> Self {
        Self(value.to_owned())
    }
}

impl Error for ParseError {}
