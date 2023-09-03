use clap::{Parser, Subcommand};

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
        key: String,
        #[clap(default_value="")]
        value: String,
    }
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();
    let _ = match cli.args {
        Kings::Post { addr, key, value } => core::run(addr, key, value).await,
    };
}
