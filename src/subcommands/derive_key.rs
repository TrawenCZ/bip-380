use std::str::FromStr;

use bip32::{secp256k1::elliptic_curve::zeroize::Zeroizing, Prefix, XPrv, XPub};

use crate::{
    structs::{derive_key_config::DeriveKeyConfig, parsing_error::ParsingError},
    utils::error_messages::invalid_seed_length_err,
};

use super::utils::hexadecimal::decode_hex;

/// The `derive_key` function in Rust parses input to derive public and private keys based on the
/// configuration provided.
///
/// Arguments:
///
/// * `input`: The `input` parameter is either
///   - public key, prefixed with *xpub*
///   - private key, prefixed with *xprv*
///   - seed, which is considered as seed when neither *xpub* and *xprv* prefixes are present
/// * `config`: The `config` parameter in the `derive_key` function is of type `DeriveKeyConfig`, which
///   is a reference to a struct containing configuration settings for deriving keys. This struct
///   includes a field `path` which is a collection of child numbers used in the key derivation process.
///
/// Returns:
///
/// The function `derive_key` returns a `Result` containing a `String` or a `ParsingError`. The `String`
/// value contains the derived key in the format "xpub:xpriv" where `xpub` is the public key and `xpriv`
/// is the private key. The key generation process respects the included Derivation Path included in
/// `DeriveKeyConfig`.
pub fn derive_key(input: String, config: &DeriveKeyConfig) -> Result<String, ParsingError> {
    let (xpub, xpriv) = match input.chars().collect::<Vec<char>>().as_slice() {
        priv_key @ ['x', 'p', 'r', 'v', ..] => {
            let mut xpriv = XPrv::from_str(&priv_key.iter().collect::<String>())?;

            for child_number in config.path.iter() {
                xpriv = xpriv.derive_child(child_number)?
            }

            let xpub = xpriv.public_key();

            (xpub.to_string(Prefix::XPUB), xpriv.to_string(Prefix::XPRV))
        }
        pub_key @ ['x', 'p', 'u', 'b', ..] => {
            let mut xpub = XPub::from_str(&pub_key.iter().collect::<String>())?;

            for child_number in config.path.iter() {
                xpub = xpub.derive_child(child_number)?
            }

            (xpub.to_string(Prefix::XPUB), Zeroizing::new("".into()))
        }
        seed_input => {
            let seed_no_whitespace = seed_input
                .iter()
                .filter(|c| !c.is_whitespace())
                .collect::<String>();

            if seed_no_whitespace.chars().count() % 2 != 0 {
                return Err(ParsingError::new(&invalid_seed_length_err(
                    &seed_no_whitespace,
                )));
            }

            let seed = decode_hex(&seed_no_whitespace)?;

            let root_xprv = XPrv::derive_from_path(seed, &config.path)?;

            let xpub = root_xprv.public_key();

            (
                xpub.to_string(Prefix::XPUB),
                root_xprv.to_string(Prefix::XPRV),
            )
        }
    };
    Ok(format!("{}:{}", xpub, *xpriv))
}

#[cfg(test)]
mod tests {
    use crate::tests::get_cmd;

    #[test]
    fn test_simple_seed() {
        let expected_output = "xpub661MyMwAqRbcFtXgS5sYJABqqG9YLmC4Q1Rdap9gSE8NqtwybGhePY2gZ29ESFjqJoCu1Rupje8YtGqsefD265TMg7usUDFdp6W1EGMcet8:xprv9s21ZrQH143K3QTDL4LXw2F7HEK3wJUD2nW2nRk4stbPy6cq3jPPqjiChkVvvNKmPGJxWUtg6LnF5kejMRNNU3TGtRBeJgk33yuGBxrMPHi\n";
        get_cmd()
            .args(["derive-key", "000102030405060708090a0b0c0d0e0f"])
            .assert()
            .success()
            .stdout(expected_output);
    }

    #[test]
    fn test_seed_with_single_whitespace() {
        let expected_output = "xpub661MyMwAqRbcFtXgS5sYJABqqG9YLmC4Q1Rdap9gSE8NqtwybGhePY2gZ29ESFjqJoCu1Rupje8YtGqsefD265TMg7usUDFdp6W1EGMcet8:xprv9s21ZrQH143K3QTDL4LXw2F7HEK3wJUD2nW2nRk4stbPy6cq3jPPqjiChkVvvNKmPGJxWUtg6LnF5kejMRNNU3TGtRBeJgk33yuGBxrMPHi\n";
        get_cmd()
            .args(["derive-key", "00 0102030405060708090a0b0c0d0e0f"])
            .assert()
            .success()
            .stdout(expected_output);
    }

    #[test]
    fn test_seed_with_single_whitespace_and_uppercase_chars() {
        let expected_output = "xpub661MyMwAqRbcFtXgS5sYJABqqG9YLmC4Q1Rdap9gSE8NqtwybGhePY2gZ29ESFjqJoCu1Rupje8YtGqsefD265TMg7usUDFdp6W1EGMcet8:xprv9s21ZrQH143K3QTDL4LXw2F7HEK3wJUD2nW2nRk4stbPy6cq3jPPqjiChkVvvNKmPGJxWUtg6LnF5kejMRNNU3TGtRBeJgk33yuGBxrMPHi\n";
        get_cmd()
            .args(["derive-key", "00 0102030405060708090A0B0c0d0E0F"])
            .assert()
            .success()
            .stdout(expected_output);
    }

    #[test]
    fn test_seed_with_multiple_whitespaces() {
        let expected_output = "xpub661MyMwAqRbcFtXgS5sYJABqqG9YLmC4Q1Rdap9gSE8NqtwybGhePY2gZ29ESFjqJoCu1Rupje8YtGqsefD265TMg7usUDFdp6W1EGMcet8:xprv9s21ZrQH143K3QTDL4LXw2F7HEK3wJUD2nW2nRk4stbPy6cq3jPPqjiChkVvvNKmPGJxWUtg6LnF5kejMRNNU3TGtRBeJgk33yuGBxrMPHi\n";
        get_cmd()
            .args(["derive-key", "00   01\t02030405060708090a0b0c0d0e0f"])
            .assert()
            .success()
            .stdout(expected_output);
    }

    #[test]
    fn test_seed_with_large_seed() {
        let expected_output = "xpub661MyMwAqRbcFW31YEwpkMuc5THy2PSt5bDMsktWQcFF8syAmRUapSCGu8ED9W6oDMSgv6Zz8idoc4a6mr8BDzTJY47LJhkJ8UB7WEGuduB:xprv9s21ZrQH143K31xYSDQpPDxsXRTUcvj2iNHm5NUtrGiGG5e2DtALGdso3pGz6ssrdK4PFmM8NSpSBHNqPqm55Qn3LqFtT2emdEXVYsCzC2U\n";
        get_cmd()
            .args(["derive-key", "fffcf9f6f3f0edeae7e4e1dedbd8d5d2cfccc9c6c3c0bdbab7b4b1aeaba8a5a29f9c999693908d8a8784817e7b7875726f6c696663605d5a5754514e4b484542"])
            .assert()
            .success()
            .stdout(expected_output);
    }

    #[test]
    fn test_seed_with_simple_input_through_stdin() {
        let expected_output = "xpub661MyMwAqRbcFtXgS5sYJABqqG9YLmC4Q1Rdap9gSE8NqtwybGhePY2gZ29ESFjqJoCu1Rupje8YtGqsefD265TMg7usUDFdp6W1EGMcet8:xprv9s21ZrQH143K3QTDL4LXw2F7HEK3wJUD2nW2nRk4stbPy6cq3jPPqjiChkVvvNKmPGJxWUtg6LnF5kejMRNNU3TGtRBeJgk33yuGBxrMPHi\n";
        get_cmd()
            .args(["derive-key", "-"])
            .write_stdin("000102030405060708090a0b0c0d0e0f")
            .assert()
            .success()
            .stdout(expected_output);
    }

    #[test]
    fn test_seed_with_complex_input_through_stdin() {
        let expected_output = "\
        xpub661MyMwAqRbcFtXgS5sYJABqqG9YLmC4Q1Rdap9gSE8NqtwybGhePY2gZ29ESFjqJoCu1Rupje8YtGqsefD265TMg7usUDFdp6W1EGMcet8:xprv9s21ZrQH143K3QTDL4LXw2F7HEK3wJUD2nW2nRk4stbPy6cq3jPPqjiChkVvvNKmPGJxWUtg6LnF5kejMRNNU3TGtRBeJgk33yuGBxrMPHi\n\
        xpub661MyMwAqRbcFW31YEwpkMuc5THy2PSt5bDMsktWQcFF8syAmRUapSCGu8ED9W6oDMSgv6Zz8idoc4a6mr8BDzTJY47LJhkJ8UB7WEGuduB:xprv9s21ZrQH143K31xYSDQpPDxsXRTUcvj2iNHm5NUtrGiGG5e2DtALGdso3pGz6ssrdK4PFmM8NSpSBHNqPqm55Qn3LqFtT2emdEXVYsCzC2U\n\
        ";
        get_cmd()
            .args(["derive-key", "-"])
            .write_stdin("\
            000102030405060708090a0b0c0d0e0f\n\
            fffcf9f6f3f0edeae7e4e1dedbd8d5d2cfccc9c6c3c0bdbab7b4b1aeaba8a5a29f9c999693908d8a8784817e7b7875726f6c696663605d5a5754514e4b484542\n\
            ")
            .assert()
            .success()
            .stdout(expected_output);
    }

    #[test]
    fn test_seed_with_two_valid_and_one_invalid_stdin_inputs() {
        let expected_output = "\
        xpub661MyMwAqRbcFtXgS5sYJABqqG9YLmC4Q1Rdap9gSE8NqtwybGhePY2gZ29ESFjqJoCu1Rupje8YtGqsefD265TMg7usUDFdp6W1EGMcet8:xprv9s21ZrQH143K3QTDL4LXw2F7HEK3wJUD2nW2nRk4stbPy6cq3jPPqjiChkVvvNKmPGJxWUtg6LnF5kejMRNNU3TGtRBeJgk33yuGBxrMPHi\n\
        xpub661MyMwAqRbcFW31YEwpkMuc5THy2PSt5bDMsktWQcFF8syAmRUapSCGu8ED9W6oDMSgv6Zz8idoc4a6mr8BDzTJY47LJhkJ8UB7WEGuduB:xprv9s21ZrQH143K31xYSDQpPDxsXRTUcvj2iNHm5NUtrGiGG5e2DtALGdso3pGz6ssrdK4PFmM8NSpSBHNqPqm55Qn3LqFtT2emdEXVYsCzC2U\n\
        ";
        get_cmd()
            .args(["derive-key", "-"])
            .write_stdin("\
            000102030405060708090a0b0c0d0e0f\n\
            fffcf9f6f3f0edeae7e4e1dedbd8d5d2cfccc9c6c3c0bdbab7b4b1aeaba8a5a29f9c999693908d8a8784817e7b7875726f6c696663605d5a5754514e4b484542
            
            
            xx
            ")
            .assert()
            .failure()
            .stdout(expected_output);
    }

    #[test]
    fn test_derive_from_pub_key_with_path() {
        let expected_output = "xpub6H1LXWLaKsWFhvm6RVpEL9P4KfRZSW7abD2ttkWP3SSQvnyA8FSVqNTEcYFgJS2UaFcxupHiYkro49S8yGasTvXEYBVPamhGW6cFJodrTHy:\n";
        get_cmd()
            .args(["derive-key", "xpub6D4BDPcP2GT577Vvch3R8wDkScZWzQzMMUm3PWbmWvVJrZwQY4VUNgqFJPMM3No2dFDFGTsxxpG5uJh7n7epu4trkrX7x7DogT5Uv6fcLW5", "--path", "2/1000000000"])
            .assert()
            .success()
            .stdout(expected_output);
    }

    #[test]
    fn test_derive_from_priv_key_with_path_at_the_end() {
        let expected_output = "xpub6H1LXWLaKsWFhvm6RVpEL9P4KfRZSW7abD2ttkWP3SSQvnyA8FSVqNTEcYFgJS2UaFcxupHiYkro49S8yGasTvXEYBVPamhGW6cFJodrTHy:xprvA41z7zogVVwxVSgdKUHDy1SKmdb533PjDz7J6N6mV6uS3ze1ai8FHa8kmHScGpWmj4WggLyQjgPie1rFSruoUihUZREPSL39UNdE3BBDu76\n";
        get_cmd()
            .args(["derive-key", "xprv9wTYmMFdV23N2TdNG573QoEsfRrWKQgWeibmLntzniatZvR9BmLnvSxqu53Kw1UmYPxLgboyZQaXwTCg8MSY3H2EU4pWcQDnRnrVA1xe8fs", "--path", "2H/2/1000000000"])
            .assert()
            .success()
            .stdout(expected_output);
    }

    #[test]
    fn test_derive_from_priv_key_with_path_at_the_beginning() {
        let expected_output = "xpub6H1LXWLaKsWFhvm6RVpEL9P4KfRZSW7abD2ttkWP3SSQvnyA8FSVqNTEcYFgJS2UaFcxupHiYkro49S8yGasTvXEYBVPamhGW6cFJodrTHy:xprvA41z7zogVVwxVSgdKUHDy1SKmdb533PjDz7J6N6mV6uS3ze1ai8FHa8kmHScGpWmj4WggLyQjgPie1rFSruoUihUZREPSL39UNdE3BBDu76\n";
        get_cmd()
            .args(["derive-key", "--path", "2H/2/1000000000","xprv9wTYmMFdV23N2TdNG573QoEsfRrWKQgWeibmLntzniatZvR9BmLnvSxqu53Kw1UmYPxLgboyZQaXwTCg8MSY3H2EU4pWcQDnRnrVA1xe8fs"])
            .assert()
            .success()
            .stdout(expected_output);
    }

    #[test]
    fn test_pubkey_and_prvkey_mismatch() {
        let expected_stderr = "Parsing error: cryptographic error\n";
        get_cmd()
            .args(["derive-key", "xpub661MyMwAqRbcEYS8w7XLSVeEsBXy79zSzH1J8vCdxAZningWLdN3zgtU6LBpB85b3D2yc8sfvZU521AAwdZafEz7mnzBBsz4wKY5fTtTQBm"])
            .assert()
            .failure()
            .stderr(expected_stderr);
    }
}
