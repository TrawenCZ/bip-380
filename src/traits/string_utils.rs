use crate::{
    structs::parsing_error::ParsingError, utils::error_messages::script_arg_extraction_err,
};

pub trait CharArrayUtils {
    fn stringify(self) -> String;

    fn trimify(self) -> Vec<char>;

    fn extract_args(self, label: &str) -> Result<Vec<String>, ParsingError>;
}

impl CharArrayUtils for &[char] {
    fn stringify(self) -> String {
        self.iter().collect()
    }

    fn trimify(self) -> Vec<char> {
        self.stringify().trim().chars().collect()
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
