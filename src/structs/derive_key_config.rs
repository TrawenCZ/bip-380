use bip32::DerivationPath;

use crate::{parsers::flag_parser::parse_value_flag, traits::parsable::Parsable};

use super::parsing_error::ParsingError;

#[derive(Debug, PartialEq, Eq, Default)]
pub struct DeriveKeyConfig {
    pub path: DerivationPath,
}

impl Parsable for DeriveKeyConfig {
    fn parse(args: &mut Vec<&str>) -> Result<Self, ParsingError> {
        let path = parse_value_flag(args, "path")?
            .map(|mut raw_path| {
                match raw_path.chars().nth(0) {
                    Some('/') => raw_path.insert(0, 'm'),
                    _ => raw_path.insert_str(0, "m/"),
                };

                raw_path
                    .to_lowercase()
                    .parse::<DerivationPath>()
                    .map_err(|err| ParsingError {
                        message: err.to_string(),
                    })
            })
            .transpose()?
            .unwrap_or("m".parse()?);
        Ok(DeriveKeyConfig { path })
    }
}

mod tests {
    #[allow(unused_imports)]
    use bip32::DerivationPath;

    #[allow(unused_imports)]
    use crate::{
        structs::{derive_key_config::DeriveKeyConfig, parsing_error::ParsingError},
        traits::parsable::Parsable,
    };

    #[test]
    fn test_valid_path_with_leading_slash() {
        let path = "/300/500h/100'/200H";
        let mut args = vec!["derive-key", "--path", path];

        let parsed_path = format!("m{}", path.to_lowercase())
            .parse::<DerivationPath>()
            .expect("Valid path should be parsed.");

        assert_eq!(
            DeriveKeyConfig::parse(&mut args),
            Ok(DeriveKeyConfig { path: parsed_path })
        )
    }

    #[test]
    fn test_valid_path_without_leading_slash() {
        let path = "300/500h/100'/200H";
        let mut args = vec!["derive-key", "--path", path];

        let parsed_path = format!("m/{}", path.to_lowercase())
            .parse::<DerivationPath>()
            .expect("Valid path should be parsed.");

        assert_eq!(
            DeriveKeyConfig::parse(&mut args),
            Ok(DeriveKeyConfig { path: parsed_path })
        )
    }

    #[test]
    fn test_invalid_path_slash_only() {
        let path = "/";
        let mut args = vec!["derive-key", "--path", path];

        let path_parse_error = format!("m{path}")
            .parse::<DerivationPath>()
            .expect_err("Invalid path should not be parsed.");

        assert_eq!(
            DeriveKeyConfig::parse(&mut args),
            Err(ParsingError {
                message: path_parse_error.to_string()
            })
        )
    }

    #[test]
    fn test_invalid_path_trailing_slash() {
        let path = "1/";
        let mut args = vec!["derive-key", "--path", path];

        let path_parse_error = format!("m/{path}")
            .parse::<DerivationPath>()
            .expect_err("Invalid path should not be parsed.");

        assert_eq!(
            DeriveKeyConfig::parse(&mut args),
            Err(ParsingError {
                message: path_parse_error.to_string()
            })
        )
    }

    #[test]
    fn test_invalid_path_letter_only() {
        let path = "a";
        let mut args = vec!["derive-key", "--path", path];

        let path_parse_error = format!("m/{path}")
            .parse::<DerivationPath>()
            .expect_err("Invalid path should not be parsed.");

        assert_eq!(
            DeriveKeyConfig::parse(&mut args),
            Err(ParsingError {
                message: path_parse_error.to_string()
            })
        )
    }

    #[test]
    fn test_invalid_path_double_slash() {
        let path = "//";
        let mut args = vec!["derive-key", "--path", path];

        let path_parse_error = format!("m{path}")
            .parse::<DerivationPath>()
            .expect_err("Invalid path should not be parsed.");

        assert_eq!(
            DeriveKeyConfig::parse(&mut args),
            Err(ParsingError {
                message: path_parse_error.to_string()
            })
        )
    }

    #[test]
    fn test_invalid_path_integer_size_overlow() {
        let path = "/2147483648";
        let mut args = vec!["derive-key", "--path", path];

        let path_parse_error = format!("m{path}")
            .parse::<DerivationPath>()
            .expect_err("Invalid path should not be parsed.");

        assert_eq!(
            DeriveKeyConfig::parse(&mut args),
            Err(ParsingError {
                message: path_parse_error.to_string()
            })
        )
    }
}
