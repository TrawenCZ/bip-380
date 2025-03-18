use std::env;
mod arg_parser;
mod structs;
mod traits;

fn main() {
    // collect args from the CLI skipping the first argument, which is the name of the program
    let args: Vec<String> = env::args().skip(1).collect();

    // convert the Vec<String> to Vec<&str>
    let args: Vec<&str> = args.iter().map(|s| s.as_str()).collect();

    #[allow(unused_variables)]
    let command = arg_parser::parse_args(args).unwrap_or_else(|err| {
        eprintln!("{}", err);
        std::process::exit(1);
    });
}
