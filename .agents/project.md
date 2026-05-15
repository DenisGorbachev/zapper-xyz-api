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

## `graphql` dir

- Must contain `schema.graphql`
- Must contain `queries.graphql`
  - Must contain all queries

## `zapper-xyz-api` lib crate

A Rust crate that exports the types related to zapper.xyz API.

Requirements:

- Must use `graphql_client` to execute requests
- Must implement pagination for queries
- Must implement rate limiting for queries

## `zapper-xyz-api` bin crate

A Rust crate that provides a CLI for zapper.xyz API.

## Query struct

A struct that derives `GraphQLQuery`.

- Must have the following `response_derives`:
  - `PartialEq`
  - `Clone`
  - `Debug`
