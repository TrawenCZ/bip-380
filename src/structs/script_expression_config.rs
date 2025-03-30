use crate::{parsers::flag_parser::parse_boolean_flag, traits::parsable::Parsable};

use super::parsing_error::ParsingError;

#[derive(Debug, PartialEq, Eq)]
pub struct ScriptExpressionConfig {
    pub compute_checksum: bool,
    pub verify_checksum: bool,
}

impl Parsable for ScriptExpressionConfig {
    fn parse(args: &mut Vec<&str>) -> Result<Self, ParsingError> {
        let compute_checksum = parse_boolean_flag(args, "compute-checksum");
        let verify_checksum = parse_boolean_flag(args, "verify-checksum");
        if compute_checksum && verify_checksum {
            return Err(ParsingError::new(
                "use only '--verify-checksum' or '--compute-checksum', not both",
            ));
        }

        Ok(ScriptExpressionConfig {
            compute_checksum,
            verify_checksum,
        })
    }
}

mod tests {

    #[allow(unused_imports)]
    use crate::{
        structs::parsing_error::ParsingError,
        structs::script_expression_config::ScriptExpressionConfig, traits::parsable::Parsable,
    };

    #[test]
    fn test_no_checksum_flags_provided() {
        let mut args = vec!["script-expression"];

        assert_eq!(
            ScriptExpressionConfig::parse(&mut args),
            Ok(ScriptExpressionConfig {
                compute_checksum: false,
                verify_checksum: false
            })
        );
    }

    #[test]
    fn test_compute_checksum_flag_provided() {
        let mut args = vec!["script-expression", "--compute-checksum"];

        assert_eq!(
            ScriptExpressionConfig::parse(&mut args),
            Ok(ScriptExpressionConfig {
                compute_checksum: true,
                verify_checksum: false
            })
        );
    }

    #[test]
    fn test_verify_checksum_flag_provided() {
        let mut args = vec!["script-expression", "--verify-checksum"];

        assert_eq!(
            ScriptExpressionConfig::parse(&mut args),
            Ok(ScriptExpressionConfig {
                compute_checksum: false,
                verify_checksum: true
            })
        );
    }

    #[test]
    fn test_both_checksum_flags_provided() {
        let mut args = vec![
            "script-expression",
            "--compute-checksum",
            "--verify-checksum",
        ];

        assert_eq!(
            ScriptExpressionConfig::parse(&mut args),
            Err(ParsingError::new(
                "use only '--verify-checksum' or '--compute-checksum', not both"
            ))
        );
    }
}
