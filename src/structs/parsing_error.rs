use std::num::ParseIntError;

#[derive(Debug, Eq, PartialEq)]
pub struct ParsingError {
    pub message: String,
}

impl ParsingError {
    #[must_use]
    pub fn new(message: &str) -> ParsingError {
        ParsingError {
            message: message.to_string(),
        }
    }
}

impl std::fmt::Display for ParsingError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Parsing error: {}", self.message)
    }
}

impl From<bip32::Error> for ParsingError {
    fn from(value: bip32::Error) -> Self {
        ParsingError::new(value.to_string().as_str())
    }
}

impl From<ParseIntError> for ParsingError {
    fn from(value: ParseIntError) -> Self {
        ParsingError::new(value.to_string().as_str())
    }
}
