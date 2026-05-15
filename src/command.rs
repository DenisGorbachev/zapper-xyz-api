use crate::{Client, ClientNewError, Key, parse_key};
use clap::Parser;
use clap::Subcommand as ClapSubcommand;
use errgonomic::handle;
use std::process::ExitCode;
use thiserror::Error;

use Subcommand::*;

#[derive(Parser, Debug)]
#[command(author, version, about, propagate_version = true)]
pub struct Command {
    #[arg(long, env = "ZAPPER_API_KEY", hide_env_values = true, value_parser = parse_key)]
    pub key: Key,
    #[command(subcommand)]
    pub subcommand: Subcommand,
}

#[derive(ClapSubcommand, Clone, Debug)]
pub enum Subcommand {
    Portfolio(portfolio_command::PortfolioCommand),
}

impl Command {
    pub async fn run(self) -> Result<ExitCode, CommandRunError> {
        use CommandRunError::*;
        let Self {
            key,
            subcommand,
        } = self;
        let client = handle!(Client::new(key), ClientNewFailed);
        let exit_code = match subcommand {
            Portfolio(command) => handle!(command.run(client).await, PortfolioCommandRunFailed),
        };
        Ok(exit_code)
    }
}

#[derive(Error, Debug)]
pub enum CommandRunError {
    #[error("failed to construct Zapper client")]
    ClientNewFailed { source: ClientNewError },
    #[error("failed to run portfolio command")]
    PortfolioCommandRunFailed { source: PortfolioCommandRunError },
}

mod portfolio_command;

pub use portfolio_command::*;
