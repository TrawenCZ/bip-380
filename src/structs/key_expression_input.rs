use crate::traits::parsable::Parsable;

use super::parsing_error::ParsingError;

#[derive(Debug, PartialEq, Eq)]
pub struct KeyExpressionInput {}

impl Parsable for KeyExpressionInput {
    fn parse(_args: Vec<&str>) -> Result<Self, ParsingError> {
        Ok(KeyExpressionInput {})
    }
}
