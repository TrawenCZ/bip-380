use std::num::ParseIntError;

use crate::structs::parsing_error::ParsingError;

/// The function `decode_hex` takes a hexadecimal string as input and returns a Result containing a
/// vector of u8 bytes after decoding the hexadecimal string.
///
/// Arguments:
///
/// * `s`: The function `decode_hex` takes a hexadecimal string `s` as input and attempts to decode it
///   into a vector of bytes (`Vec<u8>`). Each pair of characters in the input string represents a byte in
///   hexadecimal format. The function processes the input string by converting each pair of characters
///   into a
///
/// Returns:
///
/// The `decode_hex` function is returning a `Result` containing a decoded number in `Vec<u8>` if conversion
/// was successful or a `ParseIntError`.
pub fn decode_hex(s: &str) -> Result<Vec<u8>, ParseIntError> {
    (0..s.len())
        .step_by(2)
        .map(|i| u8::from_str_radix(&s[i..i + 2], 16))
        .collect()
}

/// The function `assert_hexadecimal_format` checks if the input string is a valid hexadecimal string.
///
/// Arguments:
///
/// * `input`: A string slice that represents the input to be checked.
/// * `label`: A string slice that represents the label for the input (for the error message).
///
/// Returns:
///
/// A `Result` that indicates whether the input string is a valid hexadecimal string or not.
/// If the input string is valid, it returns `Ok(())`. If the input string is not valid, it returns an
/// `Err` containing a `ParsingError`.
pub fn assert_hexadecimal_format(input: &str, label: &str) -> Result<(), ParsingError> {
    let mut input_clone = input.to_string();
    input_clone.retain(|c| c != ' ');

    if input_clone.is_empty() || input_clone.chars().any(|c| !c.is_ascii_hexdigit()) {
        return Err(ParsingError::new(&format!(
            "{label} '{input}' is not a valid hexadecimal string!"
        )));
    }
    Ok(())
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_is_hexadecimal() {
        // ok
        assert!(assert_hexadecimal_format("1234", "argument").is_ok());
        assert!(assert_hexadecimal_format("  12 34  ", "argument").is_ok());
        assert!(assert_hexadecimal_format("  12 34 ff ", "argument").is_ok());
        assert!(assert_hexadecimal_format("deadbeef", "argument").is_ok());
        assert!(assert_hexadecimal_format(
            "00 01 02 03 04 05 06 07 08 09 0a 0b 0c 0d 0e 0f",
            "argument"
        )
        .is_ok());
        assert!(assert_hexadecimal_format(
            "0 0 0 1 0 2 0 3 0 4 0 5 0 6 0 7 0 8 0 9 0 A 0 B 0 C 0 D 0 E 0 F",
            "argument"
        )
        .is_ok());
        assert!(assert_hexadecimal_format("123", "argument").is_ok());
        assert!(assert_hexadecimal_format(" 1 ", "argument").is_ok());
        assert!(assert_hexadecimal_format("f", "argument").is_ok());

        // errors
        assert_eq!(
            assert_hexadecimal_format("123G", "argument")
                .unwrap_err()
                .to_string(),
            "Parsing error: argument '123G' is not a valid hexadecimal string!"
        );
        assert_eq!(
            assert_hexadecimal_format("", "argument")
                .unwrap_err()
                .to_string(),
            "Parsing error: argument '' is not a valid hexadecimal string!"
        );
        assert_eq!(
            assert_hexadecimal_format("  ", "argument")
                .unwrap_err()
                .to_string(),
            "Parsing error: argument '  ' is not a valid hexadecimal string!"
        );
        assert_eq!(
            assert_hexadecimal_format(
                "00\t01\t02\t03\t04\t05\t06\t07\t08\t09\t0a\t0b\t0c\t0d\t0e\t0f",
                "argument"
            )
            .unwrap_err().to_string(),
            "Parsing error: argument '00\t01\t02\t03\t04\t05\t06\t07\t08\t09\t0a\t0b\t0c\t0d\t0e\t0f' is not a valid hexadecimal string!"
        );
    }
}
