use clap::Parser;
use infernos::cli::{Cli, Commands};
use infernos::common::error::Result;

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();

    let cli = Cli::parse();
    match cli.command {
        Commands::Node(args) => {
            tracing::info!("Executing node command: {:?}", args);
        }
        Commands::Call(args) => {
            tracing::info!("Executing call command for model: {}", args.model);
        }
    }

    Ok(())
}
