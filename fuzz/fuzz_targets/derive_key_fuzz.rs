#![no_main]

use libfuzzer_sys::fuzz_target;

fuzz_target!(|args: Vec<String>| {
    let args = vec!["derive-key".to_string()]
        .into_iter()
        .chain(args.into_iter())
        .filter(|arg| arg != "-")
        .collect::<Vec<_>>();

    let str_args: Vec<&str> = args.iter().map(|s| s.as_str()).collect();

    let _ = bip380::run_cli(str_args);
});