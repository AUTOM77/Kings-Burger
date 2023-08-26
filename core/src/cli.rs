use clap::{Parser, Subcommand};
use core::{Task, Token};

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

    let _token = Token::new();
    _token.unwrap().debug();

    match cli.args {
        Kings::Post { addr, promo } => Task::new(addr, promo).debug(),
    }
    
    Ok(())
}
