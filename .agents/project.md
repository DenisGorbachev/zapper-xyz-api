# Zapper API concepts

## `zapper-xyz-api`

A Rust package with the following crates:

- [`zapper-xyz-api` lib crate](#zapper-xyz-api-lib-crate)
- [`zapper-xyz-api` bin crate](#zapper-xyz-api-bin-crate)

Requirements:

- Must contain files:
  - [`Cargo.toml`](#cargotoml)

## Cargo.toml

- Must have dependencies:
  - `graphql_client`
  - `governor`
  - `errgonomic`
  - `serde`
- May have dependencies:
  - `strum`
- Every version under `dependencies` key must be specified only up to the first non-zero part (good: "1", "0.3", bad: "1.0", "0.3.3")

## `zapper-xyz-api` lib crate

A Rust crate that exports the types related to zapper.xyz API.

Requirements:

- Must define `struct Client`
- Must define `struct RateLimits`
- Must use `graphql_client` to execute requests
- Must contain `graphql` dir
  - Must contain `schema.graphql` file
  - Must contain `queries.graphql` file
    - Must contain all queries
- Must implement pagination for queries
- Must implement rate limiting for queries
- Must define Rust types for the following GraphQL scalar types from `schema.graphql`:
  - `Address`
  - `ChainId`

## `zapper-xyz-api` bin crate

A Rust crate that provides a CLI for zapper.xyz API.

- Must contain [Command](#command)
- Must contain [PortfolioCommand](#portfoliocommand)

## src/lib.rs

- Must have a crate-level doc comment:
  ```text
  Zapper API has a bug: it doesn't return the tokens with missing prices even with `includeTokensWithMissingPrices: true`. The final non-empty page’s cursor decodes to "ec6d06c9426495f2fffae17618ab5826:0". It is effectively "{portfolio-id}:{balanceUSD}", so every zero-USD token shares the same cursor value. See also: "Zapper API totalCount investigation" thread.
  ```

## Query struct

A struct that derives `GraphQLQuery`.

- Must have the following `response_derives`:
  - `PartialEq`
  - `Clone`
  - `Debug`

## `PortfolioV2TokenBalancesByToken` query

A GraphQL query that returns all token balances for one account.

- Must use `portfolioV2` edge with exactly one address in the `addresses` argument
- Must use `tokenBalances` edge
- Must use `byToken` edge
- Must set `filters.includeTokensWithMissingPrices` to `true`
- Must paginate the `byToken` edge
- Must return the token balance fields needed to stream complete token balance pages

## Command

A Rust struct that represents a CLI command.

- Must have fields:
  - `key: Key`
- Must have methods:
  - `run`
    - Must construct `client`
    - Must pass the `client` by value to the subcommand

## PortfolioCommand

- Must have fields:
  - `addresses: Vec<String>`
- Must have methods:
  - `run`
    - Must iterate over `addresses` and send requests for [`PortfolioV2TokenBalancesByToken` query](#portfoliov2tokenbalancesbytoken-query), passing exactly one address per request
    - Must paginate all token balances for each address
    - Must stream the results to `stdout` as soon as they are available

## turn_cursor_page

A Rust function that implements pagination for Zapper API.

- Must stop if the new cursor is equal to the previous cursor (this is a bug in the API).
