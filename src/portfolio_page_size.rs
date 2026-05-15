use core::num::NonZeroI64;
use errgonomic::{handle, handle_bool, handle_opt};
use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};
use std::num::ParseIntError;
use std::str::FromStr;
use thiserror::Error;

pub const MAX_PORTFOLIO_PAGE_SIZE: i64 = 100;

#[derive(Serialize, Deserialize, Eq, PartialEq, Hash, Copy, Clone, Debug)]
#[serde(try_from = "i64", into = "i64")]
pub struct PortfolioPageSize(NonZeroI64);

impl PortfolioPageSize {
    pub fn get(self) -> i64 {
        self.0.get()
    }
}

impl Default for PortfolioPageSize {
    fn default() -> Self {
        let value = NonZeroI64::new(MAX_PORTFOLIO_PAGE_SIZE).expect("always succeeds because the max portfolio page size is non-zero");
        Self(value)
    }
}

impl Display for PortfolioPageSize {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        Display::fmt(&self.get(), f)
    }
}

impl From<PortfolioPageSize> for i64 {
    fn from(value: PortfolioPageSize) -> Self {
        value.get()
    }
}

impl FromStr for PortfolioPageSize {
    type Err = PortfolioPageSizeFromStrError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        use PortfolioPageSizeFromStrError::*;
        let value = handle!(input.parse::<i64>(), ParseFailed, input: input.to_owned());
        let value = handle!(Self::try_from(value), TryFromFailed, input: input.to_owned());
        Ok(value)
    }
}

#[derive(Error, Debug)]
pub enum PortfolioPageSizeFromStrError {
    #[error("failed to parse portfolio page size")]
    ParseFailed { source: ParseIntError, input: String },
    #[error("failed to convert parsed portfolio page size")]
    TryFromFailed { source: ConvertI64ToPortfolioPageSizeError, input: String },
}

impl TryFrom<i64> for PortfolioPageSize {
    type Error = ConvertI64ToPortfolioPageSizeError;

    fn try_from(value: i64) -> Result<Self, Self::Error> {
        use ConvertI64ToPortfolioPageSizeError::*;
        handle_bool!(value.is_negative(), NegativeInvalid);
        let value = handle_opt!(NonZeroI64::new(value), ZeroInvalid);
        handle_bool!(value.get() > MAX_PORTFOLIO_PAGE_SIZE, TooLargeInvalid);
        Ok(Self(value))
    }
}

#[derive(Error, Debug, Copy, Clone)]
pub enum ConvertI64ToPortfolioPageSizeError {
    #[error("portfolio page size must be positive")]
    NegativeInvalid,
    #[error("portfolio page size must be positive")]
    ZeroInvalid,
    #[error("portfolio page size must be at most '{max}'", max = MAX_PORTFOLIO_PAGE_SIZE)]
    TooLargeInvalid,
}
