use std::fmt::Error;

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
    fn parse(args: Vec<String>) -> Result<Self, Error>
    where
        Self: Sized;
}
