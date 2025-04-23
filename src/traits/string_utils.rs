use crate::{
    structs::parsing_error::ParsingError, utils::error_messages::script_arg_extraction_err,
};

const SPACE: char = ' ';

pub trait Trimifiable: Sized {
    type Output: Sized;

    /// Trims leading and trailing spaces (U+0020) from a character array.
    fn trimify(self) -> Self::Output;
}

impl<'a> Trimifiable for &'a str {
    type Output = &'a str;
    fn trimify(self) -> Self::Output {
        let first_non_space = self.find(|c: char| c != SPACE);
        let last_non_space = self.rfind(|c: char| c != SPACE);

        match (first_non_space, last_non_space) {
            (Some(start), Some(end)) => &self[start..=end],
            _ => "",
        }
    }
}

impl Trimifiable for &[char] {
    type Output = Vec<char>;
    fn trimify(self) -> Self::Output {
        let first_non_space = self.iter().position(|&c| c != SPACE);
        let last_non_space = self.iter().rposition(|&c| c != SPACE);

        match (first_non_space, last_non_space) {
            (Some(start), Some(end)) => self[start..=end].to_vec(),
            _ => vec![],
        }
    }
}

pub trait StringSliceUtils<'a>: Trimifiable<Output = &'a str> {
    fn charify(self) -> Vec<char>;
}

impl<'a> StringSliceUtils<'a> for &'a str {
    fn charify(self) -> Vec<char> {
        self.chars().collect()
    }
}

pub trait CharArrayUtils: Trimifiable<Output = Vec<char>> {
    fn stringify(self) -> String;

    /// Extracts function arguments from a string slice.
    ///
    /// # Arguments
    ///
    /// * `label` - The function's label to use for error messages.
    ///
    /// # Returns
    ///
    /// A vector of strings representing the extracted arguments. Each string is trimmed of leading and trailing spaces (via trimify method).
    fn extract_args(self, label: &str) -> Result<Vec<String>, ParsingError>;
}

impl CharArrayUtils for &[char] {
    fn stringify(self) -> String {
        self.iter().collect()
    }

    fn extract_args(self, label: &str) -> Result<Vec<String>, ParsingError> {
        match self.trimify().as_slice() {
            ['(', raw_inputs @ .., ')'] => match raw_inputs.trimify().as_slice() {
                inner_arg if inner_arg.contains(&'(') && matches!(inner_arg.last(), Some(')')) => {
                    Ok(vec![inner_arg.trimify().stringify()])
                }
                _ => Ok(raw_inputs
                    .stringify()
                    .split(',')
                    .map(|arg| arg.trimify().to_string())
                    .collect()),
            },
            _ => Err(ParsingError::new(&script_arg_extraction_err(label))),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_trimify_for_the_char_array() {
        let chars: &[char] = &[' ', ' ', ' ', 'H', 'e', 'l', 'l', 'o', ' ', ' ', ' '];
        assert_eq!(chars.trimify(), vec!['H', 'e', 'l', 'l', 'o']);
        let chars: &[char] = &['H', 'e', 'l', 'l', 'o'];
        assert_eq!(chars.trimify(), vec!['H', 'e', 'l', 'l', 'o']);
        let chars: &[char] = &[' ', ' ', ' '];
        assert_eq!(chars.trimify(), vec![]);
        let chars: &[char] = &[];
        assert_eq!(chars.trimify(), vec![]);
        let chars: &[char] = &[' ', 't', ' '];
        assert_eq!(chars.trimify(), vec!['t']);
        let chars: &[char] = &[' ', 'H', 'e', 'l', 'l', 'o'];
        assert_eq!(chars.trimify(), vec!['H', 'e', 'l', 'l', 'o']);
        let chars: &[char] = &['H', 'e', 'l', 'l', 'o', ' '];
        assert_eq!(chars.trimify(), vec!['H', 'e', 'l', 'l', 'o']);
        let chars: &[char] = &['H', 'e', 'l', ' ', 'l', 'o'];
        assert_eq!(chars.trimify(), vec!['H', 'e', 'l', ' ', 'l', 'o']);
        let chars: &[char] = &[' ', '\t', ' ', 'H', 'e', 'l', 'l', 'o', ' ', '\t', ' '];
        assert_eq!(
            chars.trimify(),
            vec!['\t', ' ', 'H', 'e', 'l', 'l', 'o', ' ', '\t']
        );
    }
    #[test]
    fn test_trimify_for_the_string_slice() {
        assert_eq!("   Hello   ".trimify(), "Hello");
        assert_eq!("Hello".trimify(), "Hello");
        assert_eq!("   ".trimify(), "");
        assert_eq!("".trimify(), "");
        assert_eq!(" t ".trimify(), "t");
        assert_eq!(" Hello".trimify(), "Hello");
        assert_eq!("Hello ".trimify(), "Hello");
        assert_eq!("Hel lo".trimify(), "Hel lo");
        assert_eq!(" \t Hello \t ".trimify(), "\t Hello \t");
    }
}
