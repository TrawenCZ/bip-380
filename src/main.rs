use std::env;

use parsers::arg_parser::{self, Command};
use subcommands::derive_key::derive_key;
use subcommands::key_expression::key_expression;
use subcommands::script_expression::script_expression;
mod parsers;
mod structs;
mod subcommands;
mod traits;

fn main() {
    // collect args from the CLI skipping the first argument, which is the name of the program
    let args: Vec<String> = env::args().skip(1).collect();

    // convert the Vec<String> to Vec<&str>
    let args: Vec<&str> = args.iter().map(|s| s.as_str()).collect();

    let (command, inputs) = arg_parser::parse_args(args).unwrap_or_else(|err| {
        eprintln!("{}", err);
        std::process::exit(1);
    });

    match command {
        Command::KeyExpression(config) => {
            for input in inputs {
                key_expression(input, &config).unwrap_or_else(|err| {
                    eprintln!("{}", err);
                    std::process::exit(1);
                });
            }
        }
        Command::ScriptExpression(config) => {
            for input in inputs {
                script_expression(input, &config).unwrap_or_else(|err| {
                    eprintln!("{}", err);
                    std::process::exit(1);
                });
            }
        }
        Command::DeriveKey(config) => {
            for input in inputs {
                derive_key(input, &config).unwrap_or_else(|err| {
                    eprintln!("{}", err);
                    std::process::exit(1);
                });
            }
        }
        Command::Help => {
            println!("Help message");
        }
    }

    std::process::exit(0);
}

#[cfg(test)]
mod test {
    use super::*;
    use assert_cmd::Command;

    #[test]
    fn test_help() {
        let expected_help_message = "Help message\n";
        Command::cargo_bin(env!("CARGO_PKG_NAME"))
            .unwrap()
            .arg("--help")
            .assert()
            .success()
            .stdout(expected_help_message);
    }
}
