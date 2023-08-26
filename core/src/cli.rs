use clap::{Parser, Subcommand};
use core::{Task, Token};
use std::fs;
use toml;

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

    let token: Token = {
        let config_text = fs::read_to_string("./data/config.toml").expect("Token: error reading file");
        toml::from_str(&config_text).expect("Token: error reading stream")
    };

    token.debug();

    match cli.args {
        Kings::Post { addr, promo } => Task::new(addr, promo).debug(),
    }
    
    Ok(())
}
