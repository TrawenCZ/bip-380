use crate::traits::parsable::Parsable;

use super::parsing_error::ParsingError;

#[derive(Debug, PartialEq, Eq)]
pub struct ScriptExpressionInput {}

impl Parsable for ScriptExpressionInput {
    fn parse(_args: Vec<&str>) -> Result<Self, ParsingError> {
        Ok(ScriptExpressionInput {})
    }
}
