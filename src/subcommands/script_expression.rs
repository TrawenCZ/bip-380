use crate::{
    structs::{parsing_error::ParsingError, script_expression_config::ScriptExpressionConfig},
    traits::string_utils::{CharArrayUtils, StringSliceUtils, Trimifiable},
    utils::error_messages::script_sh_unsupported_arg_err,
};

use super::{
    key_expression::validate_key_expression,
    utils::{
        checksum::{
            checksum_check, checksum_create, checksum_length_check, CHECKSUM_DIVIDER_SYMBOL,
        },
        hexadecimal::assert_hexadecimal_format,
    },
};

pub fn script_expression(
    input: &str,
    config: &ScriptExpressionConfig,
) -> Result<String, ParsingError> {
    let (script, checksum) = divide_script_and_checksum(input);
    match script.charify().trimify().as_slice() {
        ['r', 'a', 'w', rest @ ..] => match rest.extract_args("raw")?.as_slice() {
            [arg] => {
                assert_hexadecimal_format(arg, "raw function argument")?;
            }
            _ => return Err(ParsingError::new("script parsing failed!")),
        },
        ['m', 'u', 'l', 't', 'i', rest @ ..] => match rest.extract_args("multi")?.as_slice() {
            [arg_count, rest_of_args @ ..] => match arg_count.parse::<i32>()? {
                val if val < 0 => {
                    return Err(ParsingError::new("arg count indicator cannot be negative"))
                }
                val => {
                    let val_usize: usize = val.try_into().expect("value is positive");
                    if val_usize <= rest_of_args.len() {
                        for arg in rest_of_args {
                            validate_key_expression(arg.clone())?;
                        }
                    } else {
                        return Err(ParsingError::new(
                            "arg count indicator cannot be higher than actual args count",
                        ));
                    }
                }
            },
            _ => return Err(ParsingError::new("at least two arguments needed")),
        },
        ['p', 'k', 'h', rest @ ..] => match rest.extract_args("pkh")?.as_slice() {
            [arg] => {
                validate_key_expression(arg.clone())?;
            }
            _ => {
                return Err(ParsingError::new(
                    "exactly one argument is needed for pkh script",
                ))
            }
        },
        ['p', 'k', rest @ ..] => match rest.extract_args("pk")?.as_slice() {
            [arg] => {
                validate_key_expression(arg.clone())?;
            }
            _ => {
                return Err(ParsingError::new(
                    "exactly one argument is needed for pk script",
                ))
            }
        },
        ['s', 'h', rest @ ..] => match rest.extract_args("sh")?.as_slice() {
            [arg]
                if arg.starts_with("pkh") || arg.starts_with("pk") || arg.starts_with("multi") =>
            {
                script_expression(&arg.clone(), &ScriptExpressionConfig::default())?;
            }
            [arg] => return Err(ParsingError::new(&script_sh_unsupported_arg_err(arg))),
            _ => {
                return Err(ParsingError::new(
                    "exactly one argument is needed for sh script",
                ))
            }
        },
        _ => return Err(ParsingError::new("parsing of the script failed!")),
    }
    script_operation(&script, checksum.as_ref(), config)
}

fn divide_script_and_checksum(input: &str) -> (String, Option<String>) {
    let parts: Vec<&str> = input.splitn(2, CHECKSUM_DIVIDER_SYMBOL).collect();
    let script = parts.first().map_or("", |v| v).to_string();
    let checksum = parts.get(1).map(|s| (*s).to_string());
    (script, checksum)
}

fn script_operation(
    script: &str,
    checksum: Option<&String>,
    config: &ScriptExpressionConfig,
) -> Result<String, ParsingError> {
    if config.compute_checksum {
        // ignores checksum
        return Ok(format!("{}#{}", script, checksum_create(script)));
    }
    match checksum {
        Some(checksum) => {
            if checksum_length_check(checksum) {
                if config.verify_checksum {
                    if checksum_check(script, checksum) {
                        Ok(format!(
                            "Veritification of the '{script}#{checksum}' script succeeded!"
                        ))
                    } else {
                        Err(ParsingError::new("checksum verification failed!"))
                    }
                } else {
                    Ok(format!("{script}#{checksum}"))
                }
            } else {
                Err(ParsingError::new("checksum length is incorrect!"))
            }
        }
        None => {
            if config.verify_checksum {
                Err(ParsingError::new("checksum is required for verification!"))
            } else {
                Ok(script.to_string())
            }
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::{
        structs::script_expression_config::ScriptExpressionConfig, test_utils::get_cmd,
        utils::error_messages::script_arg_extraction_err,
    };

    const CONFIG_WITH_FALSE_COMPUTE_AND_VERIFY: ScriptExpressionConfig = ScriptExpressionConfig {
        compute_checksum: false,
        verify_checksum: false,
    };

    const CONFIG_WITH_TRUE_VERIFY: ScriptExpressionConfig = ScriptExpressionConfig {
        compute_checksum: false,
        verify_checksum: true,
    };

    const CONFIG_WITH_TRUE_COMPUTE: ScriptExpressionConfig = ScriptExpressionConfig {
        compute_checksum: true,
        verify_checksum: false,
    };

    #[test]
    fn test_raw_script() {
        assert_eq!(
            script_expression("raw(deadbeef)", &CONFIG_WITH_FALSE_COMPUTE_AND_VERIFY),
            Ok("raw(deadbeef)".to_string())
        );
        assert_eq!(
            script_expression("raw( deadbeef )", &CONFIG_WITH_FALSE_COMPUTE_AND_VERIFY),
            Ok("raw( deadbeef )".to_string())
        );
        assert_eq!(
            script_expression("raw(DEAD BEEF)", &CONFIG_WITH_FALSE_COMPUTE_AND_VERIFY),
            Ok("raw(DEAD BEEF)".to_string())
        );
        assert_eq!(
            script_expression("raw(DEA D BEEF)", &CONFIG_WITH_FALSE_COMPUTE_AND_VERIFY),
            Ok("raw(DEA D BEEF)".to_string())
        );
        assert_eq!(
            script_expression(
                "    raw    (   D    E   A    D    )    ",
                &CONFIG_WITH_FALSE_COMPUTE_AND_VERIFY
            ),
            Ok("    raw    (   D    E   A    D    )    ".to_string())
        );
        assert_eq!(
            script_expression("  \t\t\t  raw  \t\t\t  (  \t\t\t  D  \t\t\t  E  \t\t\t  A  \t\t\t  D  \t\t\t  )  \t\t\t  ", &CONFIG_WITH_FALSE_COMPUTE_AND_VERIFY),
            Err(ParsingError::new("parsing of the script failed!"))
        );
        assert_eq!(
            script_expression("raw(\tDEADBEEF)", &CONFIG_WITH_FALSE_COMPUTE_AND_VERIFY),
            Err(ParsingError::new(
                "raw function argument '\tDEADBEEF' is not a valid hexadecimal string!"
            ))
        );
        assert_eq!(
            script_expression("raw(\nDEADBEEF)", &CONFIG_WITH_FALSE_COMPUTE_AND_VERIFY),
            Err(ParsingError::new(
                "raw function argument '\nDEADBEEF' is not a valid hexadecimal string!"
            ))
        );
        assert_eq!(
            script_expression("raw(\u{a0}DEADBEEF)", &CONFIG_WITH_FALSE_COMPUTE_AND_VERIFY),
            Err(ParsingError::new(
                "raw function argument '\u{a0}DEADBEEF' is not a valid hexadecimal string!"
            ))
        );
        assert_eq!(
            script_expression("raw(nothexadecimal)", &CONFIG_WITH_FALSE_COMPUTE_AND_VERIFY),
            Err(ParsingError::new(
                "raw function argument 'nothexadecimal' is not a valid hexadecimal string!"
            ))
        );
        assert_eq!(
            script_expression("raw(nothexadecimal)", &CONFIG_WITH_FALSE_COMPUTE_AND_VERIFY),
            Err(ParsingError::new(
                "raw function argument 'nothexadecimal' is not a valid hexadecimal string!"
            ))
        );

        assert_eq!(
            script_expression("raw()", &CONFIG_WITH_FALSE_COMPUTE_AND_VERIFY),
            Err(ParsingError::new(
                "raw function argument '' is not a valid hexadecimal string!"
            ))
        );
        assert_eq!(
            script_expression("ra w(deadbeef)", &CONFIG_WITH_FALSE_COMPUTE_AND_VERIFY),
            Err(ParsingError::new("parsing of the script failed!"))
        );
        assert_eq!(
            script_expression("raw(deadbeef)#", &CONFIG_WITH_FALSE_COMPUTE_AND_VERIFY),
            Err(ParsingError::new("checksum length is incorrect!"))
        );
        assert_eq!(
            script_expression(
                "raw(deadbeef)#89f8spxmx",
                &CONFIG_WITH_FALSE_COMPUTE_AND_VERIFY
            ),
            Err(ParsingError::new("checksum length is incorrect!"))
        );
        assert_eq!(
            script_expression(
                "raw(deadbeef)#89f8spx",
                &CONFIG_WITH_FALSE_COMPUTE_AND_VERIFY
            ),
            Err(ParsingError::new("checksum length is incorrect!"))
        );
        assert_eq!(
            script_expression(
                "raw(deadbeef)#0invalid", // despite it being incorrent, the verification is ignored, but the count of 8 chars still fits
                &CONFIG_WITH_FALSE_COMPUTE_AND_VERIFY
            ),
            Ok("raw(deadbeef)#0invalid".to_string())
        );
        assert_eq!(
            script_expression(
                "raw(deadbeef)##89f8spxm",
                &CONFIG_WITH_FALSE_COMPUTE_AND_VERIFY
            ),
            Err(ParsingError::new("checksum length is incorrect!"))
        );
        assert_eq!(
            script_expression("rawraw)", &CONFIG_WITH_FALSE_COMPUTE_AND_VERIFY),
            Err(ParsingError::new(&script_arg_extraction_err("raw")))
        );

        assert_eq!(
            script_expression("raw(deadbeef)#89f8spxm", &CONFIG_WITH_TRUE_VERIFY),
            Ok("Veritification of the 'raw(deadbeef)#89f8spxm' script succeeded!".to_string())
        );
        assert_eq!(
            script_expression("raw( deadbeef )#985dv2zl", &CONFIG_WITH_TRUE_VERIFY),
            Ok("Veritification of the 'raw( deadbeef )#985dv2zl' script succeeded!".to_string())
        );
        assert_eq!(
            script_expression("raw(DEAD BEEF)#qqn7ll2h", &CONFIG_WITH_TRUE_VERIFY),
            Ok("Veritification of the 'raw(DEAD BEEF)#qqn7ll2h' script succeeded!".to_string())
        );
        assert_eq!(
            script_expression("raw(DEA D BEEF)#egs9fwsr", &CONFIG_WITH_TRUE_VERIFY),
            Ok("Veritification of the 'raw(DEA D BEEF)#egs9fwsr' script succeeded!".to_string())
        );
        assert_eq!(
            script_expression("raw(DEA D BEEF)#agaaa9aa", &CONFIG_WITH_TRUE_VERIFY),
            Err(ParsingError::new("checksum verification failed!"))
        );
        assert_eq!(
            script_expression("raw(deedbeef)#89f8spxm", &CONFIG_WITH_TRUE_VERIFY),
            Err(ParsingError::new("checksum verification failed!"))
        );
        assert_eq!(
            script_expression("raw(DEA D BEEF)", &CONFIG_WITH_TRUE_VERIFY),
            Err(ParsingError::new("checksum is required for verification!"))
        );

        assert_eq!(
            script_expression("raw(deadbeef)", &CONFIG_WITH_TRUE_COMPUTE),
            Ok("raw(deadbeef)#89f8spxm".to_string())
        );
        assert_eq!(
            script_expression("raw( deadbeef )", &CONFIG_WITH_TRUE_COMPUTE),
            Ok("raw( deadbeef )#985dv2zl".to_string())
        );
        assert_eq!(
            script_expression("raw(DEAD BEEF)", &CONFIG_WITH_TRUE_COMPUTE),
            Ok("raw(DEAD BEEF)#qqn7ll2h".to_string())
        );
        assert_eq!(
            script_expression("raw(DEA D BEEF)", &CONFIG_WITH_TRUE_COMPUTE),
            Ok("raw(DEA D BEEF)#egs9fwsr".to_string())
        );
        assert_eq!(
            script_expression("raw(deadbeef)#xxxxxxxxxxxxxxxx", &CONFIG_WITH_TRUE_COMPUTE),
            Ok("raw(deadbeef)#89f8spxm".to_string())
        );
        assert_eq!(
            script_expression("raw(deadbeef)####", &CONFIG_WITH_TRUE_COMPUTE),
            Ok("raw(deadbeef)#89f8spxm".to_string())
        );
    }

    #[test]
    fn test_multi_script() {
        assert_eq!(script_expression("multi(2, xpub661MyMwAqRbcFtXgS5sYJABqqG9YLmC4Q1Rdap9gSE8NqtwybGhePY2gZ29ESFjqJoCu1Rupje8YtGqsefD265TMg7usUDFdp6W1EGMcet8, xpub661MyMwAqRbcFW31YEwpkMuc5THy2PSt5bDMsktWQcFF8syAmRUapSCGu8ED9W6oDMSgv6Zz8idoc4a6mr8BDzTJY47LJhkJ8UB7WEGuduB)#5jlj4shz", &CONFIG_WITH_TRUE_COMPUTE), Ok("multi(2, xpub661MyMwAqRbcFtXgS5sYJABqqG9YLmC4Q1Rdap9gSE8NqtwybGhePY2gZ29ESFjqJoCu1Rupje8YtGqsefD265TMg7usUDFdp6W1EGMcet8, xpub661MyMwAqRbcFW31YEwpkMuc5THy2PSt5bDMsktWQcFF8syAmRUapSCGu8ED9W6oDMSgv6Zz8idoc4a6mr8BDzTJY47LJhkJ8UB7WEGuduB)#5jlj4shz".to_string()));
        assert_eq!(script_expression("multi(2, xpub661MyMwAqRbcFW31YEwpkMuc5THy2PSt5bDMsktWQcFF8syAmRUapSCGu8ED9W6oDMSgv6Zz8idoc4a6mr8BDzTJY47LJhkJ8UB7WEGuduB)", &CONFIG_WITH_FALSE_COMPUTE_AND_VERIFY), Err(ParsingError::new("arg count indicator cannot be higher than actual args count")));
        assert_eq!(script_expression("multi(1, xpub661MyMwAqRbcFtXgS5sYJABqqG9YLmC4Q1Rdap9gSE8NqtwybGhePY2gZ29ESFjqJoCu1Rupje8YtGqsefD265TMg7usUDFdp6W1EGMcet8, xpub661MyMwAqRbcFW31YEwpkMuc5THy2PSt5bDMsktWQcFF8syAmRUapSCGu8ED9W6oDMSgv6Zz8idoc4a6mr8BDzTJY47LJhkJ8UB7WEGuduB)", &CONFIG_WITH_FALSE_COMPUTE_AND_VERIFY), Ok("multi(1, xpub661MyMwAqRbcFtXgS5sYJABqqG9YLmC4Q1Rdap9gSE8NqtwybGhePY2gZ29ESFjqJoCu1Rupje8YtGqsefD265TMg7usUDFdp6W1EGMcet8, xpub661MyMwAqRbcFW31YEwpkMuc5THy2PSt5bDMsktWQcFF8syAmRUapSCGu8ED9W6oDMSgv6Zz8idoc4a6mr8BDzTJY47LJhkJ8UB7WEGuduB)".to_string()));
        assert_eq!(script_expression("multi(-1, xpub661MyMwAqRbcFtXgS5sYJABqqG9YLmC4Q1Rdap9gSE8NqtwybGhePY2gZ29ESFjqJoCu1Rupje8YtGqsefD265TMg7usUDFdp6W1EGMcet8)", &CONFIG_WITH_FALSE_COMPUTE_AND_VERIFY), Err(ParsingError::new("arg count indicator cannot be negative")));
        assert_eq!(script_expression("multi(1, xpub661MyMwAqRbcFtXgS5sYJABqqG9YLmC4Q1Rdap9gSE8Nqtwyb \t GhePY2gZ29ESFjqJoCu1Rupje8YtGqsefD265TMg7usUDFdp6W1EGMcet8)", &CONFIG_WITH_FALSE_COMPUTE_AND_VERIFY), Err(ParsingError::new("Input contains invalid characters")));
        assert_eq!(script_expression("multi(1, xpub661MyMwAqRbcFtXgS5sYJABqqG9YLmC4Q1Rdap9gSE8NqtwybčGhePY2gZ29ESFjqJoCu1Rupje8YtGqsefD265TMg7usUDFdp6W1EGMcet8)", &CONFIG_WITH_FALSE_COMPUTE_AND_VERIFY), Err(ParsingError::new("Input contains invalid characters")));
        assert_eq!(
            script_expression("multi(0)", &CONFIG_WITH_FALSE_COMPUTE_AND_VERIFY),
            Ok("multi(0)".to_string())
        );
        assert_eq!(
            script_expression("multi(1)", &CONFIG_WITH_FALSE_COMPUTE_AND_VERIFY),
            Err(ParsingError::new(
                "arg count indicator cannot be higher than actual args count"
            ))
        );
        assert_eq!(script_expression(" \t \t \t multi \t \t \t (\t \t \t 2 \t \t \t, \t \t \t xpub661MyMwAqRbcFtXgS5sYJABqqG9YLmC4Q1Rdap9gSE8NqtwybGhePY2gZ29ESFjqJoCu1Rupje8YtGqsefD265TMg7usUDFdp6W1EGMcet8, \t \t \t xpub661MyMwAqRbcFW31YEwpkMuc5THy2PSt5bDMsktWQcFF8syAmRUapSCGu8ED9W6oDMSgv6Zz8idoc4a6mr8BDzTJY47LJhkJ8UB7WEGuduB)\t \t \t", &CONFIG_WITH_FALSE_COMPUTE_AND_VERIFY), Err(ParsingError::new("parsing of the script failed!")));
        assert_eq!(script_expression("multi(\t2, xpub661MyMwAqRbcFtXgS5sYJABqqG9YLmC4Q1Rdap9gSE8NqtwybGhePY2gZ29ESFjqJoCu1Rupje8YtGqsefD265TMg7usUDFdp6W1EGMcet8, xpub661MyMwAqRbcFW31YEwpkMuc5THy2PSt5bDMsktWQcFF8syAmRUapSCGu8ED9W6oDMSgv6Zz8idoc4a6mr8BDzTJY47LJhkJ8UB7WEGuduB)", &CONFIG_WITH_FALSE_COMPUTE_AND_VERIFY), Err(ParsingError::new("invalid digit found in string")));
        assert_eq!(script_expression("multi(2,\txpub661MyMwAqRbcFtXgS5sYJABqqG9YLmC4Q1Rdap9gSE8NqtwybGhePY2gZ29ESFjqJoCu1Rupje8YtGqsefD265TMg7usUDFdp6W1EGMcet8, xpub661MyMwAqRbcFW31YEwpkMuc5THy2PSt5bDMsktWQcFF8syAmRUapSCGu8ED9W6oDMSgv6Zz8idoc4a6mr8BDzTJY47LJhkJ8UB7WEGuduB)", &CONFIG_WITH_FALSE_COMPUTE_AND_VERIFY), Err(ParsingError::new("Input contains invalid characters")));
        assert_eq!(script_expression("multi(2, xpub661MyMwAqRbcFtXgS5sYJABqqG9YLmC4Q1Rdap9gSE8NqtwybGhePY2gZ29ESFjqJoCu1Rupje8YtGqsefD265TMg7usUDFdp6W1EGMcet8,\txpub661MyMwAqRbcFW31YEwpkMuc5THy2PSt5bDMsktWQcFF8syAmRUapSCGu8ED9W6oDMSgv6Zz8idoc4a6mr8BDzTJY47LJhkJ8UB7WEGuduB)", &CONFIG_WITH_FALSE_COMPUTE_AND_VERIFY), Err(ParsingError::new("Input contains invalid characters")));
        assert_eq!(script_expression("multi(\n2, xpub661MyMwAqRbcFtXgS5sYJABqqG9YLmC4Q1Rdap9gSE8NqtwybGhePY2gZ29ESFjqJoCu1Rupje8YtGqsefD265TMg7usUDFdp6W1EGMcet8, xpub661MyMwAqRbcFW31YEwpkMuc5THy2PSt5bDMsktWQcFF8syAmRUapSCGu8ED9W6oDMSgv6Zz8idoc4a6mr8BDzTJY47LJhkJ8UB7WEGuduB)", &CONFIG_WITH_FALSE_COMPUTE_AND_VERIFY), Err(ParsingError::new("invalid digit found in string")));
        assert_eq!(script_expression("multi(2,\nxpub661MyMwAqRbcFtXgS5sYJABqqG9YLmC4Q1Rdap9gSE8NqtwybGhePY2gZ29ESFjqJoCu1Rupje8YtGqsefD265TMg7usUDFdp6W1EGMcet8, xpub661MyMwAqRbcFW31YEwpkMuc5THy2PSt5bDMsktWQcFF8syAmRUapSCGu8ED9W6oDMSgv6Zz8idoc4a6mr8BDzTJY47LJhkJ8UB7WEGuduB)", &CONFIG_WITH_FALSE_COMPUTE_AND_VERIFY), Err(ParsingError::new("Input contains invalid characters")));
        assert_eq!(script_expression("multi(2, xpub661MyMwAqRbcFtXgS5sYJABqqG9YLmC4Q1Rdap9gSE8NqtwybGhePY2gZ29ESFjqJoCu1Rupje8YtGqsefD265TMg7usUDFdp6W1EGMcet8,\nxpub661MyMwAqRbcFW31YEwpkMuc5THy2PSt5bDMsktWQcFF8syAmRUapSCGu8ED9W6oDMSgv6Zz8idoc4a6mr8BDzTJY47LJhkJ8UB7WEGuduB)", &CONFIG_WITH_FALSE_COMPUTE_AND_VERIFY), Err(ParsingError::new("Input contains invalid characters")));
        assert_eq!(script_expression("multi(\u{a0}2, xpub661MyMwAqRbcFtXgS5sYJABqqG9YLmC4Q1Rdap9gSE8NqtwybGhePY2gZ29ESFjqJoCu1Rupje8YtGqsefD265TMg7usUDFdp6W1EGMcet8, xpub661MyMwAqRbcFW31YEwpkMuc5THy2PSt5bDMsktWQcFF8syAmRUapSCGu8ED9W6oDMSgv6Zz8idoc4a6mr8BDzTJY47LJhkJ8UB7WEGuduB)", &CONFIG_WITH_FALSE_COMPUTE_AND_VERIFY), Err(ParsingError::new("invalid digit found in string")));
        assert_eq!(script_expression("multi(2,\u{a0}xpub661MyMwAqRbcFtXgS5sYJABqqG9YLmC4Q1Rdap9gSE8NqtwybGhePY2gZ29ESFjqJoCu1Rupje8YtGqsefD265TMg7usUDFdp6W1EGMcet8, xpub661MyMwAqRbcFW31YEwpkMuc5THy2PSt5bDMsktWQcFF8syAmRUapSCGu8ED9W6oDMSgv6Zz8idoc4a6mr8BDzTJY47LJhkJ8UB7WEGuduB)", &CONFIG_WITH_FALSE_COMPUTE_AND_VERIFY), Err(ParsingError::new("Input contains invalid characters")));
        assert_eq!(script_expression("multi(2, xpub661MyMwAqRbcFtXgS5sYJABqqG9YLmC4Q1Rdap9gSE8NqtwybGhePY2gZ29ESFjqJoCu1Rupje8YtGqsefD265TMg7usUDFdp6W1EGMcet8,\u{a0}xpub661MyMwAqRbcFW31YEwpkMuc5THy2PSt5bDMsktWQcFF8syAmRUapSCGu8ED9W6oDMSgv6Zz8idoc4a6mr8BDzTJY47LJhkJ8UB7WEGuduB)", &CONFIG_WITH_FALSE_COMPUTE_AND_VERIFY), Err(ParsingError::new("Input contains invalid characters")));
    }

    #[test]
    fn test_pk_script() {
        assert_eq!(script_expression("pk(xpub661MyMwAqRbcFtXgS5sYJABqqG9YLmC4Q1Rdap9gSE8NqtwybGhePY2gZ29ESFjqJoCu1Rupje8YtGqsefD265TMg7usUDFdp6W1EGMcet8)", &CONFIG_WITH_TRUE_COMPUTE), Ok("pk(xpub661MyMwAqRbcFtXgS5sYJABqqG9YLmC4Q1Rdap9gSE8NqtwybGhePY2gZ29ESFjqJoCu1Rupje8YtGqsefD265TMg7usUDFdp6W1EGMcet8)#axav5m0j".to_string()));
        assert_eq!(script_expression("pk(   xpub661MyMwAqRbcFtXgS5sYJABqqG9YLmC4Q1Rdap9gSE8NqtwybGhePY2gZ29ESFjqJoCu1Rupje8YtGqsefD265TMg7usUDFdp6W1EGMcet8)", &CONFIG_WITH_TRUE_COMPUTE), Ok("pk(   xpub661MyMwAqRbcFtXgS5sYJABqqG9YLmC4Q1Rdap9gSE8NqtwybGhePY2gZ29ESFjqJoCu1Rupje8YtGqsefD265TMg7usUDFdp6W1EGMcet8)#yjz8lyzk".to_string()));
        assert_eq!(
            script_expression("pk(xpub_invalid_format)", &CONFIG_WITH_TRUE_COMPUTE),
            Err(ParsingError::new("Invalid xpub key: base58 error"))
        );
        assert_eq!(script_expression("pk(xpub661MyMwAqRbcFtXgS5sYJABqqG9YLmC4Q1Rdap9gSE8NqtwybGhePY2gZ29ESFjqJoCu1Rupje8YtGqsefD265TMg7usUDFdp6W1EGMcet8)#invalid_checksum", &CONFIG_WITH_TRUE_COMPUTE), Ok("pk(xpub661MyMwAqRbcFtXgS5sYJABqqG9YLmC4Q1Rdap9gSE8NqtwybGhePY2gZ29ESFjqJoCu1Rupje8YtGqsefD265TMg7usUDFdp6W1EGMcet8)#axav5m0j".to_string()));
        assert_eq!(script_expression("pk(xpub661MyMwAqRbcFtXgS5sYJABqqG9YLmC4Q1Rdap9gSE8NqtwybGhePY2gZ29ESFjqJoCu1Rupje8YtGqsefD265TMg7usUDFdp6W1EGMcet8)extra", &CONFIG_WITH_TRUE_COMPUTE), Err(ParsingError::new(&script_arg_extraction_err("pk"))));
        assert_eq!(
            script_expression("pk()", &CONFIG_WITH_TRUE_COMPUTE),
            Err(ParsingError::new("Input is empty"))
        );
        assert_eq!(
            script_expression("pk(arg1, arg2)", &CONFIG_WITH_TRUE_COMPUTE),
            Err(ParsingError::new(
                "exactly one argument is needed for pk script"
            ))
        );
        assert_eq!(script_expression("  pk  (  xpub661MyMwAqRbcFtXgS5sYJABqqG9YLmC4Q1Rdap9gSE8NqtwybGhePY2gZ29ESFjqJoCu1Rupje8YtGqsefD265TMg7usUDFdp6W1EGMcet8  )  ", &CONFIG_WITH_TRUE_COMPUTE), Ok("  pk  (  xpub661MyMwAqRbcFtXgS5sYJABqqG9YLmC4Q1Rdap9gSE8NqtwybGhePY2gZ29ESFjqJoCu1Rupje8YtGqsefD265TMg7usUDFdp6W1EGMcet8  )  #004vptms".to_string()));
        assert_eq!(script_expression("pk(xpub661MyMwAqRbcFtXgS5sYJABqqG9YLmC4Q1Rdap9gSE8NqtwybGhePY2gZ29ESFjqJoCu1Rupje8YtGqsefD265TMg7usUDFdp6W1EGMcet8)", &CONFIG_WITH_FALSE_COMPUTE_AND_VERIFY), Ok("pk(xpub661MyMwAqRbcFtXgS5sYJABqqG9YLmC4Q1Rdap9gSE8NqtwybGhePY2gZ29ESFjqJoCu1Rupje8YtGqsefD265TMg7usUDFdp6W1EGMcet8)".to_string()));
        assert_eq!(
            script_expression("pk(invalid_xpub)", &CONFIG_WITH_FALSE_COMPUTE_AND_VERIFY),
            Err(ParsingError::new("Could not convert WIF from base58"))
        );
        assert_eq!(script_expression("pk(xpub661MyMwAqRbcFtXgS5sYJABqqG9YLmC4Q1Rdap9gSE8NqtwybGhePY2gZ29ESFjqJoCu1Rupje8YtGqsefD265TMg7usUDFdp6W1EGMcet8)#invalid", &CONFIG_WITH_FALSE_COMPUTE_AND_VERIFY), Err(ParsingError::new("checksum length is incorrect!")));
        assert_eq!(script_expression("pk(xpub661MyMwAqRbcFtXgS5sYJABqqG9YLmC4Q1Rdap9gSE8NqtwybGhePY2gZ29ESFjqJoCu1Rupje8YtGqsefD265TMg7usUDFdp6W1EGMcet8)#abcdefgh", &CONFIG_WITH_FALSE_COMPUTE_AND_VERIFY), Ok("pk(xpub661MyMwAqRbcFtXgS5sYJABqqG9YLmC4Q1Rdap9gSE8NqtwybGhePY2gZ29ESFjqJoCu1Rupje8YtGqsefD265TMg7usUDFdp6W1EGMcet8)#abcdefgh".to_string()));
        assert_eq!(script_expression("pk(xpub661MyMwAqRbcFtXgS5sYJABqqG9YLmC4Q1Rdap9gSE8NqtwybGhePY2gZ29ESFjqJoCu1Rupje8YtGqsefD265TMg7usUDFdp6W1EGMcet8)#axav5m0j", &CONFIG_WITH_TRUE_VERIFY), Ok("Veritification of the 'pk(xpub661MyMwAqRbcFtXgS5sYJABqqG9YLmC4Q1Rdap9gSE8NqtwybGhePY2gZ29ESFjqJoCu1Rupje8YtGqsefD265TMg7usUDFdp6W1EGMcet8)#axav5m0j' script succeeded!".to_string()));
        assert_eq!(script_expression("pk(xpub661MyMwAqRbcFtXgS5sYJABqqG9YLmC4Q1Rdap9gSE8NqtwybGhePY2gZ29ESFjqJoCu1Rupje8YtGqsefD265TMg7usUDFdp6W1EGMcet8)#invalid", &CONFIG_WITH_TRUE_VERIFY), Err(ParsingError::new("checksum length is incorrect!")));
        assert_eq!(script_expression("pk(xpub661MyMwAqRbcFtXgS5sYJABqqG9YLmC4Q1Rdap9gSE8NqtwybGhePY2gZ29ESFjqJoCu1Rupje8YtGqsefD265TMg7usUDFdp6W1EGMcet8)#abcdefgh", &CONFIG_WITH_TRUE_VERIFY), Err(ParsingError::new("checksum verification failed!")));
        assert_eq!(script_expression("pk(xpub661MyMwAqRbcFtXgS5sYJABqqG9YLmC4Q1Rdap9gSE8NqtwybGhePY2gZ29ESFjqJoCu1Rupje8YtGqsefD265TMg7usUDFdp6W1EGMcet8)", &CONFIG_WITH_TRUE_VERIFY), Err(ParsingError::new("checksum is required for verification!")));
    }

    #[test]
    fn test_pkh_script() {
        assert_eq!(script_expression("pkh(xpub661MyMwAqRbcFtXgS5sYJABqqG9YLmC4Q1Rdap9gSE8NqtwybGhePY2gZ29ESFjqJoCu1Rupje8YtGqsefD265TMg7usUDFdp6W1EGMcet8)", &CONFIG_WITH_TRUE_COMPUTE), Ok("pkh(xpub661MyMwAqRbcFtXgS5sYJABqqG9YLmC4Q1Rdap9gSE8NqtwybGhePY2gZ29ESFjqJoCu1Rupje8YtGqsefD265TMg7usUDFdp6W1EGMcet8)#vm4xc4ed".to_string()));
        assert_eq!(script_expression("pkh(   xpub661MyMwAqRbcFtXgS5sYJABqqG9YLmC4Q1Rdap9gSE8NqtwybGhePY2gZ29ESFjqJoCu1Rupje8YtGqsefD265TMg7usUDFdp6W1EGMcet8)", &CONFIG_WITH_TRUE_COMPUTE), Ok("pkh(   xpub661MyMwAqRbcFtXgS5sYJABqqG9YLmC4Q1Rdap9gSE8NqtwybGhePY2gZ29ESFjqJoCu1Rupje8YtGqsefD265TMg7usUDFdp6W1EGMcet8)#ujpe9npc".to_string()));
        assert_eq!(
            script_expression("pkh(xpub_invalid_format)", &CONFIG_WITH_TRUE_COMPUTE),
            Err(ParsingError::new("Invalid xpub key: base58 error"))
        );
        assert_eq!(script_expression("pkh(xpub661MyMwAqRbcFtXgS5sYJABqqG9YLmC4Q1Rdap9gSE8NqtwybGhePY2gZ29ESFjqJoCu1Rupje8YtGqsefD265TMg7usUDFdp6W1EGMcet8)#invalid_checksum", &CONFIG_WITH_TRUE_COMPUTE), Ok("pkh(xpub661MyMwAqRbcFtXgS5sYJABqqG9YLmC4Q1Rdap9gSE8NqtwybGhePY2gZ29ESFjqJoCu1Rupje8YtGqsefD265TMg7usUDFdp6W1EGMcet8)#vm4xc4ed".to_string()));
        assert_eq!(script_expression("pkh(xpub661MyMwAqRbcFtXgS5sYJABqqG9YLmC4Q1Rdap9gSE8NqtwybGhePY2gZ29ESFjqJoCu1Rupje8YtGqsefD265TMg7usUDFdp6W1EGMcet8)extra", &CONFIG_WITH_TRUE_COMPUTE), Err(ParsingError::new("Could not extract arguments from 'pkh' expression.")));
        assert_eq!(
            script_expression("pkh()", &CONFIG_WITH_TRUE_COMPUTE),
            Err(ParsingError::new("Input is empty"))
        );
        assert_eq!(
            script_expression("pkh(arg1, arg2)", &CONFIG_WITH_TRUE_COMPUTE),
            Err(ParsingError::new(
                "exactly one argument is needed for pkh script"
            ))
        );
        assert_eq!(script_expression("  pkh  (  xpub661MyMwAqRbcFtXgS5sYJABqqG9YLmC4Q1Rdap9gSE8NqtwybGhePY2gZ29ESFjqJoCu1Rupje8YtGqsefD265TMg7usUDFdp6W1EGMcet8  )  ", &CONFIG_WITH_TRUE_COMPUTE), Ok("  pkh  (  xpub661MyMwAqRbcFtXgS5sYJABqqG9YLmC4Q1Rdap9gSE8NqtwybGhePY2gZ29ESFjqJoCu1Rupje8YtGqsefD265TMg7usUDFdp6W1EGMcet8  )  #z4c2c9hz".to_string()));

        assert_eq!(script_expression("pkh(xpub661MyMwAqRbcFtXgS5sYJABqqG9YLmC4Q1Rdap9gSE8NqtwybGhePY2gZ29ESFjqJoCu1Rupje8YtGqsefD265TMg7usUDFdp6W1EGMcet8)", &CONFIG_WITH_FALSE_COMPUTE_AND_VERIFY), Ok("pkh(xpub661MyMwAqRbcFtXgS5sYJABqqG9YLmC4Q1Rdap9gSE8NqtwybGhePY2gZ29ESFjqJoCu1Rupje8YtGqsefD265TMg7usUDFdp6W1EGMcet8)".to_string()));
        assert_eq!(
            script_expression("pkh(invalid_xpub)", &CONFIG_WITH_FALSE_COMPUTE_AND_VERIFY),
            Err(ParsingError::new("Could not convert WIF from base58"))
        );
        assert_eq!(script_expression("pkh(xpub661MyMwAqRbcFtXgS5sYJABqqG9YLmC4Q1Rdap9gSE8NqtwybGhePY2gZ29ESFjqJoCu1Rupje8YtGqsefD265TMg7usUDFdp6W1EGMcet8)#invalid", &CONFIG_WITH_FALSE_COMPUTE_AND_VERIFY), Err(ParsingError::new("checksum length is incorrect!")));
        assert_eq!(script_expression("pkh(xpub661MyMwAqRbcFtXgS5sYJABqqG9YLmC4Q1Rdap9gSE8NqtwybGhePY2gZ29ESFjqJoCu1Rupje8YtGqsefD265TMg7usUDFdp6W1EGMcet8)#abcdefgh", &CONFIG_WITH_FALSE_COMPUTE_AND_VERIFY), Ok("pkh(xpub661MyMwAqRbcFtXgS5sYJABqqG9YLmC4Q1Rdap9gSE8NqtwybGhePY2gZ29ESFjqJoCu1Rupje8YtGqsefD265TMg7usUDFdp6W1EGMcet8)#abcdefgh".to_string()));
        assert_eq!(
            script_expression("pkh()", &CONFIG_WITH_FALSE_COMPUTE_AND_VERIFY),
            Err(ParsingError::new("Input is empty"))
        );

        assert_eq!(script_expression("pkh(xpub661MyMwAqRbcFtXgS5sYJABqqG9YLmC4Q1Rdap9gSE8NqtwybGhePY2gZ29ESFjqJoCu1Rupje8YtGqsefD265TMg7usUDFdp6W1EGMcet8)#vm4xc4ed", &CONFIG_WITH_TRUE_VERIFY), Ok("Veritification of the 'pkh(xpub661MyMwAqRbcFtXgS5sYJABqqG9YLmC4Q1Rdap9gSE8NqtwybGhePY2gZ29ESFjqJoCu1Rupje8YtGqsefD265TMg7usUDFdp6W1EGMcet8)#vm4xc4ed' script succeeded!".to_string()));
        assert_eq!(script_expression("pkh(xpub661MyMwAqRbcFtXgS5sYJABqqG9YLmC4Q1Rdap9gSE8NqtwybGhePY2gZ29ESFjqJoCu1Rupje8YtGqsefD265TMg7usUDFdp6W1EGMcet8)#invalid", &CONFIG_WITH_TRUE_VERIFY), Err(ParsingError::new("checksum length is incorrect!")));
        assert_eq!(script_expression("pkh(xpub661MyMwAqRbcFtXgS5sYJABqqG9YLmC4Q1Rdap9gSE8NqtwybGhePY2gZ29ESFjqJoCu1Rupje8YtGqsefD265TMg7usUDFdp6W1EGMcet8)#abcdefgh", &CONFIG_WITH_TRUE_VERIFY), Err(ParsingError::new("checksum verification failed!")));
        assert_eq!(script_expression("pkh(xpub661MyMwAqRbcFtXgS5sYJABqqG9YLmC4Q1Rdap9gSE8NqtwybGhePY2gZ29ESFjqJoCu1Rupje8YtGqsefD265TMg7usUDFdp6W1EGMcet8)", &CONFIG_WITH_TRUE_VERIFY), Err(ParsingError::new("checksum is required for verification!")));
    }

    #[test]
    fn test_sh_script() {
        assert_eq!(script_expression("sh(multi(2, xpub661MyMwAqRbcFtXgS5sYJABqqG9YLmC4Q1Rdap9gSE8NqtwybGhePY2gZ29ESFjqJoCu1Rupje8YtGqsefD265TMg7usUDFdp6W1EGMcet8, xpub661MyMwAqRbcFW31YEwpkMuc5THy2PSt5bDMsktWQcFF8syAmRUapSCGu8ED9W6oDMSgv6Zz8idoc4a6mr8BDzTJY47LJhkJ8UB7WEGuduB))#3txhxflq", &CONFIG_WITH_TRUE_COMPUTE), Ok("sh(multi(2, xpub661MyMwAqRbcFtXgS5sYJABqqG9YLmC4Q1Rdap9gSE8NqtwybGhePY2gZ29ESFjqJoCu1Rupje8YtGqsefD265TMg7usUDFdp6W1EGMcet8, xpub661MyMwAqRbcFW31YEwpkMuc5THy2PSt5bDMsktWQcFF8syAmRUapSCGu8ED9W6oDMSgv6Zz8idoc4a6mr8BDzTJY47LJhkJ8UB7WEGuduB))#3txhxflq".to_string()));

        assert_eq!(
            script_expression("sh(raw(deadbeef))", &CONFIG_WITH_FALSE_COMPUTE_AND_VERIFY),
            Err(ParsingError::new(&script_sh_unsupported_arg_err(
                "raw(deadbeef)"
            )))
        );
        assert_eq!(
            script_expression(
                "sh(pkh(xpub661MyMwAqRbcFtXgS5sYJABqqG9YLmC4Q1Rdap9gSE8NqtwybGhePY2gZ29ESFjqJoCu1Rupje8YtGqsefD265TMg7usUDFdp6W1EGMcet8))",
                &CONFIG_WITH_FALSE_COMPUTE_AND_VERIFY
            ),
            Ok("sh(pkh(xpub661MyMwAqRbcFtXgS5sYJABqqG9YLmC4Q1Rdap9gSE8NqtwybGhePY2gZ29ESFjqJoCu1Rupje8YtGqsefD265TMg7usUDFdp6W1EGMcet8))".to_string())
        );
        assert_eq!(
            script_expression(
                "sh(pk(xpub661MyMwAqRbcFtXgS5sYJABqqG9YLmC4Q1Rdap9gSE8NqtwybGhePY2gZ29ESFjqJoCu1Rupje8YtGqsefD265TMg7usUDFdp6W1EGMcet8))",
                &CONFIG_WITH_FALSE_COMPUTE_AND_VERIFY
            ),
            Ok("sh(pk(xpub661MyMwAqRbcFtXgS5sYJABqqG9YLmC4Q1Rdap9gSE8NqtwybGhePY2gZ29ESFjqJoCu1Rupje8YtGqsefD265TMg7usUDFdp6W1EGMcet8))".to_string())
        );
        assert_eq!(
            script_expression(
                "sh(multi(1, xpub661MyMwAqRbcFtXgS5sYJABqqG9YLmC4Q1Rdap9gSE8NqtwybGhePY2gZ29ESFjqJoCu1Rupje8YtGqsefD265TMg7usUDFdp6W1EGMcet8))",
                &CONFIG_WITH_FALSE_COMPUTE_AND_VERIFY
            ),
            Ok("sh(multi(1, xpub661MyMwAqRbcFtXgS5sYJABqqG9YLmC4Q1Rdap9gSE8NqtwybGhePY2gZ29ESFjqJoCu1Rupje8YtGqsefD265TMg7usUDFdp6W1EGMcet8))".to_string())
        );
        assert_eq!(
            script_expression("sh(multi(0))", &CONFIG_WITH_FALSE_COMPUTE_AND_VERIFY),
            Ok("sh(multi(0))".to_string())
        );
        assert_eq!(
            script_expression("sh( )", &CONFIG_WITH_FALSE_COMPUTE_AND_VERIFY),
            Err(ParsingError::new(
                "'sh' script's argument must be either 'pk', 'pkh' or 'multi' scripts, but '' was given."
            ))
        );
        assert_eq!(
            script_expression(
                "sh(multi(1, xpub1), extra_arg)",
                &CONFIG_WITH_FALSE_COMPUTE_AND_VERIFY
            ),
            Err(ParsingError::new(
                "exactly one argument is needed for sh script"
            ))
        );
        assert_eq!(
            script_expression(
                "sh(invalid_start)",
                &CONFIG_WITH_FALSE_COMPUTE_AND_VERIFY
            ),
            Err(ParsingError::new(
                "'sh' script's argument must be either 'pk', 'pkh' or 'multi' scripts, but 'invalid_start' was given."
            ))
        );
        assert_eq!(
            script_expression(
                "sh(pkh(invalid key))",
                &CONFIG_WITH_FALSE_COMPUTE_AND_VERIFY
            ),
            Err(ParsingError::new("Could not convert WIF from base58"))
        );
        assert_eq!(
            script_expression(
                "sh(multi(1, invalid key))",
                &CONFIG_WITH_FALSE_COMPUTE_AND_VERIFY
            ),
            Err(ParsingError::new("Could not convert WIF from base58"))
        );

        assert_eq!(
            script_expression(
                "sh(multi(2, xpub1, xpub2))#checksum",
                &CONFIG_WITH_TRUE_VERIFY
            ),
            Err(ParsingError::new("Invalid xpub key: base58 error"))
        );
    }

    // integration tests
    #[test]
    fn test_script_expression_verify_checksum() {
        get_cmd()
            .args([
                "script-expression",
                "--verify-checksum",
                "raw(deadbeef)#89f8spxm",
            ])
            .assert()
            .success()
            .stdout("Veritification of the \'raw(deadbeef)#89f8spxm\' script succeeded!\n");

        get_cmd()
            .args([
                "script-expression",
                "--verify-checksum",
                "raw( deadbeef )#985dv2zl",
            ])
            .assert()
            .success()
            .stdout("Veritification of the \'raw( deadbeef )#985dv2zl\' script succeeded!\n");

        get_cmd()
            .args([
                "script-expression",
                "--verify-checksum",
                "raw(DEADBEEF)#49w2hhz7",
            ])
            .assert()
            .success()
            .stdout("Veritification of the \'raw(DEADBEEF)#49w2hhz7\' script succeeded!\n");

        get_cmd()
            .args([
                "script-expression",
                "--verify-checksum",
                "raw(DEAD BEEF)#qqn7ll2h",
            ])
            .assert()
            .success()
            .stdout("Veritification of the \'raw(DEAD BEEF)#qqn7ll2h\' script succeeded!\n");

        get_cmd()
            .args([
                "script-expression",
                "--verify-checksum",
                "raw(DEA D BEEF)#egs9fwsr",
            ])
            .assert()
            .success()
            .stdout("Veritification of the \'raw(DEA D BEEF)#egs9fwsr\' script succeeded!\n");

        get_cmd()
            .args(["script-expression", "--verify-checksum", "raw(deadbeef)"])
            .assert()
            .failure()
            .stderr("Parsing error: checksum is required for verification!\n");
    }

    #[test]
    fn test_script_expression_compute_checksum() {
        get_cmd()
            .args([
                "script-expression",
                "--compute-checksum",
                "raw(deadbeef)#xxx",
            ])
            .assert()
            .success()
            .stdout("raw(deadbeef)#89f8spxm\n");

        get_cmd()
            .args([
                "script-expression",
                "--compute-checksum",
                "pkh(xpub661MyMwAqRbcFtXgS5sYJABqqG9YLmC4Q1Rdap9gSE8NqtwybGhePY2gZ29ESFjqJoCu1Rupje8YtGqsefD265TMg7usUDFdp6W1EGMcet8)",
            ])
            .assert()
            .success()
            .stdout("pkh(xpub661MyMwAqRbcFtXgS5sYJABqqG9YLmC4Q1Rdap9gSE8NqtwybGhePY2gZ29ESFjqJoCu1Rupje8YtGqsefD265TMg7usUDFdp6W1EGMcet8)#vm4xc4ed\n");

        get_cmd()
            .args([
                "script-expression",
                "--compute-checksum",
                "pkh(   xpub661MyMwAqRbcFtXgS5sYJABqqG9YLmC4Q1Rdap9gSE8NqtwybGhePY2gZ29ESFjqJoCu1Rupje8YtGqsefD265TMg7usUDFdp6W1EGMcet8)",
            ])
            .assert()
            .success()
            .stdout("pkh(   xpub661MyMwAqRbcFtXgS5sYJABqqG9YLmC4Q1Rdap9gSE8NqtwybGhePY2gZ29ESFjqJoCu1Rupje8YtGqsefD265TMg7usUDFdp6W1EGMcet8)#ujpe9npc\n");

        get_cmd()
            .args([
                "script-expression",
                "--compute-checksum",
                "multi(2, xpub661MyMwAqRbcFtXgS5sYJABqqG9YLmC4Q1Rdap9gSE8NqtwybGhePY2gZ29ESFjqJoCu1Rupje8YtGqsefD265TMg7usUDFdp6W1EGMcet8, xpub661MyMwAqRbcFW31YEwpkMuc5THy2PSt5bDMsktWQcFF8syAmRUapSCGu8ED9W6oDMSgv6Zz8idoc4a6mr8BDzTJY47LJhkJ8UB7WEGuduB)#5jlj4shz",
            ])
            .assert()
            .success()
            .stdout("multi(2, xpub661MyMwAqRbcFtXgS5sYJABqqG9YLmC4Q1Rdap9gSE8NqtwybGhePY2gZ29ESFjqJoCu1Rupje8YtGqsefD265TMg7usUDFdp6W1EGMcet8, xpub661MyMwAqRbcFW31YEwpkMuc5THy2PSt5bDMsktWQcFF8syAmRUapSCGu8ED9W6oDMSgv6Zz8idoc4a6mr8BDzTJY47LJhkJ8UB7WEGuduB)#5jlj4shz\n");
    }

    #[test]
    fn test_script_expression_compute_and_verify() {
        get_cmd()
            .args([
                "script-expression",
                "--verify-checksum",
                "--compute-checksum",
                "raw(deadbeef)",
            ])
            .assert()
            .failure()
            .stderr(
                "Parsing error: use only '--verify-checksum' or '--compute-checksum', not both\n",
            );
    }
}
