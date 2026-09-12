use clap::{Args, Subcommand};

#[derive(Args, Debug)]
pub struct NodeArgs {
    #[command(subcommand)]
    pub action: NodeAction,
}

#[derive(Subcommand, Debug)]
pub enum NodeAction {
    /// Start the Infernos operator node
    Start {
        #[arg(short, long, default_value = "config/node.toml")]
        config: String,
    },
    /// Check operator status and balance
    Status,
    /// Stop running operator node
    Stop,
}
