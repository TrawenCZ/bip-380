use std::io::{stdin, BufRead, BufReader};

use crate::{
    structs::{
        derive_key_config::DeriveKeyConfig, key_expression_config::KeyExpressionConfig,
        parsing_error::ParsingError, script_expression_config::ScriptExpressionConfig,
    },
    traits::parsable::Parsable,
    utils::error_messages::{MISSING_ARG_ERR_MSG, MISSING_INPUT_ERR_MSG},
    FAILURE,
};

#[derive(Debug, PartialEq, Eq)]
pub enum Command {
    Help,
    DeriveKey(DeriveKeyConfig),
    KeyExpression(KeyExpressionConfig),
    ScriptExpression(ScriptExpressionConfig),
}

pub type Inputs = Box<dyn Iterator<Item = String>>;

/// Get the inputs for the sub-command
/// The inputs are read from stdin if the '-' argument is present in args
/// Otherwise, the argument right after the sub-command is the input
/// Only the argument immediately following the sub-command is used as input.
/// Additional flags or arguments (e.g., --foo) are not considered.
fn get_inputs(args: &Vec<&str>) -> Result<Inputs, ParsingError> {
    // if '-' is present in args, we should read from stdin
    match args.contains(&"-") {
        true => Ok(Box::new(
            BufReader::new(stdin())
                .lines()
                .map(|line| {
                    line.unwrap_or_else(|e| {
                        eprintln!("Error reading from stdin: {}", e);
                        std::process::exit(FAILURE);
                    })
                })
                .filter(|line| !line.is_empty()),
        )),
        false => {
            let mut inputs_peekable = args.iter().skip(1).peekable();
            match inputs_peekable.peek() {
                None => Err(ParsingError::new(MISSING_INPUT_ERR_MSG)),
                Some(_) => Ok(Box::new(
                    inputs_peekable
                        .map(|s| s.to_string())
                        .collect::<Vec<String>>()
                        .into_iter(),
                )),
            }
        }
    }
}

/// Parse the given arguments into a command and inputs.
pub fn parse_args(mut args: Vec<&str>) -> Result<(Command, Inputs), ParsingError> {
    // if args includes --help, we should print the help message
    if args.contains(&"--help") {
        return Ok((Command::Help, Box::new(std::iter::empty::<String>())));
    }

    // if --help is not present, then exacly one of the three sub-commands must be present and must be the first one argument
    let first_arg = args
        .first()
        .ok_or_else(|| ParsingError::new(MISSING_ARG_ERR_MSG))?;

    let command = match *first_arg {
        "derive-key" => Command::DeriveKey(DeriveKeyConfig::parse(&mut args)?),
        "key-expression" => Command::KeyExpression(KeyExpressionConfig::parse(&mut args)?),
        "script-expression" => Command::ScriptExpression(ScriptExpressionConfig::parse(&mut args)?),
        _ => {
            return Err(ParsingError::new(&format!(
                "Invalid argument: {}",
                first_arg
            )))
        }
    };

    let inputs = get_inputs(&args)?;

    Ok((command, inputs))
}

mod tests {
    #[allow(unused_imports)]
    use super::*;

    #[test]
    fn test_help() {
        let help_command_args = [
            vec!["--help"],
            vec!["--help", "derive-key"],
            vec!["--help", "key-expression"],
            vec!["--help", "script-expression"],
            vec!["derive-key", "--help"],
            vec!["key-expression", "--help"],
            vec!["script-expression", "--help"],
        ];

        for arg in help_command_args.iter() {
            assert!(matches!(parse_args(arg.to_vec()), Ok((Command::Help, _))));
        }
    }

    #[test]
    fn test_parse_args_command_output() {
        assert!(matches!(
            parse_args(vec!["key-expression", "arg1"]),
            Ok((Command::KeyExpression(_), _))
        ));

        assert!(matches!(
            parse_args(vec!["script-expression", "arg2"]),
            Ok((Command::ScriptExpression(_), _))
        ));

        assert!(matches!(
            parse_args(vec!["derive-key", "arg3"]),
            Ok((Command::DeriveKey(_), _))
        ));
    }

    #[test]
    fn test_parse_args_invalid_input() {
        assert!(parse_args(vec!["invalid"]).is_err());

        assert!(parse_args(vec![]).is_err());
    }

    #[test]
    fn test_parse_args_flag_dropping() {
        let example_arg_set = vec!["derive-key", "--path", "100/200h", "argument"];

        let result = parse_args(example_arg_set);

        assert!(result.is_ok());

        let inputs: Vec<String> = result.unwrap().1.collect();

        assert_eq!(inputs, vec!["argument"]);
    }

    #[test]
    fn test_inputs() {
        let inputs = get_inputs(&vec!["key-expression", "input"]).unwrap();
        assert_eq!(inputs.collect::<Vec<String>>(), vec!["input"]);

        assert!(get_inputs(&vec!["key-expression", "-"]).is_ok());

        assert!(get_inputs(&vec!["key-expression"]).is_err());

        assert!(get_inputs(&vec!["key-expression", "input1", "input2"]).is_ok());
    }
}
