use std::{error::Error, fmt::Display};

#[derive(Debug, Clone)]
pub struct InvalidOperationError {
    reason: &'static str,
}

impl InvalidOperationError {
    pub fn new(reason: &'static str) -> Self {
        Self { reason }
    }
}
impl Error for InvalidOperationError {}
impl Display for InvalidOperationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Invalid operation error: {}", self.reason)
    }
}
