use graphql_client::GraphQLQuery;

pub use crate::Address;

#[derive(GraphQLQuery)]
#[graphql(schema_path = "graphql/schema.graphql", query_path = "graphql/queries.graphql", response_derives = "PartialEq, Clone, Debug, serde::Serialize")]
pub struct PortfolioV2TokenBalancesByToken;
