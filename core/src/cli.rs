use clap::{Parser, Subcommand};
use std::time::Instant;

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
    let now = Instant::now();

    let _ = match cli.args {
        Kings::Post { addr, key, value } => core::run(addr, key, value).await,
    };

    print!("runtime={:.2}_secs ", now.elapsed().as_secs_f64());
}
