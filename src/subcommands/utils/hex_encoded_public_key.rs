use crate::structs::parsing_error::ParsingError;

const HEX_ENCODED_PUBLIC_KEY_PREFIXES: [&str; 3] = ["02", "03", "04"];

pub fn has_hex_encoded_public_key_prefix(input: &str) -> bool {
    HEX_ENCODED_PUBLIC_KEY_PREFIXES
        .iter()
        .any(|prefix| input.starts_with(prefix))
}

/// This function checks if the input is a hex encoded public key
/// and returns it if it is valid. If the input is not a hex encoded public key,
/// it returns an error.
/// Hex encoded public key starts with either:
///      02 or 03, in which case it must be 66 characters long,
///      04, in which case it must be 130 characters long.
pub fn parse_hex_encoded_public_key(input: &str) -> Result<(), ParsingError> {
    if !has_hex_encoded_public_key_prefix(input) {
        return Err(ParsingError::new(
            "Hex encoded public key must start with 02, 03 or 04",
        ));
    }

    if !input.chars().all(|c| c.is_ascii_hexdigit()) {
        return Err(ParsingError::new(
            "Hex encoded public key contains non-hexadecimal characters",
        ));
    }

    if input.starts_with("04") {
        if input.len() != 130 {
            return Err(ParsingError::new(
                "Hex encoded public key with prefix '04' must be 130 characters long",
            ));
        }

        return Ok(());
    }

    if input.len() != 66 {
        return Err(ParsingError::new(
            "Hex encoded public key with prefix '02' or '03' must be 66 characters long",
        ));
    }

    Ok(())
}
