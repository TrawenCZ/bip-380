use crate::structs::parsing_error::ParsingError;
use bip32::{ChildNumber, ExtendedKeyAttrs, XPrv, XPub};
use std::str::FromStr;

pub fn has_extended_key_prefix(key: &str) -> bool {
    key.starts_with("xpub") || key.starts_with("xprv")
}

pub fn validate_extended_key_attrs(attrs: &ExtendedKeyAttrs) -> Result<(), ParsingError> {
    match attrs {
        ExtendedKeyAttrs {
            depth,
            parent_fingerprint,
            ..
        } if *depth == 0 && parent_fingerprint.iter().any(|e| *e != 0) => Err(ParsingError::new(
            "Invalid key: the key cannot have zero depth with non-zero parent fingerprint",
        )),
        ExtendedKeyAttrs {
            depth,
            child_number,
            ..
        } if *depth == 0 && child_number.index() > 0 => Err(ParsingError::new(
            "Invalid key: the key cannot have zero depth with non-zero index",
        )),
        _ => Ok(()),
    }
}

/// Validate whether key is xpub encoded extended public key or xprv encoded extended private key (as defined in BIP 32):
///     Followed by zero or more /NUM or /`NUMh` path elements indicating BIP 32 derivation steps to be taken after the given extended key.
///     Optionally followed by a single /* or /*h final step to denote all direct unhardened or hardened children.
///     Returns the key as a String
pub fn validate_extended_key(key: &str) -> Result<String, ParsingError> {
    if !has_extended_key_prefix(key) {
        return Err(ParsingError::new("Key must start with xpub or xprv"));
    }

    let (key, path) = key.split_at(key.find('/').unwrap_or(key.len()));

    // Check if the key is valid
    if key.starts_with("xpub") {
        XPub::from_str(key).map_err(|e| ParsingError::new(&format!("Invalid xpub key: {e}")))?;
    } else {
        XPrv::from_str(key).map_err(|e| ParsingError::new(&format!("Invalid xprv key: {e}")))?;
    }

    if path.is_empty() {
        return Ok(key.into());
    }

    let mut derivation_segments: Vec<&str> = path[1..].split('/').collect();

    if derivation_segments.last() == Some(&"*") || derivation_segments.last() == Some(&"*h") {
        derivation_segments.pop();
    }

    for segment in derivation_segments {
        ChildNumber::from_str(segment).map_err(|e| {
            ParsingError::new(&format!("Invalid derivation segment '{segment}': {e}"))
        })?;
    }

    Ok(key.into())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_extended_key_valid() {
        let input = "xprvA1RpRA33e1JQ7ifknakTFpgNXPmW2YvmhqLQYMmrj4xJXXWYpDPS3xz7iAxn8L39njGVyuoseXzU6rcxFLJ8HFsTjSyQbLYnMpCqE2VbFWc/3h/4h/5h/*h";
        let result = validate_extended_key(input);
        assert!(result.is_ok());
    }
}
