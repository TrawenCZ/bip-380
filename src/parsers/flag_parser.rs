use crate::{
    structs::parsing_error::ParsingError,
    utils::error_messages::{missing_follow_up_val_err, multiple_value_flags_detected_err},
};

trait FlagStringUtils {
    fn flagify(&self) -> String;
}

impl FlagStringUtils for str {
    fn flagify(&self) -> String {
        format!("--{self}")
    }
}

/// Parses a boolean flag from the provided arguments, removing all occurrences of the flag.
///
/// # Arguments
///
/// * `args` - A mutable reference to a vector of argument string slices.
/// * `key` - The flag key (without leading dashes) to search for.
///
/// # Returns
///
/// Returns `true` if the flag was present (and removed), otherwise `false`.
pub fn parse_boolean_flag(args: &mut Vec<&str>, key: &str) -> bool {
    let flag = key.flagify();
    let arg_count_on_entry = args.len();
    args.retain(|arg| *arg != flag);
    let arg_count_on_leave = args.len();
    arg_count_on_entry != arg_count_on_leave
}

/// Parses a value flag from the provided arguments, removing the flag and its value if present.
///
/// # Arguments
///
/// * `args` - A mutable reference to a vector of argument string slices.
/// * `key` - The flag key (without leading dashes) to search for.
///
/// # Returns
///
/// Returns `Ok(Some(value))` if the flag and its value are found and removed,
/// `Ok(None)` if the flag is not present,
/// or `Err(ParsingError)` if the flag is present but the value is missing or duplicated.
///
/// # Errors
///
/// Returns a [`ParsingError`] if:
/// - The flag is present but not followed by a value,
/// - The flag appears multiple times with values.
pub fn parse_value_flag(args: &mut Vec<&str>, key: &str) -> Result<Option<String>, ParsingError> {
    let flag = key.flagify();
    if args.last() == Some(&flag.as_str()) {
        return Err(ParsingError::new(&missing_follow_up_val_err(&flag)));
    }

    match args.windows(2).enumerate().find_map(|(index, w)| match w {
        [argument_1, argument_2] if *argument_1 == flag => Some((index, String::from(*argument_2))),
        _ => None,
    }) {
        Some((flag_index, _)) if args[(flag_index + 2)..args.len()].contains(&flag.as_str()) => {
            Err(ParsingError::new(&multiple_value_flags_detected_err(&flag)))
        }
        Some((flag_index, flag_value)) => {
            let mut index_counter: usize = 0;
            args.retain(|_| {
                let should_remove = (flag_index..=flag_index + 1).contains(&index_counter);
                index_counter += 1;
                !should_remove
            });
            Ok(Some(flag_value))
        }
        None => Ok(None),
    }
}

mod tests {
    #[allow(unused_imports)]
    use super::*;

    #[test]
    fn test_present_bool_flag() {
        let flag_key = "example-bool-flag";
        let flag = flag_key.flagify();
        let mut example_arg_set = vec!["derive-key", flag.as_str()];

        assert!(parse_boolean_flag(&mut example_arg_set, flag_key));

        assert_eq!(example_arg_set, vec!["derive-key"])
    }

    #[test]
    fn test_missing_bool_flag() {
        let example_arg_set = vec!["derive-key", "some-other-arg", "--and-random-flag"];
        let mut example_arg_set_cloned = example_arg_set.clone();

        assert!(!parse_boolean_flag(
            &mut example_arg_set_cloned,
            "example-non-existent-flag"
        ));

        assert_eq!(example_arg_set, example_arg_set_cloned)
    }

    #[test]
    fn test_multiple_same_bool_flags() {
        let flag_key = "example-bool-flag";
        let flag = flag_key.flagify();
        let mut example_arg_set = vec![
            "derive-key",
            flag.as_str(),
            flag.as_str(),
            "some-value",
            flag.as_str(),
            "--some-other-flag",
        ];

        assert!(parse_boolean_flag(&mut example_arg_set, flag_key));

        assert_eq!(
            example_arg_set,
            vec!["derive-key", "some-value", "--some-other-flag"]
        )
    }

    #[test]
    fn test_valid_value_flag() {
        let flag_key = "example-value-flag";
        let flag = flag_key.flagify();
        let value = String::from("and-its-value");
        let mut example_arg_set = vec!["derive-key", flag.as_str(), value.as_str()];

        assert_eq!(
            parse_value_flag(&mut example_arg_set, flag_key),
            Ok(Some(value.clone()))
        );

        assert_eq!(example_arg_set, vec!["derive-key"])
    }

    #[test]
    fn test_missing_value_in_value_flag() {
        let flag_key = "example-value-flag";
        let flag = flag_key.flagify();
        let mut example_arg_set = vec!["derive-key", flag.as_str()];

        assert_eq!(
            parse_value_flag(&mut example_arg_set, flag_key),
            Err(ParsingError::new(&missing_follow_up_val_err(
                &flag_key.flagify()
            )))
        );
    }

    #[test]
    fn test_duplicit_value_flag() {
        let flag_key = "example-value-flag";
        let flag = flag_key.flagify();
        let value = String::from("and-its-value");
        let mut example_arg_set = vec![
            "derive-key",
            flag.as_str(),
            value.as_str(),
            flag.as_str(),
            value.as_str(),
        ];

        assert_eq!(
            parse_value_flag(&mut example_arg_set, flag_key),
            Err(ParsingError::new(&multiple_value_flags_detected_err(&flag)))
        );
    }

    #[test]
    fn test_missing_value_flag() {
        let mut example_arg_set = vec!["derive-key", "--some-other-flag"];

        assert_eq!(
            parse_value_flag(&mut example_arg_set, "example-value-flag"),
            Ok(None)
        );
    }
}
