use crate::{
    structs::{
        derive_key_input::DeriveKeyInput, key_expression_input::KeyExpressionInput,
        parsing_error::ParsingError, script_expression_input::ScriptExpressionInput,
    },
    traits::parsable::Parsable,
};

#[derive(Debug, PartialEq, Eq)]
pub enum Command {
    Help,
    DeriveKey(DeriveKeyInput),
    KeyExpression(KeyExpressionInput),
    ScriptExpression(ScriptExpressionInput),
}

pub fn parse_args(args: Vec<&str>) -> Result<Command, ParsingError> {
    // if args includes --help, we should print the help message
    if args.contains(&"--help") {
        return Ok(Command::Help);
    }

    // if --help is not present, then exacly one of the three sub-commands must be present and must be the first one
    match args.first() {
        Some(arg) => match *arg {
            "derive-key" => Ok(Command::DeriveKey(DeriveKeyInput::parse(args)?)),
            "key-expression" => Ok(Command::KeyExpression(KeyExpressionInput::parse(args)?)),
            "script-expression" => Ok(Command::ScriptExpression(ScriptExpressionInput::parse(
                args,
            )?)),
            _ => Err(ParsingError::new(&format!("Invalid argument: {}", arg))),
        },
        None => Err(ParsingError::new("No valid argument was provided.")),
    }
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
            assert!(matches!(parse_args(arg.to_vec()), Ok(Command::Help)));
        }
    }

    #[test]
    fn test_parse_args() {
        assert!(matches!(
            parse_args(vec!["key-expression"]),
            Ok(Command::KeyExpression(_))
        ));

        assert!(matches!(
            parse_args(vec!["script-expression"]),
            Ok(Command::ScriptExpression(_))
        ));

        assert!(matches!(
            parse_args(vec!["derive-key"]),
            Ok(Command::DeriveKey(_))
        ));

        assert!(parse_args(vec!["invalid"]).is_err());

        assert!(parse_args(vec![]).is_err());
    }
}
