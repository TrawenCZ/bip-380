use crate::traits::parsable::Parsable;

#[derive(Debug, PartialEq, Eq)]
pub struct ScriptExpressionInput {}

impl Parsable for ScriptExpressionInput {
    fn parse(_args: Vec<&str>) -> Result<Self, String> {
        Ok(ScriptExpressionInput {})
    }
}
