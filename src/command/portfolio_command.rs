use crate::{Address, ChainId, Client, ClientPortfolioV2Error, ConvertStringToAddressError, PortfolioPageSize, PortfolioV2Request, PortfolioV2RequestNewError, PortfolioV2RequestSetAfterStringError};
use clap::Parser;
use errgonomic::{ErrVec, handle, handle_iter, handle_opt};
use serde::Serialize;
use std::io::{BufWriter, Write, stdout};
use std::process::ExitCode;
use thiserror::Error;

#[derive(Parser, Clone, Debug)]
pub struct PortfolioCommand {
    #[arg(num_args = 1.., required = true)]
    pub addresses: Vec<String>,
    #[arg(long = "chain-id")]
    pub chain_ids: Vec<ChainId>,
    #[arg(long, default_value_t = PortfolioPageSize::default())]
    pub first: PortfolioPageSize,
}

impl PortfolioCommand {
    pub async fn run(self, client: Client) -> Result<ExitCode, PortfolioCommandRunError> {
        use PortfolioCommandRunError::*;
        let Self {
            addresses,
            chain_ids,
            first,
        } = self;
        let addresses = handle!(parse_addresses(addresses), ParseAddressesFailed);
        let mut request = handle!(PortfolioV2Request::new(addresses, chain_ids, first), PortfolioV2RequestNewFailed);
        let mut stdout = BufWriter::new(stdout().lock());
        loop {
            let page_request = request.clone();
            let data = handle!(client.portfolio_v2(page_request).await, PortfolioV2Failed);
            let token_balances = data.portfolio_v2.token_balances;
            let by_token = token_balances.by_token;
            handle!(
                by_token
                    .edges
                    .iter()
                    .map(|edge| &edge.node)
                    .try_for_each(|token_balance| write_json_line(&mut stdout, token_balance)),
                WriteJsonLineFailed
            );
            handle!(stdout.flush(), FlushStdoutFailed);
            let page_info = by_token.page_info;
            if page_info.has_next_page {
                let after = handle_opt!(page_info.end_cursor, PageEndCursorNotFound, request);
                handle!(request.set_after_string(after), PortfolioV2RequestSetAfterStringFailed);
            } else {
                break;
            }
        }
        Ok(ExitCode::SUCCESS)
    }
}

#[derive(Error, Debug)]
pub enum PortfolioCommandRunError {
    #[error("failed to parse portfolio addresses")]
    ParseAddressesFailed { source: ParseAddressesError },
    #[error("failed to construct portfolioV2 request")]
    PortfolioV2RequestNewFailed { source: PortfolioV2RequestNewError },
    #[error("failed to query portfolioV2")]
    PortfolioV2Failed { source: ClientPortfolioV2Error },
    #[error("failed to set portfolioV2 page cursor")]
    PortfolioV2RequestSetAfterStringFailed { source: PortfolioV2RequestSetAfterStringError },
    #[error("failed to write token balance")]
    WriteJsonLineFailed { source: WriteJsonLineError },
    #[error("portfolioV2 page info did not contain an end cursor")]
    PageEndCursorNotFound { request: PortfolioV2Request },
    #[error("failed to flush stdout")]
    FlushStdoutFailed { source: std::io::Error },
}

pub fn parse_addresses(addresses: Vec<String>) -> Result<Vec<Address>, ParseAddressesError> {
    use ParseAddressesError::*;
    let addresses = addresses.into_iter().map(Address::try_from);
    Ok(handle_iter!(addresses, ConvertStringsToAddressesFailed))
}

#[derive(Error, Debug)]
pub enum ParseAddressesError {
    #[error("failed to convert {len} strings to addresses", len = source.len())]
    ConvertStringsToAddressesFailed { source: ErrVec<ConvertStringToAddressError> },
}

pub fn write_json_line(writer: &mut impl Write, value: &impl Serialize) -> Result<(), WriteJsonLineError> {
    use WriteJsonLineError::*;
    handle!(serde_json::to_writer(&mut *writer, value), ToWriterFailed);
    handle!(writeln!(writer), WriteNewlineFailed);
    Ok(())
}

#[derive(Error, Debug)]
pub enum WriteJsonLineError {
    #[error("failed to write JSON to stdout")]
    ToWriterFailed { source: serde_json::Error },
    #[error("failed to write newline to stdout")]
    WriteNewlineFailed { source: std::io::Error },
}
