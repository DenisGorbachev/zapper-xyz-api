use crate::{Address, ChainId, Client, ClientPortfolioV2TokenBalancesByTokenError, PageSize, PortfolioV2TokenBalancesByTokenRequest, PortfolioV2TokenBalancesByTokenRequestSetAfterStringError};
use clap::Parser;
use errgonomic::{handle, handle_opt};
use serde::Serialize;
use std::io::{BufWriter, Write, stdout};
use std::process::ExitCode;
use thiserror::Error;

#[derive(Parser, Clone, Debug)]
pub struct PortfolioCommand {
    #[arg(num_args = 1.., required = true)]
    pub addresses: Vec<Address>,
    #[arg(long = "chain-id")]
    pub chain_ids: Vec<ChainId>,
    #[arg(long, default_value_t = PageSize::default())]
    pub first: PageSize,
}

impl PortfolioCommand {
    pub async fn run(self, client: Client) -> Result<ExitCode, PortfolioCommandRunError> {
        use PortfolioCommandRunError::*;
        let Self {
            addresses,
            chain_ids,
            first,
        } = self;
        let mut stdout = BufWriter::new(stdout().lock());
        for address in addresses {
            let mut request = PortfolioV2TokenBalancesByTokenRequest::new(address, chain_ids.clone(), first);
            loop {
                let page_request = request.clone();
                let data = handle!(
                    client
                        .portfolio_v2_token_balances_by_token(page_request)
                        .await,
                    PortfolioV2TokenBalancesByTokenFailed
                );
                let by_token = data.portfolio_v2.token_balances.by_token;
                let page = PortfolioAddressTokenBalancesPage {
                    address: &request.address,
                    data: &by_token,
                };
                handle!(write_json_line(&mut stdout, &page), WriteJsonLineFailed);
                handle!(stdout.flush(), FlushStdoutFailed);
                let page_info = by_token.page_info;
                if page_info.has_next_page {
                    let after = handle_opt!(page_info.end_cursor, TokenPageEndCursorNotFound, request);
                    handle!(request.set_after_string(after), PortfolioV2TokenBalancesByTokenRequestSetAfterStringFailed);
                } else {
                    break;
                }
            }
        }
        Ok(ExitCode::SUCCESS)
    }
}

#[derive(Error, Debug)]
pub enum PortfolioCommandRunError {
    #[error("failed to query portfolioV2 token balances by token")]
    PortfolioV2TokenBalancesByTokenFailed { source: ClientPortfolioV2TokenBalancesByTokenError },
    #[error("failed to set portfolioV2 token page cursor")]
    PortfolioV2TokenBalancesByTokenRequestSetAfterStringFailed { source: PortfolioV2TokenBalancesByTokenRequestSetAfterStringError },
    #[error("failed to write portfolio address token page")]
    WriteJsonLineFailed { source: WriteJsonLineError },
    #[error("portfolioV2 token page info did not contain an end cursor")]
    TokenPageEndCursorNotFound { request: PortfolioV2TokenBalancesByTokenRequest },
    #[error("failed to flush stdout")]
    FlushStdoutFailed { source: std::io::Error },
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

mod portfolio_address_token_balances_page;

pub use portfolio_address_token_balances_page::*;
