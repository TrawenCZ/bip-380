use std::{env, ffi::OsString};

use bip380::run_cli;

pub mod parsers;
pub mod structs;
pub mod subcommands;
pub mod test_utils;
pub mod traits;
pub mod utils;

// exit codes
const SUCCESS: i32 = 0;
const FAILURE: i32 = 1;

fn main() {
    // collect args from the CLI skipping the first argument, which is the name of the program
    let args: Vec<String> = env::args_os()
        .map(std::ffi::OsString::into_string)
        .skip(1)
        .collect::<Result<Vec<String>, OsString>>()
        .unwrap_or_else(|err| {
            eprintln!("Error converting argument to string: {err:?}");
            std::process::exit(FAILURE);
        });

    // convert the Vec<String> to Vec<&str>
    let args: Vec<&str> = args.iter().map(std::string::String::as_str).collect();

    let exit_code = match run_cli(args) {
        Ok(()) => SUCCESS,
        Err(code) => code,
    };

    std::process::exit(exit_code);
}
