use crate::traits::parsable::Parsable;

use super::parsing_error::ParsingError;

#[derive(Debug, PartialEq, Eq)]
pub struct DeriveKeyConfig {}

impl Parsable for DeriveKeyConfig {
    fn parse(_args: Vec<&str>) -> Result<Self, ParsingError> {
        Ok(DeriveKeyConfig {})
    }
}
