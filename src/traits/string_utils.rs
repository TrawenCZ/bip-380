use crate::{
    structs::parsing_error::ParsingError, utils::error_messages::script_arg_extraction_err,
};

pub trait StringUtils {
    fn charify(self) -> Vec<char>;
}

impl StringUtils for &str {
    fn charify(self) -> Vec<char> {
        self.chars().collect()
    }
}

pub trait CharArrayUtils {
    fn stringify(self) -> String;

    /// Trims leading and trailing spaces (U+0020) from a character array.
    fn trimify(self) -> Vec<char>;

    fn extract_args(self, label: &str) -> Result<Vec<String>, ParsingError>;
}

impl CharArrayUtils for &[char] {
    fn stringify(self) -> String {
        self.iter().collect()
    }

    fn trimify(self) -> Vec<char> {
        let first_non_space = self.iter().position(|&c| c != ' ');
        let last_non_space = self.iter().rposition(|&c| c != ' ');

        match (first_non_space, last_non_space) {
            (Some(start), Some(end)) => self[start..=end].to_vec(),
            _ => vec![],
        }
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
                    .map(|arg| arg.trim().to_string())
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
    fn test_trimify() {
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
}
