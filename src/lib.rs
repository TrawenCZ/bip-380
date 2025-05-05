use parsers::arg_parser::{self, Command};
use subcommands::derive_key::derive_key;
use subcommands::key_expression::key_expression;
use subcommands::script_expression::script_expression;
use utils::info_messages::HELP_MESSAGE;

mod parsers;
mod structs;
mod subcommands;
mod test_utils;
mod traits;
mod utils;

/// Exit codes
pub const SUCCESS: i32 = 0;
pub const FAILURE: i32 = 1;

/// Parses the command-line arguments and runs the logic accordingly.
///
/// # Arguments
///
/// * `args` - A collection of command-line arguments to be parsed.
///
/// # Returns
///
/// * On success, returns a unit type (nothing).
/// * On failure, prints the error message to standard error and returns a failure code.
///
/// # Errors
///
/// This function propagates any errors returned by `arg_parser::parse_args` or by subcommands and maps them
/// to a failure return code.
pub fn run_cli(args: Vec<&str>) -> Result<(), i32> {
    let (command, inputs) = arg_parser::parse_args(args).map_err(|err| {
        eprintln!("{err}");
        FAILURE
    })?;

    match command {
        Command::KeyExpression(config) => {
            for input in inputs {
                match key_expression(input, &config) {
                    Ok(result) => println!("{result}"),
                    Err(err) => {
                        eprintln!("{err}");
                        return Err(FAILURE);
                    }
                }
            }
        }
        Command::ScriptExpression(config) => {
            for input in inputs {
                match script_expression(&input, &config) {
                    Ok(result) => println!("{result}"),
                    Err(err) => {
                        eprintln!("{err}");
                        return Err(FAILURE);
                    }
                }
            }
        }
        Command::DeriveKey(config) => {
            for input in inputs {
                match derive_key(&input, &config) {
                    Ok(result) => println!("{result}"),
                    Err(err) => {
                        eprintln!("{err}");
                        return Err(FAILURE);
                    }
                }
            }
        }
        Command::Help => {
            println!("{HELP_MESSAGE}");
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {

    use std::vec;

    use super::*;
    use crate::test_utils::get_cmd;

    #[test]
    fn test_help() {
        let expected_help_message = format!("{HELP_MESSAGE}\n");
        get_cmd()
            .arg("--help")
            .assert()
            .success()
            .stdout(expected_help_message.clone());

        get_cmd()
            .args(vec!["derive-key", "--help"])
            .assert()
            .success()
            .stdout(expected_help_message.clone());

        get_cmd()
            .args(vec!["--help", "derive-key"])
            .assert()
            .success()
            .stdout(expected_help_message.clone());

        get_cmd()
            .args(vec!["derive-key", "-", "--help"])
            .write_stdin("000102030405060708090a0b0c0d0e0f")
            .assert()
            .success()
            .stdout(expected_help_message);
    }
}
