use errgonomic::{handle, handle_bool, handle_opt};
use serde::{Deserialize, Serialize};
use std::ops::Not;
use std::str::FromStr;
use thiserror::Error;

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
        let body = handle_opt!(value.strip_prefix("0x"), PrefixInvalid, value);
        handle_bool!(value.len() != 42, LengthInvalid, value);
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

impl FromStr for Address {
    type Err = AddressFromStrError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        use AddressFromStrError::*;
        let value = handle!(Self::try_from(input.to_owned()), TryFromFailed, input: input.to_owned());
        Ok(value)
    }
}

#[derive(Error, Debug)]
pub enum AddressFromStrError {
    #[error("failed to parse address '{input}'")]
    TryFromFailed { source: ConvertStringToAddressError, input: String },
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
