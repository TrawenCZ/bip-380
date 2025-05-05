use crate::structs::parsing_error::ParsingError;

/// A trait for parsing command line arguments into a command struct.
pub trait Parsable {
    /// Parses command line arguments into a struct implementing this trait.
    ///
    /// # Arguments
    ///
    /// * `args` - A mutable reference to a vector of argument string slices to be parsed.
    ///
    /// # Errors
    ///
    /// Returns a [`ParsingError`] if the arguments are invalid or parsing fails.
    fn parse(args: &mut Vec<&str>) -> Result<Self, ParsingError>
    where
        Self: Sized;
}
