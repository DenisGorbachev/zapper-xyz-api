use crate::portfolio_v2_token_balances_by_token_types::Variables;
use crate::{Address, ChainId, CursorPaginatedRequest, PageSize};
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

impl From<PortfolioV2TokenBalancesByTokenRequest> for Variables {
    fn from(request: PortfolioV2TokenBalancesByTokenRequest) -> Self {
        let PortfolioV2TokenBalancesByTokenRequest {
            address,
            chain_ids,
            first,
            after,
        } = request;
        Self {
            addresses: vec![address],
            chain_ids: chain_ids.map(|chain_ids| chain_ids.into_iter().map(ChainId::get).collect()),
            first: i64::from(first),
            after,
        }
    }
}

impl CursorPaginatedRequest for PortfolioV2TokenBalancesByTokenRequest {
    fn cursor_after(&self) -> Option<&str> {
        self.after.as_deref()
    }

    fn set_cursor_after(&mut self, after: String) {
        self.set_after(after);
    }
}
