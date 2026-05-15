use clap::Parser;
use errgonomic::exit_result;
use std::process::ExitCode;
use zapper_xyz_api::Command;

#[tokio::main]
async fn main() -> ExitCode {
    let args = Command::parse();
    let result = args.run().await;
    exit_result(result)
}

#[test]
fn verify_cli() {
    use clap::CommandFactory;
    Command::command().debug_assert();
}
