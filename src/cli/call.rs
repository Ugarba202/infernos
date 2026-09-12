use clap::Args;

#[derive(Args, Debug)]
pub struct CallArgs {
    /// Infernos node endpoint URL
    #[arg(short, long, default_value = "http://127.0.0.1:8080")]
    pub endpoint: String,

    /// Target model
    #[arg(short, long, default_value = "llama3")]
    pub model: String,

    /// Prompt text to send for inference
    #[arg(short, long)]
    pub prompt: String,

    /// Optional session budget limit in Satoshis
    #[arg(short, long)]
    pub budget: Option<u64>,
}
