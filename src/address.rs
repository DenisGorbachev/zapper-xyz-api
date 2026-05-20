use derive_more::{AsRef, Display, From, Into};
use errgonomic::handle;
use non_empty_str::{EmptyString, NonEmptyString};
use serde::{Deserialize, Serialize};
use std::str::FromStr;
use thiserror::Error;

#[derive(AsRef, Display, From, Into, Serialize, Deserialize, Eq, PartialEq, Hash, Clone, Debug)]
#[as_ref(NonEmptyString, String, str)]
#[into(NonEmptyString, String)]
#[serde(transparent)]
pub struct Address(NonEmptyString);

impl TryFrom<String> for Address {
    type Error = ConvertStringToAddressError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        use ConvertStringToAddressError::*;
        let value = handle!(NonEmptyString::try_from(value), NonEmptyStringTryFromFailed);
        Ok(Self::from(value))
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
    #[error("failed to construct non-empty Zapper address")]
    NonEmptyStringTryFromFailed { source: EmptyString },
}
