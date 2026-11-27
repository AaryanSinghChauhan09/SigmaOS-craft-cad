#![forbid(unsafe_code)]
use clap::{Parser, Subcommand};
#[derive(Parser)]
#[command(name = "sigmaos-craft-cad", about = "Craft CAD drafting and engineering studio for SigmaOS")]
struct Cli { #[command(subcommand)] cmd: Commands }
#[derive(Subcommand)]
enum Commands { List, Status }
fn main() {
    let cli = Cli::parse();
    match cli.cmd {
        Commands::List   => println!("sigmaos-craft-cad: no entries configured."),
        Commands::Status => println!("sigmaos-craft-cad: ready."),
    }
}
