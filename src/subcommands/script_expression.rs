use crate::structs::{
    parsing_error::ParsingError, script_expression_config::ScriptExpressionConfig,
};

use super::utils::{
    checksum::{checksum_check, checksum_create, checksum_length_check, CHECKSUM_DIVIDER_SYMBOL},
    hexadecimal::assert_hexadecimal_format,
};

pub fn script_expression(
    input: String,
    config: &ScriptExpressionConfig,
) -> Result<String, ParsingError> {
    let (script, checksum) = divide_script_and_checksum(input);
    match script.trim().chars().collect::<Vec<char>>().as_slice() {
        ['r', 'a', 'w', wrapped_hex @ .., ')'] => match wrapped_hex
            .iter()
            .collect::<String>()
            .trim()
            .chars()
            .collect::<Vec<char>>()
            .as_slice()
        {
            ['(', hex @ ..] => {
                let hex = hex.iter().collect::<String>();
                assert_hexadecimal_format(&hex, "raw function argument")?;
                script_operation(&script, &checksum, config)
            }
            _ => Err(ParsingError::new("script parsing failed!")),
        },
        _ => todo!(),
    }
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
    use crate::structs::script_expression_config::ScriptExpressionConfig;

    #[test]
    fn test_raw_script() {
        // without checksum validation and computation
        let config_with_false_compute_and_verify = ScriptExpressionConfig {
            compute_checksum: false,
            verify_checksum: false,
        };
        assert_eq!(
            script_expression(
                "raw(deadbeef)".to_string(),
                &config_with_false_compute_and_verify
            ),
            Ok("raw(deadbeef)".to_string())
        );
        assert_eq!(
            script_expression(
                "raw( deadbeef )".to_string(),
                &config_with_false_compute_and_verify
            ),
            Ok("raw( deadbeef )".to_string())
        );
        assert_eq!(
            script_expression(
                "raw(DEAD BEEF)".to_string(),
                &config_with_false_compute_and_verify
            ),
            Ok("raw(DEAD BEEF)".to_string())
        );
        assert_eq!(
            script_expression(
                "raw(DEA D BEEF)".to_string(),
                &config_with_false_compute_and_verify
            ),
            Ok("raw(DEA D BEEF)".to_string())
        );
        assert_eq!(
            script_expression("  \t\t\t  raw  \t\t\t  (  \t\t\t  D  \t\t\t  E  \t\t\t  A  \t\t\t  D  \t\t\t  )  \t\t\t  ".to_string(), &config_with_false_compute_and_verify),
            Ok("  \t\t\t  raw  \t\t\t  (  \t\t\t  D  \t\t\t  E  \t\t\t  A  \t\t\t  D  \t\t\t  )  \t\t\t  ".to_string())
        );
        assert_eq!(
            script_expression(
                "raw(nothexadecimal)".to_string(),
                &config_with_false_compute_and_verify
            ),
            Err(ParsingError::new(
                "raw function argument 'nothexadecimal' is not a valid hexadecimal string!"
            ))
        );
        assert_eq!(
            script_expression("raw()".to_string(), &config_with_false_compute_and_verify),
            Err(ParsingError::new(
                "raw function argument '' is not a valid hexadecimal string!"
            ))
        );
        assert_eq!(
            script_expression(
                "raw(deadbeef)#invalid".to_string(),
                &config_with_false_compute_and_verify
            ),
            Err(ParsingError::new("checksum length is incorrect!"))
        );
        assert_eq!(
            script_expression(
                "raw(deadbeef)#0invalid".to_string(), // despite it being incorrent, the verification is ignored, but the count of 8 chars still fits
                &config_with_false_compute_and_verify
            ),
            Ok("raw(deadbeef)#0invalid".to_string())
        );
        assert_eq!(
            script_expression("rawraw)".to_string(), &config_with_false_compute_and_verify),
            Err(ParsingError::new("script parsing failed!"))
        );

        // with verify checksum
        let config_with_true_verify = ScriptExpressionConfig {
            compute_checksum: false,
            verify_checksum: true,
        };
        assert_eq!(
            script_expression(
                "raw(deadbeef)#89f8spxm".to_string(),
                &config_with_true_verify
            ),
            Ok("Veritification of the 'raw(deadbeef)#89f8spxm' script succeeded!".to_string())
        );
        assert_eq!(
            script_expression(
                "raw( deadbeef )#985dv2zl".to_string(),
                &config_with_true_verify
            ),
            Ok("Veritification of the 'raw( deadbeef )#985dv2zl' script succeeded!".to_string())
        );
        assert_eq!(
            script_expression(
                "raw(DEAD BEEF)#qqn7ll2h".to_string(),
                &config_with_true_verify
            ),
            Ok("Veritification of the 'raw(DEAD BEEF)#qqn7ll2h' script succeeded!".to_string())
        );
        assert_eq!(
            script_expression(
                "raw(DEA D BEEF)#egs9fwsr".to_string(),
                &config_with_true_verify
            ),
            Ok("Veritification of the 'raw(DEA D BEEF)#egs9fwsr' script succeeded!".to_string())
        );
        assert_eq!(
            script_expression(
                "raw(DEA D BEEF)#agaaa9aa".to_string(),
                &config_with_true_verify
            ),
            Err(ParsingError::new("checksum verification failed!"))
        );
        assert_eq!(
            script_expression("raw(DEA D BEEF)".to_string(), &config_with_true_verify),
            Err(ParsingError::new("checksum is required for verification!"))
        );

        // with compute checksum
        let config_with_true_compute = ScriptExpressionConfig {
            compute_checksum: true,
            verify_checksum: false,
        };
        assert_eq!(
            script_expression("raw(deadbeef)".to_string(), &config_with_true_compute),
            Ok("raw(deadbeef)#89f8spxm".to_string())
        );
        assert_eq!(
            script_expression("raw( deadbeef )".to_string(), &config_with_true_compute),
            Ok("raw( deadbeef )#985dv2zl".to_string())
        );
        assert_eq!(
            script_expression("raw(DEAD BEEF)".to_string(), &config_with_true_compute),
            Ok("raw(DEAD BEEF)#qqn7ll2h".to_string())
        );
        assert_eq!(
            script_expression("raw(DEA D BEEF)".to_string(), &config_with_true_compute),
            Ok("raw(DEA D BEEF)#egs9fwsr".to_string())
        );
        assert_eq!(
            script_expression(
                "raw(deadbeef)#xxxxxxxxxxxxxxxx".to_string(),
                &config_with_true_compute
            ),
            Ok("raw(deadbeef)#89f8spxm".to_string())
        );
    }
}
