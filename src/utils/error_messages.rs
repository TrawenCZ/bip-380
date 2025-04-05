pub fn missing_follow_up_val_err(key: &str) -> String {
    format!("Missing follow-up value after flag '{key}'!")
}

pub fn multiple_value_flags_detected_err(key: &str) -> String {
    format!("Multiple flags '{key}' found. You can only specify flag with a value once!")
}

pub fn invalid_seed_length_err(seed_no_whitespace: &str) -> String {
    format!("The provided seed '{seed_no_whitespace}' doesn't have even length and thus cannot be complete valid hexadecimal number representation")
}

pub fn script_arg_extraction_err(label: &str) -> String {
    format!("Could not extract arguments from '{label}' expression.")
}

pub fn script_sh_unsupported_arg_err(arg: &str) -> String {
    format!("'sh' script's argument must be either 'pk', 'pkh' or 'multi' scripts, but '{arg}' was given.")
}

pub const MISSING_INPUT_ERR_MSG: &str = "No input argument provided. You must provide at least one input argument or include '-' to read from standard input.";

pub const MISSING_ARG_ERR_MSG: &str = "No argument provided. Please specify the sub-command.";
