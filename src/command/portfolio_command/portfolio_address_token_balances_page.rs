use crate::Address;
use serde::Serialize;

#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct PortfolioAddressTokenBalancesPage<'a, T: Serialize> {
    pub address: &'a Address,
    pub total_balance_usd: f64,
    pub token_balances: &'a T,
}
