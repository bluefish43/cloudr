use std::{
    error::Error,
    fmt::Display
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NullPointerError(pub String);

impl Display for NullPointerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "NullPointerError: {}", self.0)
    }
}

impl Error for NullPointerError {}