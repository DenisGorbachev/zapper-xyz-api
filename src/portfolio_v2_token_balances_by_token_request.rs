use crate::portfolio_v2_token_balances_by_token_types::Variables;
use crate::{Address, ChainId, PortfolioPageSize};
use errgonomic::handle_bool;
use serde::{Deserialize, Serialize};
use std::ops::Not;
use thiserror::Error;

#[derive(Serialize, Deserialize, Eq, PartialEq, Clone, Debug)]
pub struct PortfolioV2TokenBalancesByTokenRequest {
    pub address: Address,
    pub chain_ids: Vec<ChainId>,
    pub first: PortfolioPageSize,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub after: Option<String>,
}

impl PortfolioV2TokenBalancesByTokenRequest {
    pub fn new(address: Address, chain_ids: Vec<ChainId>, first: PortfolioPageSize) -> Self {
        Self {
            address,
            chain_ids,
            first,
            after: None,
        }
    }

    pub fn variables(&self) -> Result<Variables, PortfolioV2TokenBalancesByTokenRequestVariablesError> {
        use PortfolioV2TokenBalancesByTokenRequestVariablesError::*;
        handle_bool!(self.after.as_ref().is_some_and(String::is_empty), AfterEmptyInvalid);
        Ok(Variables {
            addresses: vec![self.address.clone()],
            chain_ids: self
                .chain_ids
                .is_empty()
                .not()
                .then(|| self.chain_ids.iter().copied().map(ChainId::get).collect()),
            first: self.first.get(),
            after: self.after.clone(),
        })
    }

    pub fn set_after_string(&mut self, after: String) -> Result<(), PortfolioV2TokenBalancesByTokenRequestSetAfterStringError> {
        use PortfolioV2TokenBalancesByTokenRequestSetAfterStringError::*;
        handle_bool!(after.is_empty(), EmptyInvalid, after);
        self.after = Some(after);
        Ok(())
    }
}

#[derive(Error, Debug)]
pub enum PortfolioV2TokenBalancesByTokenRequestSetAfterStringError {
    #[error("portfolio token cursor must not be empty")]
    EmptyInvalid { after: String },
}

#[derive(Error, Debug, Copy, Clone)]
pub enum PortfolioV2TokenBalancesByTokenRequestVariablesError {
    #[error("portfolioV2 token cursor must not be empty")]
    AfterEmptyInvalid,
}
