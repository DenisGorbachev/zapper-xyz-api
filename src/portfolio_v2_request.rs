use crate::portfolio_v2_query_types::Variables;
use crate::{Address, ChainId, PortfolioPageSize};
use errgonomic::handle_bool;
use serde::{Deserialize, Serialize};
use std::ops::Not;
use thiserror::Error;

#[derive(Serialize, Deserialize, Eq, PartialEq, Clone, Debug)]
pub struct PortfolioV2Request {
    pub addresses: Vec<Address>,
    pub chain_ids: Vec<ChainId>,
    pub first: PortfolioPageSize,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub after: Option<String>,
}

impl PortfolioV2Request {
    pub fn new(addresses: Vec<Address>, chain_ids: Vec<ChainId>, first: PortfolioPageSize) -> Result<Self, PortfolioV2RequestNewError> {
        use PortfolioV2RequestNewError::*;
        let request = Self {
            addresses,
            chain_ids,
            first,
            after: None,
        };
        handle_bool!(request.addresses.is_empty(), AddressesEmptyInvalid, request);
        Ok(request)
    }

    pub fn variables(&self) -> Result<Variables, PortfolioV2RequestVariablesError> {
        use PortfolioV2RequestVariablesError::*;
        handle_bool!(self.addresses.is_empty(), AddressesEmptyInvalid);
        Ok(Variables {
            addresses: self.addresses.clone(),
            chain_ids: self
                .chain_ids
                .is_empty()
                .not()
                .then(|| self.chain_ids.iter().copied().map(ChainId::get).collect()),
            first: self.first.get(),
            after: self.after.clone(),
        })
    }

    pub fn set_after_string(&mut self, after: String) -> Result<(), PortfolioV2RequestSetAfterStringError> {
        use PortfolioV2RequestSetAfterStringError::*;
        handle_bool!(after.is_empty(), EmptyInvalid, after);
        self.after = Some(after);
        Ok(())
    }
}

#[derive(Error, Debug)]
pub enum PortfolioV2RequestNewError {
    #[error("portfolioV2 request must contain at least one address")]
    AddressesEmptyInvalid { request: PortfolioV2Request },
}

#[derive(Error, Debug)]
pub enum PortfolioV2RequestSetAfterStringError {
    #[error("portfolio cursor must not be empty")]
    EmptyInvalid { after: String },
}

#[derive(Error, Debug)]
pub enum PortfolioV2RequestVariablesError {
    #[error("portfolioV2 request must contain at least one address")]
    AddressesEmptyInvalid,
}
