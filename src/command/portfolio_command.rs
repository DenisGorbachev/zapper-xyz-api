use crate::{Address, ChainId, Client, PageSize, PageTurnerPortfolioV2TokenBalancesByTokenRequestClientError, PortfolioV2TokenBalancesByTokenRequest, WriteJsonLineError, write_json_line};
use clap::Parser;
use errgonomic::handle;
use futures::{TryStreamExt, pin_mut};
use page_turner::PageTurner;
use std::io::{BufWriter, Write, stdout};
use std::ops::Not;
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
        let chain_ids = chain_ids.is_empty().not().then_some(chain_ids);
        for address in addresses {
            let request = PortfolioV2TokenBalancesByTokenRequest::new(address.clone(), chain_ids.clone(), first);
            let pages = client.pages(request);
            pin_mut!(pages);
            while let Some(data) = handle!(pages.try_next().await, TryNextFailed) {
                let page = PortfolioAddressTokenBalancesPage {
                    address: &address,
                    data: &data,
                };
                handle!(write_json_line(&mut stdout, &page), WriteJsonLineFailed);
                handle!(stdout.flush(), FlushStdoutFailed);
            }
        }
        Ok(ExitCode::SUCCESS)
    }
}

#[derive(Error, Debug)]
pub enum PortfolioCommandRunError {
    #[error("failed to read the next portfolioV2 token balances by token page")]
    TryNextFailed { source: PageTurnerPortfolioV2TokenBalancesByTokenRequestClientError },
    #[error("failed to write portfolio address token page")]
    WriteJsonLineFailed { source: WriteJsonLineError },
    #[error("failed to flush stdout")]
    FlushStdoutFailed { source: std::io::Error },
}

mod portfolio_address_token_balances_page;
pub use portfolio_address_token_balances_page::*;
