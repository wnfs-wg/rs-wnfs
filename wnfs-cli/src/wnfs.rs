use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about)]
struct Cli {}

fn main() {
    let _cli = Cli::parse();
}
