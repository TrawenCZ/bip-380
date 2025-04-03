use crate::structs::parsing_error::ParsingError;
use bip32::DerivationPath;

/// Validate the key origin
///
/// If the key origin is valid, this function returns Ok(()), otherwise it returns a Err(ParsingError).
///
/// Key origin  consists of:
///      An open bracket [
///      Exactly 8 hex characters for the fingerprint of the key where the derivation starts (see BIP 32 for details)
///      Followed by zero or more /NUM or /NUMh path elements to indicate the unhardened or hardened derivation steps between the fingerprint and the key that follows.
///      A closing bracket ]
///
pub fn validate_key_origin(key_origin: &str) -> Result<(), ParsingError> {
    let content = key_origin
        .strip_prefix('[')
        .and_then(|s| s.strip_suffix(']'))
        .ok_or_else(|| ParsingError::new("Key origin must start with [ and end with ]"))?;

    if content.len() < 8 {
        return Err(ParsingError::new("Fingerprint must be 8 characters long"));
    }
    let (fingerprint, path) = content.split_at(8);

    if !fingerprint.chars().all(|c| c.is_ascii_hexdigit()) {
        return Err(ParsingError::new("Fingerprint is not valid hex"));
    }

    let path = format!("m{}", path);

    path.parse::<DerivationPath>()
        .map_err(|e| ParsingError::new(&format!("Invalid derivation path: {}", e)))?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_key_origin_valid() {
        // This key origin contains a valid 8-character fingerprint followed by valid derivation path elements.
        let key_origin = "[deadbeef/0h/1h/2]";
        let result = validate_key_origin(key_origin);
        assert!(
            result.is_ok(),
            "Expected valid key origin to pass validation"
        );
    }

    #[test]
    fn test_validate_key_origin_missing_brackets() {
        let key_origin = "deadbeef/0h/1h/2";
        let result = validate_key_origin(key_origin);
        assert!(
            result.is_err(),
            "Expected key origin missing brackets to return error"
        );
    }

    #[test]
    fn test_validate_key_origin_invalid_fingerprint() {
        // Fingerprint contains a non hex-digit character 'g'
        let key_origin = "[deadbeeg/0h/1h/2]";
        let result = validate_key_origin(key_origin);
        assert!(
            result.is_err(),
            "Expected invalid fingerprint to return error"
        );
    }

    #[test]
    fn test_validate_key_origin_short_fingerprint() {
        // Fingerprint length is less than 8 characters.
        let key_origin = "[dead/0h/1h/2]";
        let result = validate_key_origin(key_origin);
        assert!(
            result.is_err(),
            "Expected short fingerprint to return error"
        );
    }

    #[test]
    fn test_validate_key_origin_invalid_path() {
        // Fingerprint length is less than 8 characters.
        let key_origin = "[dead/0h//2]";
        let result = validate_key_origin(key_origin);
        assert!(
            result.is_err(),
            "Expected short fingerprint to return error"
        );
    }
}
