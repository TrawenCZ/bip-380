use crate::traits::parsable::Parsable;

use super::parsing_error::ParsingError;

#[derive(Debug, PartialEq, Eq)]
pub struct DeriveKeyInput {}

impl Parsable for DeriveKeyInput {
    fn parse(_args: Vec<&str>) -> Result<Self, ParsingError> {
        Ok(DeriveKeyInput {})
    }
}
