use std::str::FromStr;

use bip32::{secp256k1::elliptic_curve::zeroize::Zeroizing, Prefix, XPrv, XPub};

use crate::{
    structs::{derive_key_config::DeriveKeyConfig, parsing_error::ParsingError},
    traits::string_utils::{CharArrayUtils, StringSliceUtils},
    utils::error_messages::invalid_seed_length_err,
};

use super::utils::{extended_key::validate_extended_key_attrs, hexadecimal::decode_hex};

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
pub fn derive_key(input: &str, config: &DeriveKeyConfig) -> Result<String, ParsingError> {
    let (xpub, xpriv) = match input.charify().as_slice() {
        priv_key @ ['x', 'p', 'r', 'v', ..] => {
            let mut xpriv = XPrv::from_str(&priv_key.iter().collect::<String>())?;

            for child_number in config.path.iter() {
                xpriv = xpriv.derive_child(child_number)?;
            }

            validate_extended_key_attrs(xpriv.attrs())?;

            let xpub = xpriv.public_key();

            validate_extended_key_attrs(xpub.attrs())?;

            (xpub.to_string(Prefix::XPUB), xpriv.to_string(Prefix::XPRV))
        }
        pub_key @ ['x', 'p', 'u', 'b', ..] => {
            let mut xpub = XPub::from_str(&pub_key.iter().collect::<String>())?;

            for child_number in config.path.iter() {
                xpub = xpub.derive_child(child_number)?;
            }

            validate_extended_key_attrs(xpub.attrs())?;

            (xpub.to_string(Prefix::XPUB), Zeroizing::new(String::new()))
        }
        seed_input => {
            let seed_no_whitespace = seed_input
                .stringify()
                .split([' ', '\t'])
                .map(|slice| {
                    if slice.chars().count() % 2 == 0 {
                        Ok(slice)
                    } else {
                        Err(ParsingError::new(&invalid_seed_length_err(slice)))
                    }
                })
                .collect::<Result<String, ParsingError>>()?;

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
    use crate::{
        structs::derive_key_config::DeriveKeyConfig, test_utils::get_cmd,
        traits::parsable::Parsable,
    };

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

    fn get_config(path: &str) -> DeriveKeyConfig {
        DeriveKeyConfig::parse(&mut vec!["derive-key", "--path", path]).unwrap()
    }

    mod validate_and_normalize_seed_tests {
        use super::super::{derive_key, DeriveKeyConfig};

        #[test]
        fn basic_seed_validation_valid() {
            let result = derive_key(
                "fffcf9f6f3f0edeae7e4e1dedbd8d5d2cfccc9c6c3c0bdbab7b4b1aeaba8a5a29f9c999693908d8a8784817e7b7875726f6c696663605d5a5754514e4b484542",
                &DeriveKeyConfig::default()
            );
            assert!(result.is_ok());
        }

        #[test]
        fn basic_seed_validation_valid_with_spaces() {
            let result = derive_key(
                "ff fc   f9f6 f3f0edeae7e4e1dedbd8d5d2cfccc9c6c3c0bdbab7b4b1aeaba8a5a29f9c999693908d8a8784817e7b7875726f6c696663605d5a5754514e4b484542",
                &DeriveKeyConfig::default()
            );
            assert!(result.is_ok());
        }

        #[test]
        fn basic_seed_validation_valid_with_tabs() {
            let result = derive_key(
                "ff\tfcf9f6\tf3f0edeae7e4e1dedbd8d5d2cfccc9c6c3c0bdbab7b4b1aeaba8a5a29f9c999693908d8a8784817e7b7875726f6c696663605d5a5754514e4b484542",
                &DeriveKeyConfig::default()
            );
            assert!(result.is_ok());
        }

        #[test]
        fn basic_seed_validation_valid_with_tabs_and_spaces() {
            let result = derive_key(
                "ff\tfc\t\tf9   f6f3f0edeae7e4e1dedbd8d5d2cfccc9c6c3c0bdbab7b4b1aeaba8a5a29f9c999693908d8a8784817e7b7875726f6c696663605d5a5754514e4b484542"
                , &DeriveKeyConfig::default()
            );
            assert!(result.is_ok());
        }

        #[test]
        fn basic_seed_validation_valid_2() {
            let result = derive_key(
                "4b381541583be4423346c643850da4b320e46a87ae3d2a4e6da11eba819cd4acba45d239319ac14f863b8d5ab5a0d0c64d2e8a1e7d1457df2e5a3c51c73235be",
                &DeriveKeyConfig::default()
            );
            assert!(result.is_ok());
        }

        #[test]
        fn basic_seed_validation_valid_1() {
            let result = derive_key(
                "000102030405060708090a0b0c0d0e0f",
                &DeriveKeyConfig::default(),
            );
            assert!(result.is_ok());
        }

        #[test]
        fn basic_seed_validation_valid_4() {
            let result = derive_key(
                "3ddd5602285899a946114506157c7997e5444528f3003f6134712147db19b678",
                &DeriveKeyConfig::default(),
            );
            assert!(result.is_ok());
        }
    }

    mod derive_key_from_seed_without_path_tests {
        use super::super::{derive_key, DeriveKeyConfig};

        #[test]
        fn seed_without_path_vector_2() {
            let result = derive_key(
                "fffcf9f6f3f0edeae7e4e1dedbd8d5d2cfccc9c6c3c0bdbab7b4b1aeaba8a5a29f9c999693908d8a8784817e7b7875726f6c696663605d5a5754514e4b484542",
                &DeriveKeyConfig::default()
            );
            assert!(result.is_ok());
        }

        #[test]
        fn seed_without_path_vector_3() {
            let result = derive_key(
                "4b381541583be4423346c643850da4b320e46a87ae3d2a4e6da11eba819cd4acba45d239319ac14f863b8d5ab5a0d0c64d2e8a1e7d1457df2e5a3c51c73235be",
                &DeriveKeyConfig::default()
            );
            assert!(result.is_ok());
        }

        #[test]
        fn seed_without_path_vector_4() {
            let result = derive_key(
                "3ddd5602285899a946114506157c7997e5444528f3003f6134712147db19b678",
                &DeriveKeyConfig::default(),
            );
            assert!(result.is_ok());
        }

        #[test]
        fn seed_without_path_vector_1() {
            let result = derive_key(
                "000102030405060708090a0b0c0d0e0f",
                &DeriveKeyConfig::default(),
            );
            assert!(result.is_ok());
        }
    }

    mod derive_key_from_seed_with_path_tests {
        use super::super::derive_key;
        use super::get_config;

        #[test]
        fn seed_without_path_vector_2_1() {
            let result = derive_key(
                "fffcf9f6f3f0edeae7e4e1dedbd8d5d2cfccc9c6c3c0bdbab7b4b1aeaba8a5a29f9c999693908d8a8784817e7b7875726f6c696663605d5a5754514e4b484542",
                &get_config("/0")
            );
            assert!(result.is_ok());
        }

        #[test]
        fn seed_without_path_vector_2_2() {
            let result = derive_key(
                "fffcf9f6f3f0edeae7e4e1dedbd8d5d2cfccc9c6c3c0bdbab7b4b1aeaba8a5a29f9c999693908d8a8784817e7b7875726f6c696663605d5a5754514e4b484542",
                &get_config("/0/2147483647H")
            );
            assert!(result.is_ok());
        }

        #[test]
        fn seed_without_path_vector_2_3() {
            let result = derive_key(
                "fffcf9f6f3f0edeae7e4e1dedbd8d5d2cfccc9c6c3c0bdbab7b4b1aeaba8a5a29f9c999693908d8a8784817e7b7875726f6c696663605d5a5754514e4b484542",
                &get_config("/0/2147483647H/1")
            );
            assert!(result.is_ok());
        }

        #[test]
        fn seed_without_path_vector_2_4() {
            let result = derive_key(
                "fffcf9f6f3f0edeae7e4e1dedbd8d5d2cfccc9c6c3c0bdbab7b4b1aeaba8a5a29f9c999693908d8a8784817e7b7875726f6c696663605d5a5754514e4b484542",
                &get_config("/0/2147483647H/1/2147483646H")
            );
            assert!(result.is_ok());
        }

        #[test]
        fn seed_without_path_vector_2_5() {
            let result = derive_key(
                "fffcf9f6f3f0edeae7e4e1dedbd8d5d2cfccc9c6c3c0bdbab7b4b1aeaba8a5a29f9c999693908d8a8784817e7b7875726f6c696663605d5a5754514e4b484542",
                &get_config("/0/2147483647H/1/2147483646H/2")
            );
            assert!(result.is_ok());
        }

        #[test]
        fn seed_without_path_vector_3_1() {
            let result = derive_key(
                "4b381541583be4423346c643850da4b320e46a87ae3d2a4e6da11eba819cd4acba45d239319ac14f863b8d5ab5a0d0c64d2e8a1e7d1457df2e5a3c51c73235be",
                &get_config("/0")
            );
            assert!(result.is_ok());
        }
    }

    mod derive_key_from_xpub_without_path_tests {
        use super::super::{derive_key, DeriveKeyConfig};

        #[test]
        fn derive_key_xpub_vector_1_1() {
            let result = derive_key(
                "xpub661MyMwAqRbcFtXgS5sYJABqqG9YLmC4Q1Rdap9gSE8NqtwybGhePY2gZ29ESFjqJoCu1Rupje8YtGqsefD265TMg7usUDFdp6W1EGMcet8",
                &DeriveKeyConfig::default()
            );
            assert!(result.is_ok());
        }

        #[test]
        fn derive_key_xpub_vector_1_2() {
            let result = derive_key(
                "xpub68Gmy5EdvgibQVfPdqkBBCHxA5htiqg55crXYuXoQRKfDBFA1WEjWgP6LHhwBZeNK1VTsfTFUHCdrfp1bgwQ9xv5ski8PX9rL2dZXvgGDnw",
                &DeriveKeyConfig::default()
            );
            assert!(result.is_ok());
        }

        #[test]
        fn derive_key_xpub_vector_1_3() {
            let result = derive_key(
                "xpub6D4BDPcP2GT577Vvch3R8wDkScZWzQzMMUm3PWbmWvVJrZwQY4VUNgqFJPMM3No2dFDFGTsxxpG5uJh7n7epu4trkrX7x7DogT5Uv6fcLW5",
                &DeriveKeyConfig::default());
            assert!(result.is_ok());
        }

        #[test]
        fn derive_key_xpub_vector_1_4() {
            let result = derive_key(
                "xpub6FHa3pjLCk84BayeJxFW2SP4XRrFd1JYnxeLeU8EqN3vDfZmbqBqaGJAyiLjTAwm6ZLRQUMv1ZACTj37sR62cfN7fe5JnJ7dh8zL4fiyLHV",
                &DeriveKeyConfig::default()
            );
            assert!(result.is_ok());
        }

        #[test]
        fn derive_key_xpub_vector_1_5() {
            let result = derive_key(
                "xpub6H1LXWLaKsWFhvm6RVpEL9P4KfRZSW7abD2ttkWP3SSQvnyA8FSVqNTEcYFgJS2UaFcxupHiYkro49S8yGasTvXEYBVPamhGW6cFJodrTHy",
                &DeriveKeyConfig::default());
            assert!(result.is_ok());
        }

        #[test]
        fn derive_key_xpub_version_invalid() {
            let result = derive_key(
                "xpub661MyMwAqRbcEYS8w7XLSVeEsBXy79zSzH1J8vCdxAZningWLdN3zgtU6LBpB85b3D2yc8sfvZU521AAwdZafEz7mnzBBsz4wKY5fTtTQBm",
                &DeriveKeyConfig::default());
            assert!(result.is_err());
        }

        #[test]
        fn derive_key_xpub_prefix_04_invalid() {
            let result = derive_key(
                "xpub661MyMwAqRbcEYS8w7XLSVeEsBXy79zSzH1J8vCdxAZningWLdN3zgtU6Txnt3siSujt9RCVYsx4qHZGc62TG4McvMGcAUjeuwZdduYEvFn",
                &DeriveKeyConfig::default()
            );
            assert!(result.is_err());
        }

        #[test]
        fn derive_key_xpub_prefix_01_invalid() {
            let result = derive_key(
                "xpub661MyMwAqRbcEYS8w7XLSVeEsBXy79zSzH1J8vCdxAZningWLdN3zgtU6N8ZMMXctdiCjxTNq964yKkwrkBJJwpzZS4HS2fxvyYUA4q2Xe4",
                &DeriveKeyConfig::default()
            );
            assert!(result.is_err());
        }

        #[test]
        fn derive_key_xpub_zero_depth_with_non_zero_parent_fingerprint_invalid() {
            let result = derive_key(
                "xpub661no6RGEX3uJkY4bNnPcw4URcQTrSibUZ4NqJEw5eBkv7ovTwgiT91XX27VbEXGENhYRCf7hyEbWrR3FewATdCEebj6znwMfQkhRYHRLpJ",
                &DeriveKeyConfig::default()
            );
            assert!(result.is_err());
        }

        #[test]
        fn derive_key_xpub_zero_depth_with_non_zero_index_invalid() {
            let result = derive_key(
                "xpub661MyMwAuDcm6CRQ5N4qiHKrJ39Xe1R1NyfouMKTTWcguwVcfrZJaNvhpebzGerh7gucBvzEQWRugZDuDXjNDRmXzSZe4c7mnTK97pTvGS8",
                &DeriveKeyConfig::default()
            );
            assert!(result.is_err());
        }

        #[test]
        fn derive_key_xpub_invalid_pubkey_invalid() {
            let result = derive_key(
                "xpub661MyMwAqRbcEYS8w7XLSVeEsBXy79zSzH1J8vCdxAZningWLdN3zgtU6Q5JXayek4PRsn35jii4veMimro1xefsM58PgBMrvdYre8QyULY",
                &DeriveKeyConfig::default()
            );
            assert!(result.is_err());
        }
    }

    mod derive_key_from_xpub_with_path_tests {
        use super::super::derive_key;
        use super::get_config;

        #[test]
        fn derive_key_xpub_vector_2_1() {
            let result = derive_key(
                "xpub661MyMwAqRbcFW31YEwpkMuc5THy2PSt5bDMsktWQcFF8syAmRUapSCGu8ED9W6oDMSgv6Zz8idoc4a6mr8BDzTJY47LJhkJ8UB7WEGuduB",
                &get_config("0")
            );
            assert!(result.is_ok());
        }

        #[test]
        fn derive_key_xpub_vector_2_2_hardened_invalid() {
            let result = derive_key(
                "xpub69H7F5d8KSRgmmdJg2KhpAK8SR3DjMwAdkxj3ZuxV27CprR9LgpeyGmXUbC6wb7ERfvrnKZjXoUmmDznezpbZb7ap6r1D3tgFxHmwMkQTPH",
                &get_config("/2147483647H")
            );
            assert!(result.is_err());
        }
    }

    mod derive_key_from_xprv_with_path_tests {
        use super::super::derive_key;
        use super::get_config;

        #[test]
        fn derive_key_xprv_vector_2() {
            let result = derive_key(
                "xprv9s21ZrQH143K31xYSDQpPDxsXRTUcvj2iNHm5NUtrGiGG5e2DtALGdso3pGz6ssrdK4PFmM8NSpSBHNqPqm55Qn3LqFtT2emdEXVYsCzC2U",
                &get_config("0/2147483647H/1/2147483646H/2")
            );
            assert!(result.is_ok());
        }

        #[test]
        fn derive_key_xprv_vector_2_1() {
            let result = derive_key(
                "xprv9s21ZrQH143K31xYSDQpPDxsXRTUcvj2iNHm5NUtrGiGG5e2DtALGdso3pGz6ssrdK4PFmM8NSpSBHNqPqm55Qn3LqFtT2emdEXVYsCzC2U",
                &get_config("0")
            );
            assert!(result.is_ok());
        }

        #[test]
        fn derive_key_xprv_vector_2_2() {
            let result = derive_key(
                "xprv9vHkqa6EV4sPZHYqZznhT2NPtPCjKuDKGY38FBWLvgaDx45zo9WQRUT3dKYnjwih2yJD9mkrocEZXo1ex8G81dwSM1fwqWpWkeS3v86pgKt",
                &get_config("0/2147483647H")
            );
            assert!(result.is_ok());
        }

        #[test]
        fn derive_key_xprv_vector_2_3() {
            let result = derive_key(
                "xprv9wSp6B7kry3Vj9m1zSnLvN3xH8RdsPP1Mh7fAaR7aRLcQMKTR2vidYEeEg2mUCTAwCd6vnxVrcjfy2kRgVsFawNzmjuHc2YmYRmagcEPdU9",
                &get_config("0/2147483647H/1")
            );
            assert!(result.is_ok());
        }

        #[test]
        fn derive_key_xprv_vector_2_4() {
            let result = derive_key(
                "xprv9zFnWC6h2cLgpmSA46vutJzBcfJ8yaJGg8cX1e5StJh45BBciYTRXSd25UEPVuesF9yog62tGAQtHjXajPPdbRCHuWS6T8XA2ECKADdw4Ef",
                &get_config("/0/2147483647H/1/2147483646H")
            );
            assert!(result.is_ok());
        }

        #[test]
        fn derive_key_xprv_vector_2_5() {
            let result = derive_key(
                "xprvA1RpRA33e1JQ7ifknakTFpgNXPmW2YvmhqLQYMmrj4xJXXWYpDPS3xz7iAxn8L39njGVyuoseXzU6rcxFLJ8HFsTjSyQbLYnMpCqE2VbFWc", &get_config("/0/2147483647H/1/2147483646H/2")
            );
            assert!(result.is_ok());
        }
    }

    mod derive_key_from_xprv_without_path_tests {
        use super::super::{derive_key, DeriveKeyConfig};

        #[test]
        fn derive_key_xprv_valid() {
            let result = derive_key(
                "xprv9s21ZrQH143K3QTDL4LXw2F7HEK3wJUD2nW2nRk4stbPy6cq3jPPqjiChkVvvNKmPGJxWUtg6LnF5kejMRNNU3TGtRBeJgk33yuGBxrMPHi",
                &DeriveKeyConfig::default()
            );
            assert!(result.is_ok());
        }

        #[test]
        fn derive_key_xprv_valid_vector_1_2() {
            let result = derive_key(
                "xprv9uHRZZhk6KAJC1avXpDAp4MDc3sQKNxDiPvvkX8Br5ngLNv1TxvUxt4cV1rGL5hj6KCesnDYUhd7oWgT11eZG7XnxHrnYeSvkzY7d2bhkJ7",
                &DeriveKeyConfig::default());
            assert!(result.is_ok());
        }

        #[test]
        fn derive_key_xprv_prvkey_version_pubkey_missmach_invalid() {
            let result = derive_key(
                "xprv9s21ZrQH143K24Mfq5zL5MhWK9hUhhGbd45hLXo2Pq2oqzMMo63oStZzFGTQQD3dC4H2D5GBj7vWvSQaaBv5cxi9gafk7NF3pnBju6dwKvH",
                &DeriveKeyConfig::default());
            assert!(result.is_err());
        }

        #[test]
        fn derive_key_xprv_prvkey_prefix_04_invalid() {
            let result = derive_key(
                "xprv9s21ZrQH143K24Mfq5zL5MhWK9hUhhGbd45hLXo2Pq2oqzMMo63oStZzFGpWnsj83BHtEy5Zt8CcDr1UiRXuWCmTQLxEK9vbz5gPstX92JQ",
                &DeriveKeyConfig::default());
            assert!(result.is_err());
        }

        #[test]
        fn derive_key_xprv_prvkey_prefix_01_invalid() {
            let result = derive_key(
                "xprv9s21ZrQH143K24Mfq5zL5MhWK9hUhhGbd45hLXo2Pq2oqzMMo63oStZzFAzHGBP2UuGCqWLTAPLcMtD9y5gkZ6Eq3Rjuahrv17fEQ3Qen6J",
                &DeriveKeyConfig::default());
            assert!(result.is_err());
        }

        #[test]
        fn derive_key_xprv_zero_depth_non_zero_parent_invalid() {
            let result = derive_key(
                "xprv9s2SPatNQ9Vc6GTbVMFPFo7jsaZySyzk7L8n2uqKXJen3KUmvQNTuLh3fhZMBoG3G4ZW1N2kZuHEPY53qmbZzCHshoQnNf4GvELZfqTUrcv",
                &DeriveKeyConfig::default());
            assert!(result.is_err());
        }

        #[test]
        fn derive_key_xprv_zero_depth_non_zero_index_invalid() {
            let result = derive_key(
                "xprv9s21ZrQH4r4TsiLvyLXqM9P7k1K3EYhA1kkD6xuquB5i39AU8KF42acDyL3qsDbU9NmZn6MsGSUYZEsuoePmjzsB3eFKSUEh3Gu1N3cqVUN",
                &DeriveKeyConfig::default());
            assert!(result.is_err());
        }

        #[test]
        fn derive_key_xprv_0_not_in_1_nminus1_invalid() {
            let result = derive_key(
                "xprv9s21ZrQH143K24Mfq5zL5MhWK9hUhhGbd45hLXo2Pq2oqzMMo63oStZzF93Y5wvzdUayhgkkFoicQZcP3y52uPPxFnfoLZB21Teqt1VvEHx",
                &DeriveKeyConfig::default());
            assert!(result.is_err());
        }

        #[test]
        fn derive_key_xprv_n_not_in_1_nminus1_invalid() {
            let result = derive_key(
                "xprv9s21ZrQH143K24Mfq5zL5MhWK9hUhhGbd45hLXo2Pq2oqzMMo63oStZzFAzHGBP2UuGCqWLTAPLcMtD5SDKr24z3aiUvKr9bJpdrcLg1y3G",
                &DeriveKeyConfig::default());
            assert!(result.is_err());
        }

        #[test]
        fn derive_key_xprv_checksum_invalid() {
            let result = derive_key(
                "xprv9s21ZrQH143K3QTDL4LXw2F7HEK3wJUD2nW2nRk4stbPy6cq3jPPqjiChkVvvNKmPGJxWUtg6LnF5kejMRNNU3TGtRBeJgk33yuGBxrMPHL",
                &DeriveKeyConfig::default());
            assert!(result.is_err());
        }
    }
}
