use crate::traits::parsable::Parsable;

use super::parsing_error::ParsingError;

#[derive(Debug, PartialEq, Eq)]
pub struct KeyExpressionConfig {}

impl Parsable for KeyExpressionConfig {
    fn parse(_args: Vec<&str>) -> Result<Self, ParsingError> {
        Ok(KeyExpressionConfig {})
    }
}
