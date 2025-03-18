use crate::traits::parsable::Parsable;

#[derive(Debug, PartialEq, Eq)]
pub struct KeyExpressionInput {}

impl Parsable for KeyExpressionInput {
    fn parse(_args: Vec<&str>) -> Result<Self, String> {
        Ok(KeyExpressionInput {})
    }
}
