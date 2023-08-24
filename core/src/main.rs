use clap::Parser;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[arg(short)]
    addr: String,
    #[arg(short)]
    promo: u32,
}

fn main() {
    let cli = Cli::parse();
    let (_addr, _promo) = (cli.addr, cli.promo);

    println!("{:?}", _promo);
}