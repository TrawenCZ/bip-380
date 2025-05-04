use std::env;

use parsers::arg_parser::{self, Command};
use subcommands::derive_key::derive_key;
use subcommands::key_expression::key_expression;
use subcommands::script_expression::script_expression;
use utils::info_messages::HELP_MESSAGE;
mod parsers;
mod structs;
mod subcommands;
mod traits;
mod utils;

// exit codes
const SUCCESS: i32 = 0;
const FAILURE: i32 = 1;

fn main() {
    // collect args from the CLI skipping the first argument, which is the name of the program
    let args: Vec<String> = env::args().skip(1).collect();

    // convert the Vec<String> to Vec<&str>
    let args: Vec<&str> = args.iter().map(std::string::String::as_str).collect();

    let (command, inputs) = arg_parser::parse_args(args).unwrap_or_else(|err| {
        eprintln!("{err}");
        std::process::exit(FAILURE);
    });

    match command {
        Command::KeyExpression(config) => {
            for input in inputs {
                match key_expression(input, &config) {
                    Ok(result) => println!("{result}"),
                    Err(err) => {
                        eprintln!("{err}");
                        std::process::exit(FAILURE);
                    }
                }
            }
        }
        Command::ScriptExpression(config) => {
            for input in inputs {
                match script_expression(input, &config) {
                    Ok(result) => println!("{result}"),
                    Err(err) => {
                        eprintln!("{err}");
                        std::process::exit(FAILURE);
                    }
                }
            }
        }
        Command::DeriveKey(config) => {
            for input in inputs {
                match derive_key(input, &config) {
                    Ok(result) => println!("{result}"),
                    Err(err) => {
                        eprintln!("{err}");
                        std::process::exit(FAILURE);
                    }
                }
            }
        }
        Command::Help => {
            println!("{HELP_MESSAGE}");
        }
    }

    std::process::exit(SUCCESS);
}

#[cfg(test)]
mod tests {
    use std::vec;

    use super::*;
    use assert_cmd::Command;

    pub fn get_cmd() -> Command {
        Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap()
    }

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

    #[test]
    fn derive_key_invalid_seed_whitespaces_error() {
        get_cmd()
            .args(vec!["derive-key", "0	0 0 1 02030405060708090a0b0c0d0e0f"])
            .assert()
            .failure();
    }
}
