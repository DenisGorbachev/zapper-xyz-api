use core::num::NonZeroI64;
use errgonomic::{handle, handle_bool, handle_opt};
use serde::{Deserialize, Serialize};
use std::fmt::{self, Display, Formatter};
use std::num::ParseIntError;
use std::str::FromStr;
use thiserror::Error;

#[derive(Serialize, Deserialize, Eq, PartialEq, Hash, Copy, Clone, Debug)]
#[serde(try_from = "i64", into = "i64")]
pub struct ChainId(NonZeroI64);

impl ChainId {
    pub fn get(self) -> i64 {
        self.0.get()
    }
}

impl Display for ChainId {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        Display::fmt(&self.get(), f)
    }
}

impl From<ChainId> for i64 {
    fn from(value: ChainId) -> Self {
        value.get()
    }
}

impl FromStr for ChainId {
    type Err = ChainIdFromStrError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        use ChainIdFromStrError::*;
        let value = handle!(input.parse::<i64>(), ParseFailed, input: input.to_owned());
        let value = handle!(Self::try_from(value), TryFromFailed, input: input.to_owned());
        Ok(value)
    }
}

#[derive(Error, Debug)]
pub enum ChainIdFromStrError {
    #[error("failed to parse chain ID")]
    ParseFailed { source: ParseIntError, input: String },
    #[error("failed to convert parsed chain ID")]
    TryFromFailed { source: ConvertI64ToChainIdError, input: String },
}

impl TryFrom<i64> for ChainId {
    type Error = ConvertI64ToChainIdError;

    fn try_from(value: i64) -> Result<Self, Self::Error> {
        use ConvertI64ToChainIdError::*;
        handle_bool!(value.is_negative(), NegativeInvalid);
        let value = handle_opt!(NonZeroI64::new(value), ZeroInvalid);
        Ok(Self(value))
    }
}

#[derive(Error, Debug, Copy, Clone)]
pub enum ConvertI64ToChainIdError {
    #[error("chain ID must be positive")]
    NegativeInvalid,
    #[error("chain ID must be positive")]
    ZeroInvalid,
}
