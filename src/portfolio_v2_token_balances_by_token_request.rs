use crate::portfolio_v2_token_balances_by_token_types::Variables;
use crate::{Address, ChainId, PageSize};
use derive_new::new;
use errgonomic::handle;
use non_empty_str::{EmptyString, NonEmptyString};
use serde::{Deserialize, Serialize};
use std::ops::Not;
use thiserror::Error;

#[derive(new, Serialize, Deserialize, Eq, PartialEq, Clone, Debug)]
pub struct PortfolioV2TokenBalancesByTokenRequest {
    pub address: Address,
    pub chain_ids: Vec<ChainId>,
    pub first: PageSize,
    #[new(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub after: Option<NonEmptyString>,
}

impl PortfolioV2TokenBalancesByTokenRequest {
    pub fn variables(&self) -> Variables {
        Variables {
            addresses: vec![self.address.clone()],
            chain_ids: self
                .chain_ids
                .is_empty()
                .not()
                .then(|| self.chain_ids.iter().copied().map(ChainId::get).collect()),
            first: i64::from(self.first),
            after: self.after.clone().map(String::from),
        }
    }

    pub fn set_after_string(&mut self, after: String) -> Result<(), PortfolioV2TokenBalancesByTokenRequestSetAfterStringError> {
        use PortfolioV2TokenBalancesByTokenRequestSetAfterStringError::*;
        let after = handle!(NonEmptyString::try_from(after), NonEmptyStringTryFromFailed);
        self.set_after(after);
        Ok(())
    }

    pub fn set_after(&mut self, after: NonEmptyString) {
        self.after = Some(after);
    }
}

#[derive(Error, Debug)]
pub enum PortfolioV2TokenBalancesByTokenRequestSetAfterStringError {
    #[error("failed to construct non-empty portfolioV2 token page cursor")]
    NonEmptyStringTryFromFailed { source: EmptyString },
}
