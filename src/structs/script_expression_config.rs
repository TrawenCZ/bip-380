use crate::traits::parsable::Parsable;

use super::parsing_error::ParsingError;

#[derive(Debug, PartialEq, Eq)]
pub struct ScriptExpressionConfig {}

impl Parsable for ScriptExpressionConfig {
    fn parse(_args: Vec<&str>) -> Result<Self, ParsingError> {
        Ok(ScriptExpressionConfig {})
    }
}
