use errgonomic::{handle_bool, handle_opt};
use serde::{Deserialize, Serialize};
use std::ops::Not;
use thiserror::Error;

const ADDRESS_PREFIX: &str = "0x";
const ADDRESS_LENGTH: usize = 42;

#[derive(Serialize, Deserialize, Eq, PartialEq, Hash, Clone, Debug)]
#[serde(try_from = "String", into = "String")]
pub struct Address(String);

impl From<Address> for String {
    fn from(value: Address) -> Self {
        value.0
    }
}

impl TryFrom<String> for Address {
    type Error = ConvertStringToAddressError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        use ConvertStringToAddressError::*;
        let body = handle_opt!(value.strip_prefix(ADDRESS_PREFIX), PrefixInvalid, value);
        handle_bool!(value.len() != ADDRESS_LENGTH, LengthInvalid, value);
        handle_bool!(
            body.chars()
                .all(|character| character.is_ascii_hexdigit())
                .not(),
            CharactersInvalid,
            value
        );
        Ok(Self(value))
    }
}

#[derive(Error, Debug)]
pub enum ConvertStringToAddressError {
    #[error("address must start with '0x'")]
    PrefixInvalid { value: String },
    #[error("address must contain 42 characters")]
    LengthInvalid { value: String },
    #[error("address must contain only hexadecimal characters after '0x'")]
    CharactersInvalid { value: String },
}
