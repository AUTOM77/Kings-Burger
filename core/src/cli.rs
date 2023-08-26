use clap::{Parser, Subcommand};
use core::Task;

#[derive(Parser)]
#[command(author, version)]
struct Cli {
    #[command(subcommand)]
    args: Kings,
}

#[derive(Subcommand)]
enum Kings {
    Post {
        addr: String,
        promo: String,
    },
}

fn main() -> std::io::Result<()>{
    let cli = Cli::parse();
    match cli.args {
        Kings::Post { addr, promo } => Task::new(addr, promo).run(),
    }
    Ok(())
}
