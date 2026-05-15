use errgonomic::handle_bool;
use secrecy::SecretString;
use thiserror::Error;

pub type Key = SecretString;

pub fn parse_key(input: &str) -> Result<Key, ParseKeyError> {
    use ParseKeyError::*;
    handle_bool!(input.is_empty(), EmptyInvalid);
    Ok(Key::from(input))
}

#[derive(Error, Debug)]
pub enum ParseKeyError {
    #[error("Zapper API key must not be empty")]
    EmptyInvalid,
}
