use std::env;

use arg_parser::Command;
mod arg_parser;
mod structs;
mod traits;

fn main() {
    // collect args from the CLI skipping the first argument, which is the name of the program
    let args: Vec<String> = env::args().skip(1).collect();

    // convert the Vec<String> to Vec<&str>
    let args: Vec<&str> = args.iter().map(|s| s.as_str()).collect();

    let (command, inputs) = arg_parser::parse_args(args).unwrap_or_else(|err| {
        eprintln!("{}", err);
        std::process::exit(1);
    });

    if command == Command::Help {
        println!("Help message");
        std::process::exit(0);
    }

    for input in inputs {
        println!("{}", input);
        match command {
            Command::KeyExpression(_) => {
                println!("Running Key expression subcommand with input: {}", input);
            }
            Command::ScriptExpression(_) => {
                println!("Running Script expression subcommand with input: {}", input);
            }
            Command::DeriveKey(_) => {
                println!("Running Derive key subcommand with input: {}", input);
            }
            Command::Help => { /* Already handled */ }
        }
    }
}
