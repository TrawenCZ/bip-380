use crate::{
    structs::{parsing_error::ParsingError, script_expression_config::ScriptExpressionConfig},
    traits::string_utils::CharArrayUtils,
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
    input: String,
    config: &ScriptExpressionConfig,
) -> Result<String, ParsingError> {
    let (script, checksum) = divide_script_and_checksum(input);
    match script.trim().chars().collect::<Vec<char>>().as_slice() {
        ['r', 'a', 'w', rest @ ..] => match rest.extract_args("raw")?.as_slice() {
            [arg] => {
                assert_hexadecimal_format(arg, "raw function argument")?;
            }
            _ => return Err(ParsingError::new("script parsing failed!")),
        },
        ['m', 'u', 'l', 't', 'i', rest @ ..] => match rest.extract_args("multi")?.as_slice() {
            [arg_count, rest_of_args @ ..] => match arg_count.parse::<i32>()? {
                val if val >= 0 && val <= rest_of_args.len() as i32 => {
                    for arg in rest_of_args {
                        validate_key_expression(arg.clone())?; // TODO: Try to avoid clone?
                    }
                }
                val if val < 0 => {
                    return Err(ParsingError::new("arg count indicator cannot be negative"))
                }
                _ => {
                    return Err(ParsingError::new(
                        "arg count indicator cannot be higher than actual args count",
                    ))
                }
            },
            _ => return Err(ParsingError::new("at least two arguments needed")),
        },
        ['p', 'k', 'h', rest @ ..] => match rest.extract_args("pk")?.as_slice() {
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
                script_expression(arg.clone(), &ScriptExpressionConfig::default())?;
            }
            [arg] => return Err(ParsingError::new(&script_sh_unsupported_arg_err(arg))),
            _ => {
                return Err(ParsingError::new(
                    "exactly one argument is needed for sh script",
                ))
            }
        },
        _ => todo!(),
    }
    script_operation(&script, &checksum, config)
}

fn divide_script_and_checksum(input: String) -> (String, Option<String>) {
    let parts: Vec<&str> = input.splitn(2, CHECKSUM_DIVIDER_SYMBOL).collect();
    let script = parts.first().map_or("", |v| v).to_string();
    let checksum = parts.get(1).map(|s| s.to_string());
    (script, checksum)
}

fn script_operation(
    script: &str,
    checksum: &Option<String>,
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
                            "Veritification of the '{}#{}' script succeeded!",
                            script, checksum
                        ))
                    } else {
                        Err(ParsingError::new("checksum verification failed!"))
                    }
                } else {
                    Ok(format!("{}#{}", script, checksum))
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
        structs::script_expression_config::ScriptExpressionConfig,
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
            script_expression(
                "raw(deadbeef)".to_string(),
                &CONFIG_WITH_FALSE_COMPUTE_AND_VERIFY
            ),
            Ok("raw(deadbeef)".to_string())
        );
        assert_eq!(
            script_expression(
                "raw( deadbeef )".to_string(),
                &CONFIG_WITH_FALSE_COMPUTE_AND_VERIFY
            ),
            Ok("raw( deadbeef )".to_string())
        );
        assert_eq!(
            script_expression(
                "raw(DEAD BEEF)".to_string(),
                &CONFIG_WITH_FALSE_COMPUTE_AND_VERIFY
            ),
            Ok("raw(DEAD BEEF)".to_string())
        );
        assert_eq!(
            script_expression(
                "raw(DEA D BEEF)".to_string(),
                &CONFIG_WITH_FALSE_COMPUTE_AND_VERIFY
            ),
            Ok("raw(DEA D BEEF)".to_string())
        );
        assert_eq!(
            script_expression("  \t\t\t  raw  \t\t\t  (  \t\t\t  D  \t\t\t  E  \t\t\t  A  \t\t\t  D  \t\t\t  )  \t\t\t  ".to_string(), &CONFIG_WITH_FALSE_COMPUTE_AND_VERIFY),
            Ok("  \t\t\t  raw  \t\t\t  (  \t\t\t  D  \t\t\t  E  \t\t\t  A  \t\t\t  D  \t\t\t  )  \t\t\t  ".to_string())
        );
        assert_eq!(
            script_expression(
                "raw(nothexadecimal)".to_string(),
                &CONFIG_WITH_FALSE_COMPUTE_AND_VERIFY
            ),
            Err(ParsingError::new(
                "raw function argument 'nothexadecimal' is not a valid hexadecimal string!"
            ))
        );
        assert_eq!(
            script_expression("raw()".to_string(), &CONFIG_WITH_FALSE_COMPUTE_AND_VERIFY),
            Err(ParsingError::new(
                "raw function argument '' is not a valid hexadecimal string!"
            ))
        );
        assert_eq!(
            script_expression(
                "raw(deadbeef)#invalid".to_string(),
                &CONFIG_WITH_FALSE_COMPUTE_AND_VERIFY
            ),
            Err(ParsingError::new("checksum length is incorrect!"))
        );
        assert_eq!(
            script_expression(
                "raw(deadbeef)#0invalid".to_string(), // despite it being incorrent, the verification is ignored, but the count of 8 chars still fits
                &CONFIG_WITH_FALSE_COMPUTE_AND_VERIFY
            ),
            Ok("raw(deadbeef)#0invalid".to_string())
        );
        assert_eq!(
            script_expression("rawraw)".to_string(), &CONFIG_WITH_FALSE_COMPUTE_AND_VERIFY),
            Err(ParsingError::new(&script_arg_extraction_err("raw")))
        );

        assert_eq!(
            script_expression(
                "raw(deadbeef)#89f8spxm".to_string(),
                &CONFIG_WITH_TRUE_VERIFY
            ),
            Ok("Veritification of the 'raw(deadbeef)#89f8spxm' script succeeded!".to_string())
        );
        assert_eq!(
            script_expression(
                "raw( deadbeef )#985dv2zl".to_string(),
                &CONFIG_WITH_TRUE_VERIFY
            ),
            Ok("Veritification of the 'raw( deadbeef )#985dv2zl' script succeeded!".to_string())
        );
        assert_eq!(
            script_expression(
                "raw(DEAD BEEF)#qqn7ll2h".to_string(),
                &CONFIG_WITH_TRUE_VERIFY
            ),
            Ok("Veritification of the 'raw(DEAD BEEF)#qqn7ll2h' script succeeded!".to_string())
        );
        assert_eq!(
            script_expression(
                "raw(DEA D BEEF)#egs9fwsr".to_string(),
                &CONFIG_WITH_TRUE_VERIFY
            ),
            Ok("Veritification of the 'raw(DEA D BEEF)#egs9fwsr' script succeeded!".to_string())
        );
        assert_eq!(
            script_expression(
                "raw(DEA D BEEF)#agaaa9aa".to_string(),
                &CONFIG_WITH_TRUE_VERIFY
            ),
            Err(ParsingError::new("checksum verification failed!"))
        );
        assert_eq!(
            script_expression("raw(DEA D BEEF)".to_string(), &CONFIG_WITH_TRUE_VERIFY),
            Err(ParsingError::new("checksum is required for verification!"))
        );

        assert_eq!(
            script_expression("raw(deadbeef)".to_string(), &CONFIG_WITH_TRUE_COMPUTE),
            Ok("raw(deadbeef)#89f8spxm".to_string())
        );
        assert_eq!(
            script_expression("raw( deadbeef )".to_string(), &CONFIG_WITH_TRUE_COMPUTE),
            Ok("raw( deadbeef )#985dv2zl".to_string())
        );
        assert_eq!(
            script_expression("raw(DEAD BEEF)".to_string(), &CONFIG_WITH_TRUE_COMPUTE),
            Ok("raw(DEAD BEEF)#qqn7ll2h".to_string())
        );
        assert_eq!(
            script_expression("raw(DEA D BEEF)".to_string(), &CONFIG_WITH_TRUE_COMPUTE),
            Ok("raw(DEA D BEEF)#egs9fwsr".to_string())
        );
        assert_eq!(
            script_expression(
                "raw(deadbeef)#xxxxxxxxxxxxxxxx".to_string(),
                &CONFIG_WITH_TRUE_COMPUTE
            ),
            Ok("raw(deadbeef)#89f8spxm".to_string())
        );
    }

    #[test]
    fn test_multi_script() {
        assert_eq!(script_expression("multi(2, xpub661MyMwAqRbcFtXgS5sYJABqqG9YLmC4Q1Rdap9gSE8NqtwybGhePY2gZ29ESFjqJoCu1Rupje8YtGqsefD265TMg7usUDFdp6W1EGMcet8, xpub661MyMwAqRbcFW31YEwpkMuc5THy2PSt5bDMsktWQcFF8syAmRUapSCGu8ED9W6oDMSgv6Zz8idoc4a6mr8BDzTJY47LJhkJ8UB7WEGuduB)#5jlj4shz".to_string(), &CONFIG_WITH_TRUE_COMPUTE), Ok("multi(2, xpub661MyMwAqRbcFtXgS5sYJABqqG9YLmC4Q1Rdap9gSE8NqtwybGhePY2gZ29ESFjqJoCu1Rupje8YtGqsefD265TMg7usUDFdp6W1EGMcet8, xpub661MyMwAqRbcFW31YEwpkMuc5THy2PSt5bDMsktWQcFF8syAmRUapSCGu8ED9W6oDMSgv6Zz8idoc4a6mr8BDzTJY47LJhkJ8UB7WEGuduB)#5jlj4shz".to_string()))
    }

    #[test]
    fn test_pkh_script() {
        assert_eq!(script_expression("pkh(xpub661MyMwAqRbcFtXgS5sYJABqqG9YLmC4Q1Rdap9gSE8NqtwybGhePY2gZ29ESFjqJoCu1Rupje8YtGqsefD265TMg7usUDFdp6W1EGMcet8)".to_string(), &CONFIG_WITH_TRUE_COMPUTE), Ok("pkh(xpub661MyMwAqRbcFtXgS5sYJABqqG9YLmC4Q1Rdap9gSE8NqtwybGhePY2gZ29ESFjqJoCu1Rupje8YtGqsefD265TMg7usUDFdp6W1EGMcet8)#vm4xc4ed".to_string()));

        assert_eq!(script_expression("pkh(   xpub661MyMwAqRbcFtXgS5sYJABqqG9YLmC4Q1Rdap9gSE8NqtwybGhePY2gZ29ESFjqJoCu1Rupje8YtGqsefD265TMg7usUDFdp6W1EGMcet8)".to_string(), &CONFIG_WITH_TRUE_COMPUTE), Ok("pkh(   xpub661MyMwAqRbcFtXgS5sYJABqqG9YLmC4Q1Rdap9gSE8NqtwybGhePY2gZ29ESFjqJoCu1Rupje8YtGqsefD265TMg7usUDFdp6W1EGMcet8)#ujpe9npc".to_string()))
    }

    #[test]
    fn test_sh_script() {
        assert_eq!(script_expression("sh(multi(2, xpub661MyMwAqRbcFtXgS5sYJABqqG9YLmC4Q1Rdap9gSE8NqtwybGhePY2gZ29ESFjqJoCu1Rupje8YtGqsefD265TMg7usUDFdp6W1EGMcet8, xpub661MyMwAqRbcFW31YEwpkMuc5THy2PSt5bDMsktWQcFF8syAmRUapSCGu8ED9W6oDMSgv6Zz8idoc4a6mr8BDzTJY47LJhkJ8UB7WEGuduB))#3txhxflq".to_string(), &CONFIG_WITH_TRUE_COMPUTE), Ok("sh(multi(2, xpub661MyMwAqRbcFtXgS5sYJABqqG9YLmC4Q1Rdap9gSE8NqtwybGhePY2gZ29ESFjqJoCu1Rupje8YtGqsefD265TMg7usUDFdp6W1EGMcet8, xpub661MyMwAqRbcFW31YEwpkMuc5THy2PSt5bDMsktWQcFF8syAmRUapSCGu8ED9W6oDMSgv6Zz8idoc4a6mr8BDzTJY47LJhkJ8UB7WEGuduB))#3txhxflq".to_string()));

        assert_eq!(
            script_expression(
                "sh(raw(deadbeef))".to_string(),
                &CONFIG_WITH_FALSE_COMPUTE_AND_VERIFY
            ),
            Err(ParsingError::new(&script_sh_unsupported_arg_err(
                "raw(deadbeef)"
            )))
        )
    }
}
