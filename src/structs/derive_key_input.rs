use crate::traits::parsable::Parsable;

#[derive(Debug, PartialEq, Eq)]
pub struct DeriveKeyInput {}

impl Parsable for DeriveKeyInput {
    fn parse(_args: Vec<&str>) -> Result<Self, String> {
        Ok(DeriveKeyInput {})
    }
}
