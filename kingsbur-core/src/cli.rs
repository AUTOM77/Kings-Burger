use clap::{Parser, Subcommand};
use std::env;

#[derive(Parser)]
#[command(author, version)]
struct Cli {
    #[command(subcommand)]
    args: Kings
}

#[derive(Subcommand)]
enum Kings {
    Post {
        addr: String,
        // key: String,
        #[clap(default_value="")]
        value: Vec<String>,
    }
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();
    let key = env::var("CHACHA_KEY").unwrap();
    let _ = match cli.args {
        Kings::Post { addr, value } => core::cat(addr, key, value).await,
    };
}
