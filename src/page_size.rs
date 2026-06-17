use crate::MAX_PAGE_SIZE;
use core::num::NonZeroU8;
use errgonomic::{handle, handle_bool, handle_opt};
use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};
use std::num::{ParseIntError, TryFromIntError};
use std::str::FromStr;
use thiserror::Error;

#[derive(Serialize, Deserialize, Eq, PartialEq, Hash, Copy, Clone, Debug)]
#[serde(try_from = "u64", into = "u64")]
pub struct PageSize(NonZeroU8);

impl PageSize {
    pub fn get(self) -> u8 {
        self.0.get()
    }
}

impl Default for PageSize {
    fn default() -> Self {
        let value = NonZeroU8::new(MAX_PAGE_SIZE).expect("always succeeds because the max page size is non-zero");
        Self(value)
    }
}

impl Display for PageSize {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        Display::fmt(&self.get(), f)
    }
}

impl From<PageSize> for u64 {
    fn from(value: PageSize) -> Self {
        u64::from(value.get())
    }
}

impl From<PageSize> for i64 {
    fn from(value: PageSize) -> Self {
        i64::from(value.get())
    }
}

impl FromStr for PageSize {
    type Err = PageSizeFromStrError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        use PageSizeFromStrError::*;
        let value = handle!(input.parse::<u64>(), ParseFailed, input: input.to_owned());
        let value = handle!(Self::try_from(value), TryFromFailed, input: input.to_owned());
        Ok(value)
    }
}

#[derive(Error, Debug)]
pub enum PageSizeFromStrError {
    #[error("failed to parse page size")]
    ParseFailed { source: ParseIntError, input: String },
    #[error("failed to convert parsed page size")]
    TryFromFailed { source: ConvertU64ToPageSizeError, input: String },
}

impl TryFrom<u64> for PageSize {
    type Error = ConvertU64ToPageSizeError;

    fn try_from(value: u64) -> Result<Self, Self::Error> {
        use ConvertU64ToPageSizeError::*;
        handle_bool!(value > u64::from(MAX_PAGE_SIZE), TooLargeInvalid);
        let value = handle!(u8::try_from(value), U8TryFromFailed);
        let value = handle_opt!(NonZeroU8::new(value), ZeroInvalid);
        Ok(Self(value))
    }
}

#[derive(Error, Debug, Copy, Clone)]
pub enum ConvertU64ToPageSizeError {
    #[error("page size must be positive")]
    ZeroInvalid,
    #[error("page size must be at most '{max}'", max = MAX_PAGE_SIZE)]
    TooLargeInvalid,
    #[error("failed to convert page size to u8")]
    U8TryFromFailed { source: TryFromIntError },
}
