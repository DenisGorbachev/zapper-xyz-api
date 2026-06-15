use crate::CursorPage;
use graphql_client::GraphQLQuery;

pub use crate::Address;

#[derive(GraphQLQuery)]
#[graphql(schema_path = "graphql/schema.graphql", query_path = "graphql/queries.graphql", variables_derives = "Debug", response_derives = "PartialEq, Clone, Debug, serde::Serialize")]
pub struct PortfolioV2TokenBalancesByToken;

impl CursorPage for portfolio_v2_token_balances_by_token::PortfolioV2TokenBalancesByTokenPortfolioV2TokenBalancesByToken {
    fn has_next_page(&self) -> bool {
        self.page_info.has_next_page
    }

    fn end_cursor(&self) -> Option<&str> {
        self.page_info.end_cursor.as_deref()
    }
}
