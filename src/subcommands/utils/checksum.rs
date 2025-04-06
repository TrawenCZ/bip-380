pub const CHECKSUM_DIVIDER_SYMBOL: &str = "#";
const CHECKSUM_LENGTH: usize = 8;
const INPUT_CHARSET: &str = "0123456789()[],'/*abcdefgh@:$%{}IJKLMNOPQRSTUVWXYZ&+-.;<=>?!^_|~ijklmnopqrstuvwxyzABCDEFGH`#\"\\ ";
const CHECKSUM_CHARSET: &str = "qpzry9x8gf2tvdw0s3jn54khce6mua7l";
const GENERATOR: [u64; 5] = [
    0xf5dee51989,
    0xa9fdca3312,
    0x1bab10e32d,
    0x3706b1677a,
    0x644d626ffd,
];

enum CharsetKind {
    Input,
    Checksum,
}

fn invalid_char_err_msg(kind: CharsetKind, character: char) -> String {
    let (name, set) = match kind {
        CharsetKind::Input => ("input", INPUT_CHARSET),
        CharsetKind::Checksum => ("checksum", CHECKSUM_CHARSET),
    };
    format!("All received {name} characters should be one of \"{set}\". But found character '{character}'.")
}

fn checksum_polymod(symbols: Vec<usize>) -> u64 {
    let mut checksum: u64 = 1;
    for value in symbols {
        let top = checksum >> 35;
        checksum = ((checksum & 0x7ffffffff) << 5) ^ value as u64;
        for (i, &gen) in GENERATOR.iter().enumerate() {
            checksum ^= if ((top >> i) & 1) != 0 { gen } else { 0 };
        }
    }
    checksum
}

fn checksum_expand(script: &str) -> Vec<usize> {
    let mut groups = Vec::new();
    let mut symbols = Vec::new();

    for character in script.chars() {
        let index = INPUT_CHARSET
            .find(character)
            .unwrap_or_else(|| panic!("{}", invalid_char_err_msg(CharsetKind::Input, character)));
        symbols.push(index & 31);
        groups.push(index >> 5);

        if groups.len() == 3 {
            symbols.push(groups[0] * 9 + groups[1] * 3 + groups[2]);
            groups.clear();
        }
    }

    match groups.len() {
        1 => symbols.push(groups[0]),
        2 => symbols.push(groups[0] * 3 + groups[1]),
        _ => {}
    }
    symbols
}

pub fn checksum_length_check(checksum: &str) -> bool {
    checksum.chars().count() == CHECKSUM_LENGTH
}

pub fn checksum_check(script: &str, checksum: &str) -> bool {
    checksum_length_check(checksum)
        && checksum.chars().all(|c| CHECKSUM_CHARSET.find(c).is_some())
        && script.chars().all(|c| INPUT_CHARSET.find(c).is_some())
        && checksum_polymod(
            checksum_expand(script)
                .into_iter()
                .chain(
                    checksum
                        .chars()
                        .map(|c| {
                            CHECKSUM_CHARSET.find(c).unwrap_or_else(|| {
                                panic!("{}", invalid_char_err_msg(CharsetKind::Checksum, c))
                            })
                        })
                        .collect::<Vec<usize>>(),
                )
                .collect::<Vec<usize>>(),
        ) == 1
}

pub fn checksum_create(script: &str) -> String {
    let symbols = checksum_expand(script)
        .into_iter()
        .chain([0; CHECKSUM_LENGTH])
        .collect::<Vec<usize>>();
    let checksum = checksum_polymod(symbols) ^ 1;

    (0..CHECKSUM_LENGTH)
        .map(|i| {
            CHECKSUM_CHARSET
                .chars()
                .nth(((checksum >> (5 * 7_usize.saturating_sub(i))) & 31) as usize)
                .unwrap_or_default()
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_checksum_create() {
        assert_eq!(checksum_create("raw(deadbeef)"), "89f8spxm");
        assert_eq!(checksum_create("raw( deadbeef )"), "985dv2zl");
        assert_eq!(checksum_create("raw(DEAD BEEF)"), "qqn7ll2h");
        assert_eq!(checksum_create("raw(DEA D BEEF)"), "egs9fwsr");
        assert_eq!(checksum_create("pkh(xpub661MyMwAqRbcFtXgS5sYJABqqG9YLmC4Q1Rdap9gSE8NqtwybGhePY2gZ29ESFjqJoCu1Rupje8YtGqsefD265TMg7usUDFdp6W1EGMcet8)"), "vm4xc4ed");
        assert_eq!(checksum_create("pkh(   xpub661MyMwAqRbcFtXgS5sYJABqqG9YLmC4Q1Rdap9gSE8NqtwybGhePY2gZ29ESFjqJoCu1Rupje8YtGqsefD265TMg7usUDFdp6W1EGMcet8)"), "ujpe9npc");
        assert_eq!(checksum_create("multi(2, xpub661MyMwAqRbcFtXgS5sYJABqqG9YLmC4Q1Rdap9gSE8NqtwybGhePY2gZ29ESFjqJoCu1Rupje8YtGqsefD265TMg7usUDFdp6W1EGMcet8, xpub661MyMwAqRbcFW31YEwpkMuc5THy2PSt5bDMsktWQcFF8syAmRUapSCGu8ED9W6oDMSgv6Zz8idoc4a6mr8BDzTJY47LJhkJ8UB7WEGuduB)"), "5jlj4shz");
    }

    #[test]
    fn test_checksum_check() {
        assert!(checksum_check("raw(deadbeef)", "89f8spxm"));
        assert!(checksum_check("raw( deadbeef )", "985dv2zl"));
        assert!(checksum_check("raw(DEAD BEEF)", "qqn7ll2h"));
        assert!(checksum_check("raw(DEA D BEEF)", "egs9fwsr"));
        assert!(checksum_check("pkh(xpub661MyMwAqRbcFtXgS5sYJABqqG9YLmC4Q1Rdap9gSE8NqtwybGhePY2gZ29ESFjqJoCu1Rupje8YtGqsefD265TMg7usUDFdp6W1EGMcet8)", "vm4xc4ed"));
        assert!(checksum_check("pkh(   xpub661MyMwAqRbcFtXgS5sYJABqqG9YLmC4Q1Rdap9gSE8NqtwybGhePY2gZ29ESFjqJoCu1Rupje8YtGqsefD265TMg7usUDFdp6W1EGMcet8)", "ujpe9npc"));
        assert!(checksum_check("multi(2, xpub661MyMwAqRbcFtXgS5sYJABqqG9YLmC4Q1Rdap9gSE8NqtwybGhePY2gZ29ESFjqJoCu1Rupje8YtGqsefD265TMg7usUDFdp6W1EGMcet8, xpub661MyMwAqRbcFW31YEwpkMuc5THy2PSt5bDMsktWQcFF8syAmRUapSCGu8ED9W6oDMSgv6Zz8idoc4a6mr8BDzTJY47LJhkJ8UB7WEGuduB)", "5jlj4shz"));
    }
}
