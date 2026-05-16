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

- Must use `graphql_client` to execute requests
- Must contain `graphql` dir
  - Must contain `schema.graphql` file
  - Must contain `queries.graphql` file
    - Must contain all queries
- Must implement pagination for queries
- Must implement rate limiting for queries
- Must define Rust types for the following GraphQL scalar types from `schema.graphql`:
  - `Address`

## `zapper-xyz-api` bin crate

A Rust crate that provides a CLI for zapper.xyz API.

- Must contain [Command](#command)
- Must contain [PortfolioCommand](#portfoliocommand)

## Key

A type alias for API key as `secrecy::SecretString`.

## Client

A Rust struct that contains the fields for data that is shared between API requests.

Requirements:

- Must have attributes:
  - `#[derive(Clone, Debug)]`
- Must have fields:
  - `pub inner: HttpClient` (`use reqwest::Client as HttpClient;`)
  - `pub base: Url`
  - `pub limits: RateLimits`
- Must have methods:
  - `pub fn new(key: impl Into<Key>) -> Result<Self, ClientNewError>`
    - Must call `Self::try_from`
  - `pub fn default_base_url() -> Url`
    - `url!("https://public.zapper.xyz/graphql")` (use `url-macro` crate)
- Must have impls:
  - `TryFrom<Key>`
    - Must call `Self::try_from((key, Self::default_base_url()))`
  - `TryFrom<(Key, Url)>`
    - Must construct `inner` client
      - Must set the `x-zapper-api-key` header via `default_headers`
        - Must mark the header as sensitive
    - Must call `Self::from((inner, base))`
  - `From<(HttpClient, Url)>`
  - `From<(HttpClient, Url, RateLimits)>`

## RateLimits

A Rust struct that has one field per limit in [rate limits](./docs/build.zapper.xyz/rate-limits.md).

- Must have attributes:
  - `#[derive(Debug)]`
- Every field must be a `LazyCell<DefaultDirectRateLimiter>` from `governor`
- Must have an `impl Default`
  - Must construct rate limiters according to documentation

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
