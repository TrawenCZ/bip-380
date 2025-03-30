use crate::structs::parsing_error::ParsingError;

/// A trait for parsing command line arguments into a command struct.
#[allow(unused)]
pub trait Parsable {
    /// Parse the given arguments into a struct.
    ///
    /// # Arguments
    ///
    /// * `args` - The arguments to parse.
    ///
    /// # Returns
    ///
    /// A struct with the parsed arguments.
    fn parse(args: &mut Vec<&str>) -> Result<Self, ParsingError>
    where
        Self: Sized;
}
