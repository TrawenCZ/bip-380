pub const HELP_MESSAGE: &str = "\
BIP 380

Usage:
    derive-key {value} [--path {path}] [-]

    The derive-key sub-command takes one required positional argument {value}
    (with one exception, see below), which can be either a seed, or Base58 encoded
    extended public {xpub} or private key {xpriv}.

    Depending on the type of the input {value} the utility outputs certain extended keys.
    - On input seed, outputs the master private and public key.
    - On input extended private key, outputs the extended private key, i.e., echos
      itself, and the corresponding extended public key.
    - On input extended public key, outputs the extended public key, i.e., echos
      itself.

    Valid seed {value} is a byte sequence of length between 128 and 512 bits
    represented as case-insensitive hexadecimal values. The space character ' '
    or the tab character (sometimes denoted as '\\t') can be used to separate
    the individual hexadecimal values.

    If a single dash '-' parameter is present, it indicates reading the {value}
    from the standard input. Reading from the standard input takes precendence over
    {value} provided as a command-line argument (in that case the {value}
    argument is ignored). When reading from standard input, each line of the file is
    processed as a single {value} with all the previous rules on {value} still applicable.


    --path {path}   The {path} value is a sequence of /NUM and /NUMh, where NUM is from the range
                    [0,...,2^31-1] as described in BIP 32. The path does not need to start with /.
                    In the hardened version /NUMh the h indentifier can also be substituted with H
                     or ' and these can also be mixed within a single path.



    key-expression {expr} [-]

    The key-expression parses the {expr} according to the BIP 380 Key Expressions specification
    (https://github.com/bitcoin/bips/blob/master/bip-0380.mediawiki#key-expressions). If there 
    are no parsing errors, the key expression is echoed back on a single line with 0 exit code.

    If a single dash '-' parameter is present, it indicates reading the {expr}
    from the standard input. Similar rules as described for the previous
    derive-key sub-command apply, such as, the standard input takes precendence and is
    processed line by line, etc.

    The key expression consists of the optional key origin information and then the
    actual key. Regarding the key types:
    - The utility will accept any hex encoded public keys that conform to the
      single-byte prefix (02, 03 or 04) and length (66 or 130) constraints.
    - Wallet Import Format (WIF) encoded private keys parsing and checking, see
      this wiki page - https://en.bitcoin.it/wiki/Wallet_import_format. Only expected WIF encoded
      private keys, are private keys originating as random 32 bytes and encoded using the Private 
      key to WIF routine (from the previous links). Also, the first byte in the 4th step in WIF to
      private key routine is expected to be 0x80.
    - Finally, extended public and private keys must be checked using the same BIP 32 library that
      you were using in derive-key already.



    script-expression {expr} [-]

    The script-expression sub-command implements parsing of some of the script
    expressions and optionally also checksum verification and calculation. The
    expected format of {expr} is described in BIP 380 specification
    (https://github.com/bitcoin/bips/blob/master/bip-0380.mediawiki#specification)
    and it is SCRIPT#CHECKSUM, where the SCRIPT can have one of
    the following formats (not everything from BIP 380 and the following bips is
    supported):

      pk(KEY)
      pkh(KEY)
      multi(k, KEY_1, KEY_2, ..., KEY_n)
      sh(pk(KEY))
      sh(pkh(KEY))
      sh(multi(k, KEY_1, KEY_2, ..., KEY_n))
      raw(HEX)

    If a single dash '-' parameter is present, it indicates reading the {expr}
    from the standard input. Similar rules as described for the previous
    derive-key sub-command apply, such as, the standard input takes precendence
    and is processed line by line, etc.

    --verify-checksum   If this option is used, then the checksum is 
                        expected and is verified by recalculating the checksum over 
                        SCRIPT (everything up to, not including the octothorpe #). The 
                        output is OK if the checksum verifies.

    --compute-checksum  If this option is used, then the #CHECKSUM, if provided, is 
                        ignored and new CHECKSUM is computed. The output is then the
                        original script and the checksum in the form SCRIPT#CHECKSUM.

    Note that mixing --verify-checksum and --compute-checksum options leads to an error.


The option --help displays this descriptive help message regarding the sub-comands and
flags. When --help is used it takes precendence over any other command-line arguments.";
