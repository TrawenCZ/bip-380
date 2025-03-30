pub fn missing_follow_up_val_err(key: &str) -> String {
    format!("Missing follow-up value after flag '{key}'!")
}

pub fn multiple_value_flags_detected_err(key: &str) -> String {
    format!("Multiple flags '{key}' found. You can only specify flag with a value once!")
}

pub const MISSING_INPUT_ERR_MSG: &str = "No input argument provided. You must provide at least one input argument or include '-' to read from standard input.";

pub const MISSING_ARG_ERR_MSG: &str = "No argument provided. Please specify the sub-command.";
