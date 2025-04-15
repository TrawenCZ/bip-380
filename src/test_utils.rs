#[cfg(test)]
use assert_cmd::Command;

#[cfg(test)]
pub fn get_cmd() -> Command {
    Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap()
}