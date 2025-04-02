use bip32::secp256k1::sha2::{Digest, Sha256};

use crate::structs::parsing_error::ParsingError;

pub fn validate_wif_private_key(key: &str) -> Result<(), ParsingError> {
    let bytes = bs58::decode(key)
        .into_vec()
        .map_err(|_| ParsingError::new("Could not convert WIF from base58"))?;

    // 37 bytes for uncompressed key, 38 bytes for compressed key
    if ![37, 38].contains(&bytes.len()) {
        return Err(ParsingError::new("Invalid WIF format"));
    }

    if bytes.first() != Some(&0x80) {
        return Err(ParsingError::new("WIF must start with 0x80"));
    }

    let (bytes, expected_checksum) = bytes.split_at(bytes.len() - 4);

    let new_checksum = Sha256::digest(Sha256::digest(bytes));
    let new_checksum = &new_checksum[..4];

    if expected_checksum != new_checksum {
        return Err(ParsingError::new("WIF checksum does not match"));
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_wif_private_key_valid() {
        let valid_wifs = [
            "5HueCGU8rMjxEXxiPuD5BDku4MkFqeZyd4dZ1jvhTVqvbTLvyTJ",
            "5KYZdUEo39z3FPrtuX2QbbwGnNP5zTd7yyr2SC1j299sBCnWjss",
            "5K7T2qR7K5MUMDmkJvCEPbUeALuPyc6LFEnBiwWLuKCEVdBp8qV",
            "L4rK1yDtCWekvXuE6oXD9jCYfFNV2cWRpVuPLBcCU2z8TrisoyY1",
        ];

        for valid_wif in valid_wifs.iter() {
            assert!(
                validate_wif_private_key(valid_wif).is_ok(),
                "Expected valid WIF: {}",
                valid_wif
            );
        }
    }

    #[test]
    fn test_validate_wif_private_key_invalid_checksum() {
        let invalid_wif = "L4rK1yDtCWekvXuE6oXD9jCYfFNV2cWRpVuPLBcCU2z8TrisoyXw";

        assert!(
            validate_wif_private_key(invalid_wif).is_err(),
            "Expected WIF to have invalid checksum: {:?}",
            invalid_wif
        );
    }
}
