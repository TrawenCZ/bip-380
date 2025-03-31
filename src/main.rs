use std::env;

use parsers::arg_parser::{self, Command};
use subcommands::derive_key::derive_key;
use subcommands::key_expression::key_expression;
use subcommands::script_expression::script_expression;
mod parsers;
mod structs;
mod subcommands;
mod traits;
mod utils;

// TODO: COMPLETE THE HELP MESSAGE
const HELP_MESSAGE: &str = r#"Help message"#;

// exit codes
const SUCCESS: i32 = 0;
const FAILURE: i32 = 1;

fn main() {
    // collect args from the CLI skipping the first argument, which is the name of the program
    let args: Vec<String> = env::args().skip(1).collect();

    // convert the Vec<String> to Vec<&str>
    let args: Vec<&str> = args.iter().map(|s| s.as_str()).collect();

    let (command, inputs) = arg_parser::parse_args(args).unwrap_or_else(|err| {
        eprintln!("{}", err);
        std::process::exit(FAILURE);
    });

    match command {
        Command::KeyExpression(config) => {
            for input in inputs {
                match key_expression(input, &config) {
                    Ok(result) => println!("{}", result),
                    Err(err) => {
                        eprintln!("{}", err);
                        std::process::exit(FAILURE);
                    }
                }
            }
        }
        Command::ScriptExpression(config) => {
            for input in inputs {
                match script_expression(input, &config) {
                    Ok(result) => println!("{}", result),
                    Err(err) => {
                        eprintln!("{}", err);
                        std::process::exit(FAILURE);
                    }
                }
            }
        }
        Command::DeriveKey(config) => {
            for input in inputs {
                match derive_key(input, &config) {
                    Ok(result) => println!("{}", result),
                    Err(err) => {
                        eprintln!("{}", err);
                        std::process::exit(FAILURE);
                    }
                }
            }
        }
        Command::Help => {
            println!("{}", HELP_MESSAGE);
        }
    }

    std::process::exit(SUCCESS);
}

#[cfg(test)]
mod tests {
    use super::*;
    use assert_cmd::Command;

    pub fn get_cmd() -> Command {
        Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap()
    }

    #[test]
    fn test_help() {
        let expected_help_message = "Help message\n";
        get_cmd()
            .arg("--help")
            .assert()
            .success()
            .stdout(expected_help_message);
    }
}
