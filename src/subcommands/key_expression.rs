use std::str::FromStr;

use bip32::ExtendedKey;

use crate::structs::{key_expression_config::KeyExpressionConfig, parsing_error::ParsingError};
use crate::subcommands::utils::{
    extended_key, hex_encoded_public_key, key_origin, wallet_import_format,
};

use super::utils::extended_key::{has_extended_key_prefix, validate_extended_key_attrs};
use super::utils::hex_encoded_public_key::has_hex_encoded_public_key_prefix;

const ALLOWED_CHAR_SET: &str =
    "0123456789()[],'/*abcdefgh@:$%{}IJKLMNOPQRSTUVWXYZ&+-.;<=>?!^_|~ijklmnopqrstuvwxyzABCDEFGH`# ";

pub fn key_expression(
    input: String,
    _config: &KeyExpressionConfig,
) -> Result<String, ParsingError> {
    validate_key_expression(input)
}

pub fn validate_key_expression(input: String) -> Result<String, ParsingError> {
    if input.is_empty() {
        return Err(ParsingError::new("Input is empty"));
    }

    if input.chars().any(|c| !ALLOWED_CHAR_SET.contains(c)) {
        return Err(ParsingError::new("Input contains invalid characters"));
    }

    let (key_origin, key) = split_key_expression(input.as_str())?;

    if let Some(key_origin) = key_origin {
        key_origin::validate_key_origin(key_origin)?;
    }

    validate_key(key)?;

    Ok(input)
}

fn validate_key(key: &str) -> Result<(), ParsingError> {
    if key.is_empty() {
        return Err(ParsingError::new("Key is empty"));
    }

    if key.contains('[') || key.contains(']') {
        return Err(ParsingError::new("Key can not include key origin"));
    }

    if has_hex_encoded_public_key_prefix(key) {
        hex_encoded_public_key::parse_hex_encoded_public_key(key)?;
    } else if has_extended_key_prefix(key) {
        let key_str = extended_key::validate_extended_key(key)?;
        let key = ExtendedKey::from_str(&key_str)?;
        validate_extended_key_attrs(&key.attrs)?;
    } else {
        wallet_import_format::validate_wif_private_key(key)?;
    }

    Ok(())
}

/// Split the key expression subcommand input into key origin and key
fn split_key_expression(input: &str) -> Result<(Option<&str>, &str), ParsingError> {
    if input.starts_with('[') {
        let end_index = input
            .find(']')
            .ok_or_else(|| ParsingError::new("Missing closing bracket"))?;

        // split the input into key origin and key
        let (key_origin, key) = input.split_at(end_index + 1);

        return Ok((Some(key_origin), key));
    }

    Ok((None, input))
}

#[cfg(test)]
mod tests {
    use crate::test_utils::get_cmd;

    use super::*;

    #[test]
    fn test_validate_key_origin_valid_bip_380() {
        // test vectors from bip380 specification
        // https://github.com/bitcoin/bips/blob/master/bip-0380.mediawiki#test-vectors
        let valid_key_origins = [
            "0260b2003c386519fc9eadf2b5cf124dd8eea4c4e68d5e154050a9346ea98ce600",
            "[deadbeef/0h/1h/2]0260b2003c386519fc9eadf2b5cf124dd8eea4c4e68d5e154050a9346ea98ce600",
            "[deadbeef/0h/1h/2]xprvA1RpRA33e1JQ7ifknakTFpgNXPmW2YvmhqLQYMmrj4xJXXWYpDPS3xz7iAxn8L39njGVyuoseXzU6rcxFLJ8HFsTjSyQbLYnMpCqE2VbFWc/3h/4h/5h/*h",
            "0260b2003c386519fc9eadf2b5cf124dd8eea4c4e68d5e154050a9346ea98ce600",
            "04a34b99f22c790c4e36b2b3c2c35a36db06226e41c692fc82b8b56ac1c540c5bd5b8dec5235a0fa8722476c7709c02559e3aa73aa03918ba2d492eea75abea235",
            "[deadbeef/0h/0h/0h]0260b2003c386519fc9eadf2b5cf124dd8eea4c4e68d5e154050a9346ea98ce600",
            "[deadbeef/0'/0'/0']0260b2003c386519fc9eadf2b5cf124dd8eea4c4e68d5e154050a9346ea98ce600",
            "[deadbeef/0'/0h/0']0260b2003c386519fc9eadf2b5cf124dd8eea4c4e68d5e154050a9346ea98ce600",
            "5KYZdUEo39z3FPrtuX2QbbwGnNP5zTd7yyr2SC1j299sBCnWjss",
            "L4rK1yDtCWekvXuE6oXD9jCYfFNV2cWRpVuPLBcCU2z8TrisoyY1",
            "xpub6ERApfZwUNrhLCkDtcHTcxd75RbzS1ed54G1LkBUHQVHQKqhMkhgbmJbZRkrgZw4koxb5JaHWkY4ALHY2grBGRjaDMzQLcgJvLJuZZvRcEL",
            "[deadbeef/0h/1h/2h]xpub6ERApfZwUNrhLCkDtcHTcxd75RbzS1ed54G1LkBUHQVHQKqhMkhgbmJbZRkrgZw4koxb5JaHWkY4ALHY2grBGRjaDMzQLcgJvLJuZZvRcEL",
            "[deadbeef/0h/1h/2h]xpub6ERApfZwUNrhLCkDtcHTcxd75RbzS1ed54G1LkBUHQVHQKqhMkhgbmJbZRkrgZw4koxb5JaHWkY4ALHY2grBGRjaDMzQLcgJvLJuZZvRcEL/3/4/5",
            "[deadbeef/0h/1h/2h]xpub6ERApfZwUNrhLCkDtcHTcxd75RbzS1ed54G1LkBUHQVHQKqhMkhgbmJbZRkrgZw4koxb5JaHWkY4ALHY2grBGRjaDMzQLcgJvLJuZZvRcEL/3/4/5/*",
            "xpub6ERApfZwUNrhLCkDtcHTcxd75RbzS1ed54G1LkBUHQVHQKqhMkhgbmJbZRkrgZw4koxb5JaHWkY4ALHY2grBGRjaDMzQLcgJvLJuZZvRcEL/3h/4h/5h/*",
            "xpub6ERApfZwUNrhLCkDtcHTcxd75RbzS1ed54G1LkBUHQVHQKqhMkhgbmJbZRkrgZw4koxb5JaHWkY4ALHY2grBGRjaDMzQLcgJvLJuZZvRcEL/3h/4h/5h/*h",
            "[deadbeef/0h/1h/2]xpub6ERApfZwUNrhLCkDtcHTcxd75RbzS1ed54G1LkBUHQVHQKqhMkhgbmJbZRkrgZw4koxb5JaHWkY4ALHY2grBGRjaDMzQLcgJvLJuZZvRcEL/3h/4h/5h/*h",
            "xprvA1RpRA33e1JQ7ifknakTFpgNXPmW2YvmhqLQYMmrj4xJXXWYpDPS3xz7iAxn8L39njGVyuoseXzU6rcxFLJ8HFsTjSyQbLYnMpCqE2VbFWc",
            "[deadbeef/0h/1h/2h]xprvA1RpRA33e1JQ7ifknakTFpgNXPmW2YvmhqLQYMmrj4xJXXWYpDPS3xz7iAxn8L39njGVyuoseXzU6rcxFLJ8HFsTjSyQbLYnMpCqE2VbFWc",
            "[deadbeef/0h/1h/2h]xprvA1RpRA33e1JQ7ifknakTFpgNXPmW2YvmhqLQYMmrj4xJXXWYpDPS3xz7iAxn8L39njGVyuoseXzU6rcxFLJ8HFsTjSyQbLYnMpCqE2VbFWc/3/4/5",
            "[deadbeef/0h/1h/2h]xprvA1RpRA33e1JQ7ifknakTFpgNXPmW2YvmhqLQYMmrj4xJXXWYpDPS3xz7iAxn8L39njGVyuoseXzU6rcxFLJ8HFsTjSyQbLYnMpCqE2VbFWc/3/4/5/*",
            "xprvA1RpRA33e1JQ7ifknakTFpgNXPmW2YvmhqLQYMmrj4xJXXWYpDPS3xz7iAxn8L39njGVyuoseXzU6rcxFLJ8HFsTjSyQbLYnMpCqE2VbFWc/3h/4h/5h/*",
            "xprvA1RpRA33e1JQ7ifknakTFpgNXPmW2YvmhqLQYMmrj4xJXXWYpDPS3xz7iAxn8L39njGVyuoseXzU6rcxFLJ8HFsTjSyQbLYnMpCqE2VbFWc/3h/4h/5h/*h",
            "[deadbeef/0h/1h/2]xprvA1RpRA33e1JQ7ifknakTFpgNXPmW2YvmhqLQYMmrj4xJXXWYpDPS3xz7iAxn8L39njGVyuoseXzU6rcxFLJ8HFsTjSyQbLYnMpCqE2VbFWc/3h/4h/5h/*h",
            "[deadbeef/0H/0H/0H]0260b2003c386519fc9eadf2b5cf124dd8eea4c4e68d5e154050a9346ea98ce600",
        ];

        for &key_origin in &valid_key_origins {
            let result = validate_key_expression(key_origin.to_string());
            println!("{:?}", result);
            assert!(
                result.is_ok(),
                "Expected valid key origin '{}' to pass validation",
                key_origin
            );
        }
    }

    #[test]
    fn test_validate_key_origin_invalid_bip_380() {
        // test vectors from bip380 specification
        // https://github.com/bitcoin/bips/blob/master/bip-0380.mediawiki#test-vectors
        let invalid_key_origins = [
            "[deadbeef/0h/0h/0h/*]0260b2003c386519fc9eadf2b5cf124dd8eea4c4e68d5e154050a9346ea98ce600",
            "[deadbeef/0h/0h/0h/]0260b2003c386519fc9eadf2b5cf124dd8eea4c4e68d5e154050a9346ea98ce600",
            "[deadbef/0h/0h/0h]0260b2003c386519fc9eadf2b5cf124dd8eea4c4e68d5e154050a9346ea98ce600",
            "[deadbeeef/0h/0h/0h]0260b2003c386519fc9eadf2b5cf124dd8eea4c4e68d5e154050a9346ea98ce600",
            "[deadbeef/0f/0f/0f]0260b2003c386519fc9eadf2b5cf124dd8eea4c4e68d5e154050a9346ea98ce600",
            "[deadbeef/-0/-0/-0]0260b2003c386519fc9eadf2b5cf124dd8eea4c4e68d5e154050a9346ea98ce600",
            "L4rK1yDtCWekvXuE6oXD9jCYfFNV2cWRpVuPLBcCU2z8TrisoyY1/0",
            "L4rK1yDtCWekvXuE6oXD9jCYfFNV2cWRpVuPLBcCU2z8TrisoyY1/*",
            "xprv9s21ZrQH143K31xYSDQpPDxsXRTUcvj2iNHm5NUtrGiGG5e2DtALGdso3pGz6ssrdK4PFmM8NSpSBHNqPqm55Qn3LqFtT2emdEXVYsCzC2U/2147483648",
            "xprv9s21ZrQH143K31xYSDQpPDxsXRTUcvj2iNHm5NUtrGiGG5e2DtALGdso3pGz6ssrdK4PFmM8NSpSBHNqPqm55Qn3LqFtT2emdEXVYsCzC2U/1aa",
            "[aaaaaaaa][aaaaaaaa]xprv9s21ZrQH143K31xYSDQpPDxsXRTUcvj2iNHm5NUtrGiGG5e2DtALGdso3pGz6ssrdK4PFmM8NSpSBHNqPqm55Qn3LqFtT2emdEXVYsCzC2U/2147483647'/0",
            "aaaaaaaa]xprv9s21ZrQH143K31xYSDQpPDxsXRTUcvj2iNHm5NUtrGiGG5e2DtALGdso3pGz6ssrdK4PFmM8NSpSBHNqPqm55Qn3LqFtT2emdEXVYsCzC2U/2147483647'/0",
            "[gaaaaaaa]xprv9s21ZrQH143K31xYSDQpPDxsXRTUcvj2iNHm5NUtrGiGG5e2DtALGdso3pGz6ssrdK4PFmM8NSpSBHNqPqm55Qn3LqFtT2emdEXVYsCzC2U/2147483647'/0",
            "[deadbeef]"
        ];

        for &key_origin in &invalid_key_origins {
            let result = validate_key_expression(key_origin.to_string());
            assert!(
                result.is_err(),
                "Expected invalid key origin '{}' not to pass validation",
                key_origin
            );
        }
    }

    // integration test
    #[test]
    fn test_key_expression() {
        let input_string = "0260b2003c386519fc9eadf2b5cf124dd8eea4c4e68d5e154050a9346ea98ce600";
        get_cmd()
            .args(["key-expression", input_string])
            .assert()
            .success()
            .stdout(format!("{input_string}\n"));

        let input_string = "[deadbeef/0h/1h/2]xprvA1RpRA33e1JQ7ifknakTFpgNXPmW2YvmhqLQYMmrj4xJXXWYpDPS3xz7iAxn8L39njGVyuoseXzU6rcxFLJ8HFsTjSyQbLYnMpCqE2VbFWc/3h/4h/5h/*h";
        get_cmd()
            .args(["key-expression", input_string])
            .assert()
            .success()
            .stdout(format!("{input_string}\n"));
    }

    #[test]
    fn valid_compressed_public_key() {
        let result = validate_key_expression(
            "0260b2003c386519fc9eadf2b5cf124dd8eea4c4e68d5e154050a9346ea98ce600".into(),
        );
        assert!(result.is_ok());
    }

    #[test]
    fn valid_compressed_public_key_other_prefix() {
        let result = validate_key_expression(
            "0360b2003c386519fc9eadf2b5cf124dd8eea4c4e68d5e154050a9346ea98ce600".into(),
        );
        assert!(result.is_ok());
    }

    #[test]
    fn invalid_compressed_public_too_short() {
        let result = validate_key_expression(
            "0360b2003c386519fc9eadf2b5cf124dd8eea4c4e68d5e154050a9346ea98ce60".into(),
        );
        assert!(result.is_err());
    }

    #[test]
    fn invalid_compressed_public_too_long() {
        let result = validate_key_expression(
            "0360b2003c386519fc9eadf2b5cf124dd8eea4c4e68d5e154050a9346ea98ce6000".into(),
        );
        assert!(result.is_err());
    }

    #[test]
    fn valid_uncompressed_public_key() {
        let result = validate_key_expression(
            "04a34b99f22c790c4e36b2b3c2c35a36db06226e41c692fc82b8b56ac1c540c5bd5b8dec5235a0fa8722476c7709c02559e3aa73aa03918ba2d492eea75abea235".into()
        );
        assert!(result.is_ok());
    }

    #[test]
    fn valid_extended_public_key() {
        let result = validate_key_expression("xpub6ERApfZwUNrhLCkDtcHTcxd75RbzS1ed54G1LkBUHQVHQKqhMkhgbmJbZRkrgZw4koxb5JaHWkY4ALHY2grBGRjaDMzQLcgJvLJuZZvRcEL".into());
        assert!(result.is_ok());
    }

    #[test]
    fn valid_extended_public_key_with_derivation() {
        let result = validate_key_expression("xpub6ERApfZwUNrhLCkDtcHTcxd75RbzS1ed54G1LkBUHQVHQKqhMkhgbmJbZRkrgZw4koxb5JaHWkY4ALHY2grBGRjaDMzQLcgJvLJuZZvRcEL/3/4/5".into());
        assert!(result.is_ok());
    }

    #[test]
    fn valid_extended_public_key_with_derivation_and_children() {
        let result = validate_key_expression("xpub6ERApfZwUNrhLCkDtcHTcxd75RbzS1ed54G1LkBUHQVHQKqhMkhgbmJbZRkrgZw4koxb5JaHWkY4ALHY2grBGRjaDMzQLcgJvLJuZZvRcEL/3/4/5/*".into());
        assert!(result.is_ok());
    }

    #[test]
    fn valid_extended_public_key_with_hardened_derivation_and_unhardened_children() {
        let result = validate_key_expression("xpub6ERApfZwUNrhLCkDtcHTcxd75RbzS1ed54G1LkBUHQVHQKqhMkhgbmJbZRkrgZw4koxb5JaHWkY4ALHY2grBGRjaDMzQLcgJvLJuZZvRcEL/3h/4h/5h/*".into());
        assert!(result.is_ok());
    }

    #[test]
    fn valid_extended_public_key_with_hardened_derivation_and_children() {
        let result = validate_key_expression("xpub6ERApfZwUNrhLCkDtcHTcxd75RbzS1ed54G1LkBUHQVHQKqhMkhgbmJbZRkrgZw4koxb5JaHWkY4ALHY2grBGRjaDMzQLcgJvLJuZZvRcEL/3h/4h/5h/*h".into());
        assert!(result.is_ok());
    }

    #[test]
    fn valid_extended_private_key() {
        let result = validate_key_expression("xprvA1RpRA33e1JQ7ifknakTFpgNXPmW2YvmhqLQYMmrj4xJXXWYpDPS3xz7iAxn8L39njGVyuoseXzU6rcxFLJ8HFsTjSyQbLYnMpCqE2VbFWc".into());
        assert!(result.is_ok());
    }

    #[test]
    fn valid_extended_private_key_with_derivation() {
        let result = validate_key_expression("xprvA1RpRA33e1JQ7ifknakTFpgNXPmW2YvmhqLQYMmrj4xJXXWYpDPS3xz7iAxn8L39njGVyuoseXzU6rcxFLJ8HFsTjSyQbLYnMpCqE2VbFWc/3/4/5".into());
        assert!(result.is_ok());
    }

    #[test]
    fn valid_extended_private_key_with_derivation_and_children() {
        let result = validate_key_expression("xprvA1RpRA33e1JQ7ifknakTFpgNXPmW2YvmhqLQYMmrj4xJXXWYpDPS3xz7iAxn8L39njGVyuoseXzU6rcxFLJ8HFsTjSyQbLYnMpCqE2VbFWc/3/4/5/*".into());
        assert!(result.is_ok());
    }

    #[test]
    fn valid_extended_private_key_with_hardened_derivation_and_unhardened_children() {
        let result = validate_key_expression("xprvA1RpRA33e1JQ7ifknakTFpgNXPmW2YvmhqLQYMmrj4xJXXWYpDPS3xz7iAxn8L39njGVyuoseXzU6rcxFLJ8HFsTjSyQbLYnMpCqE2VbFWc/3h/4h/5h/*".into());
        assert!(result.is_ok());
    }

    #[test]
    fn valid_extended_private_key_with_hardened_derivation_and_children() {
        let result = validate_key_expression("xprvA1RpRA33e1JQ7ifknakTFpgNXPmW2YvmhqLQYMmrj4xJXXWYpDPS3xz7iAxn8L39njGVyuoseXzU6rcxFLJ8HFsTjSyQbLYnMpCqE2VbFWc/3h/4h/5h/*h".into());
        assert!(result.is_ok());
    }

    #[test]
    fn invalid_extended_key_random_bytes() {
        let result = validate_key_expression("xprv123".into());
        assert!(result.is_err());
    }

    #[test]
    fn invalid_derivation_index_out_of_range() {
        let result = validate_key_expression("xprv9s21ZrQH143K31xYSDQpPDxsXRTUcvj2iNHm5NUtrGiGG5e2DtALGdso3pGz6ssrdK4PFmM8NSpSBHNqPqm55Qn3LqFtT2emdEXVYsCzC2U/2147483648".into());
        assert!(result.is_err());
    }

    #[test]
    fn valid_wif_uncompressed() {
        let result =
            validate_key_expression("5KYZdUEo39z3FPrtuX2QbbwGnNP5zTd7yyr2SC1j299sBCnWjss".into());
        assert!(result.is_ok());
    }

    #[test]
    fn valid_wif_compressed() {
        let result =
            validate_key_expression("L4rK1yDtCWekvXuE6oXD9jCYfFNV2cWRpVuPLBcCU2z8TrisoyY1".into());
        assert!(result.is_ok());
    }

    #[test]
    fn wif_invalid_checksum() {
        let result =
            validate_key_expression("L2yR7WsFoYmeZqch8ScZ6J2YqrJ7N9JGVd56jz17WAWWm3coAJza".into());
        assert!(result.is_err());
    }

    #[test]
    fn wif_invalid_prefix() {
        let result =
            validate_key_expression("5wGuZuPGhWjR8d1J3zfVFS6c1tM1gKZX2VZeu4fz248QepEppupV".into());
        assert!(result.is_err());
    }

    #[test]
    fn xpub_zero_depth_with_non_zero_parent_fingerprint_invalid() {
        let result = validate_key_expression("xpub661no6RGEX3uJkY4bNnPcw4URcQTrSibUZ4NqJEw5eBkv7ovTwgiT91XX27VbEXGENhYRCf7hyEbWrR3FewATdCEebj6znwMfQkhRYHRLpJ".into());
        assert!(result.is_err());
    }

    #[test]
    fn xpub_zero_depth_with_non_zero_index_invalid() {
        let result = validate_key_expression("xpub661MyMwAuDcm6CRQ5N4qiHKrJ39Xe1R1NyfouMKTTWcguwVcfrZJaNvhpebzGerh7gucBvzEQWRugZDuDXjNDRmXzSZe4c7mnTK97pTvGS8".into());
        assert!(result.is_err());
    }
    #[test]
    fn xprv_zero_depth_non_zero_parent_invalid() {
        let result = validate_key_expression("xprv9s2SPatNQ9Vc6GTbVMFPFo7jsaZySyzk7L8n2uqKXJen3KUmvQNTuLh3fhZMBoG3G4ZW1N2kZuHEPY53qmbZzCHshoQnNf4GvELZfqTUrcv".into());
        assert!(result.is_err());
    }

    #[test]
    fn xprv_zero_depth_non_zero_index_invalid() {
        let result = validate_key_expression("xprv9s21ZrQH4r4TsiLvyLXqM9P7k1K3EYhA1kkD6xuquB5i39AU8KF42acDyL3qsDbU9NmZn6MsGSUYZEsuoePmjzsB3eFKSUEh3Gu1N3cqVUN".into());
        assert!(result.is_err());
    }

    #[test]
    fn valid_public_key_02() {
        let result = validate_key_expression(
            "0260b2003c386519fc9eadf2b5cf124dd8eea4c4e68d5e154050a9346ea98ce600".into(),
        );
        assert!(result.is_ok());
    }

    #[test]
    fn valid_public_key_03() {
        let result = validate_key_expression(
            "0360b2003c386519fc9eadf2b5cf124dd8eea4c4e68d5e154050a9346ea98ce600".into(),
        );
        assert!(result.is_ok());
    }
    #[test]
    fn valid_public_key_04() {
        let result = validate_key_expression("04a34b99f22c790c4e36b2b3c2c35a36db06226e41c692fc82b8b56ac1c540c5bd5b8dec5235a0fa8722476c7709c02559e3aa73aa03918ba2d492eea75abea235".into());
        assert!(result.is_ok());
    }

    #[test]
    fn valid_public_key_with_origin() {
        let result = validate_key_expression(
            "[deadbeef/0h/0h/0h]0260b2003c386519fc9eadf2b5cf124dd8eea4c4e68d5e154050a9346ea98ce600"
                .into(),
        );
        assert!(result.is_ok());
    }

    #[test]
    fn valid_public_key_with_origin_other_indicator() {
        let result = validate_key_expression(
            "[deadbeef/0'/0'/0']0260b2003c386519fc9eadf2b5cf124dd8eea4c4e68d5e154050a9346ea98ce600"
                .into(),
        );
        assert!(result.is_ok());
    }

    #[test]
    fn valid_public_key_with_origin_alternating_indicators() {
        let result = validate_key_expression(
            "[deadbeef/0'/0h/0']0260b2003c386519fc9eadf2b5cf124dd8eea4c4e68d5e154050a9346ea98ce600"
                .into(),
        );
        assert!(result.is_ok());
    }

    #[test]
    fn valid_wif_key() {
        let result =
            validate_key_expression("5KYZdUEo39z3FPrtuX2QbbwGnNP5zTd7yyr2SC1j299sBCnWjss".into());
        assert!(result.is_ok());
    }

    #[test]
    fn valid_public_extended_key_with_every_optional_part() {
        let result = validate_key_expression("[deadbeef/0h/1h/2]xpub6ERApfZwUNrhLCkDtcHTcxd75RbzS1ed54G1LkBUHQVHQKqhMkhgbmJbZRkrgZw4koxb5JaHWkY4ALHY2grBGRjaDMzQLcgJvLJuZZvRcEL/3h/4h/5h/*h".into());
        assert!(result.is_ok());
    }

    #[test]
    fn valid_private_extended_key_with_every_optional_part() {
        let result = validate_key_expression("[deadbeef/0h/1h/2]xprvA1RpRA33e1JQ7ifknakTFpgNXPmW2YvmhqLQYMmrj4xJXXWYpDPS3xz7iAxn8L39njGVyuoseXzU6rcxFLJ8HFsTjSyQbLYnMpCqE2VbFWc/3h/4h/5h/*h".into());
        assert!(result.is_ok());
    }

    #[test]
    fn invalid_empty() {
        let result = validate_key_expression("".into());
        assert!(result.is_err());
    }

    #[test]
    fn invalid_no_key() {
        let result = validate_key_expression("[deadbeef]".into());
        assert!(result.is_err());
    }

    #[test]
    fn invalid_origin_too_short_fingerprint() {
        let result = validate_key_expression(
            "[deadbef/0h/0h/0h]0260b2003c386519fc9eadf2b5cf124dd8eea4c4e68d5e154050a9346ea98ce600"
                .into(),
        );
        assert!(result.is_err());
    }

    #[test]
    fn invalid_multiple_origins() {
        let result = validate_key_expression("[aaaaaaaa][aaaaaaaa]xprv9s21ZrQH143K31xYSDQpPDxsXRTUcvj2iNHm5NUtrGiGG5e2DtALGdso3pGz6ssrdK4PFmM8NSpSBHNqPqm55Qn3LqFtT2emdEXVYsCzC2U/2147483647'/0".into());
        assert!(result.is_err());
    }

    #[test]
    fn invalid_missing_origin_start() {
        let result = validate_key_expression("aaaaaaaa]xprv9s21ZrQH143K31xYSDQpPDxsXRTUcvj2iNHm5NUtrGiGG5e2DtALGdso3pGz6ssrdK4PFmM8NSpSBHNqPqm55Qn3LqFtT2emdEXVYsCzC2U/2147483647'/0".into());
        assert!(result.is_err());
    }

    #[test]
    fn invalid_origin_non_hex() {
        let result = validate_key_expression("[gaaaaaaa]xprv9s21ZrQH143K31xYSDQpPDxsXRTUcvj2iNHm5NUtrGiGG5e2DtALGdso3pGz6ssrdK4PFmM8NSpSBHNqPqm55Qn3LqFtT2emdEXVYsCzC2U/2147483647'/0".into());
        assert!(result.is_err());
    }

    #[test]
    fn invalid_wif_key_with_derivation() {
        let result = validate_key_expression(
            "L4rK1yDtCWekvXuE6oXD9jCYfFNV2cWRpVuPLBcCU2z8TrisoyY1/0".into(),
        );
        assert!(result.is_err());
    }

    #[test]
    fn invalid_derivation() {
        let result = validate_key_expression("xprv9s21ZrQH143K31xYSDQpPDxsXRTUcvj2iNHm5NUtrGiGG5e2DtALGdso3pGz6ssrdK4PFmM8NSpSBHNqPqm55Qn3LqFtT2emdEXVYsCzC2U/1aa".into());
        assert!(result.is_err());
    }

    #[test]
    fn invalid_missing_origin_end() {
        let result = validate_key_expression(
            "[deadbeef0260b2003c386519fc9eadf2b5cf124dd8eea4c4e68d5e154050a9346ea98ce600".into(),
        );
        assert!(result.is_err());
    }

    #[test]
    fn valid_from_project_desription() {
        let result = validate_key_expression("[deadbeef/0h/1h/2]xprvA1RpRA33e1JQ7ifknakTFpgNXPmW2YvmhqLQYMmrj4xJXXWYpDPS3xz7iAxn8L39njGVyuoseXzU6rcxFLJ8HFsTjSyQbLYnMpCqE2VbFWc/3h/4h/5h/*h".into());
        assert!(result.is_ok());
    }
}
