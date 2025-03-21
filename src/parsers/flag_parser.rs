use crate::structs::parsing_error::ParsingError;

pub trait FlagStringUtils {
    fn flagify(&self) -> String;
}

impl FlagStringUtils for str {
    fn flagify(&self) -> String {
        format!("--{self}")
    }
}

#[allow(unused)]
fn missing_follow_up_val_err(key: &str) -> ParsingError {
    ParsingError::new(format!("Missing follow-up value after flag '{key}'! ").as_str())
}

/// The function `parse_boolean_flag` checks if a boolean flag specified by `key` is present in the
/// given `args` array.
///
/// Arguments:
///
/// * `args`: The `args` parameter is a slice of string slices (`&[&str]`) representing the command
///   line arguments passed to a program or function.
/// * `key`: The `key` parameter is a string that represents the flag key that we want to parse as a
///   boolean value. It should **not contain** the leading dashes (`--`), only the name of the key itself.
///
/// Returns:
///
/// The function `parse_boolean_flag` returns a boolean value, which indicates whether the provided
/// `args` slice contains the flag generated from the `key` string.
///
/// # Examples
/// ```rs
/// let example_bool_arg_set = vec!["derive-key", "--example-bool-flag"];
/// assert!(parse_boolean_flag(&example_bool_arg_set, "example-bool-flag"));
/// ```
#[allow(unused)]
pub fn parse_boolean_flag(args: &[&str], key: &str) -> bool {
    let flag = key.flagify();
    args.contains(&flag.as_str())
}

/// The function `parse_value_flag` parses a value flag from a slice of string arguments based on a
/// specified key. Valid value flag needs value following the flag (e.g., "`--value-flag and_its_value`"")
///
/// Arguments:
///
/// * `args`: The `args` parameter is a slice of string slices (`&[&str]`), representing the command
///   line arguments passed to a program or function.
/// * `key`: The `key` parameter is a string that represents the flag key that we want to parse as a
///   boolean value. It should **not contain** the leading dashes (`--`), only the name of the key itself.
///
/// Returns:
///
/// The function `parse_value_flag` returns a `Result` containing an `Option<String>`. If
/// * the key is found along with value, an `Ok(Some(value))` is returned.
/// * the key is found but value is missing, an `Err(ParsingError(msg))` is returned.
/// * the key is missing, an `Ok(None)` is returned.
///
/// # Examples
/// ```rs
/// let value = String::from("some-value");
/// let example_value_arg_set = vec!["derive-key", "--example-value-flag", value.as_str()];
///
/// assert_eq!(
///    parse_value_flag(&example_value_arg_set, "example-value-flag"),
///    Ok(Some(value))
///);
/// ```
#[allow(unused)]
pub fn parse_value_flag(args: &[&str], key: &str) -> Result<Option<String>, ParsingError> {
    let flag = key.flagify();
    if args.last() == Some(&flag.as_str()) {
        return Err(missing_follow_up_val_err(flag.as_str()));
    }

    Ok(args.windows(2).find_map(|w| match w {
        [arg1, arg2] if *arg1 == flag => Some(String::from(*arg2)),
        _ => None,
    }))
}

mod tests {
    #[allow(unused_imports)]
    use super::*;

    #[test]
    fn test_present_bool_flag() {
        let flag_key = "example-bool-flag";
        let flag = flag_key.flagify();
        let example_arg_set = vec!["derive-key", flag.as_str()];

        assert!(parse_boolean_flag(&example_arg_set, flag_key));
    }

    #[test]
    fn test_missing_bool_flag() {
        let example_arg_set = vec!["derive-key", "some-other-arg", "--and-random-flag"];

        assert!(!parse_boolean_flag(
            &example_arg_set,
            "example-non-existent-flag"
        ));
    }

    #[test]
    fn test_valid_value_flag() {
        let flag_key = "example-value-flag";
        let flag = flag_key.flagify();
        let value = String::from("and-its-value");
        let example_arg_set = vec!["derive-key", flag.as_str(), value.as_str()];

        assert_eq!(
            parse_value_flag(&example_arg_set, flag_key),
            Ok(Some(value))
        );
    }

    #[test]
    fn test_missing_value_in_value_flag() {
        let flag_key = "example-value-flag";
        let flag = flag_key.flagify();
        let example_arg_set = vec!["derive-key", flag.as_str()];

        assert_eq!(
            parse_value_flag(&example_arg_set, flag_key),
            Err(missing_follow_up_val_err(flag_key.flagify().as_str()))
        );
    }

    #[test]
    fn test_missing_value_flag() {
        let example_arg_set = vec!["derive-key", "--some-other-flag"];

        assert_eq!(
            parse_value_flag(&example_arg_set, "example-value-flag"),
            Ok(None)
        );
    }
}
