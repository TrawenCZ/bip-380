#[derive(Debug)]
pub struct ParsingError {
    pub message: String,
}

impl ParsingError {
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
