use crate::portfolio_v2_token_balances_by_token_types::Variables;
use crate::{Address, ChainId, PageSize};
use derive_new::new;
use derive_setters::Setters;
use serde::{Deserialize, Serialize};

#[derive(new, Setters, Serialize, Deserialize, Eq, PartialEq, Clone, Debug)]
#[setters(prefix = "set_", borrow_self, into, strip_option)]
pub struct PortfolioV2TokenBalancesByTokenRequest {
    pub address: Address,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chain_ids: Option<Vec<ChainId>>,
    pub first: PageSize,
    #[new(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub after: Option<String>,
}

impl PortfolioV2TokenBalancesByTokenRequest {
    pub fn variables(&self) -> Variables {
        Variables {
            addresses: vec![self.address.clone()],
            chain_ids: self
                .chain_ids
                .as_ref()
                .map(|chain_ids| chain_ids.iter().copied().map(ChainId::get).collect()),
            first: i64::from(self.first),
            after: self.after.clone(),
        }
    }
}
