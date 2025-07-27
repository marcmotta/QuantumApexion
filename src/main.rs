// src/main.rs
/*
 * Main executable for QuantumApexion
 */

use clap::Parser;
use quantumapexion::{Result, run};

#[derive(Parser)]
#[command(version, about = "QuantumApexion - A Rust implementation")]
struct Cli {
    /// Enable verbose output
    #[arg(short, long)]
    verbose: bool,
}

fn main() -> Result<()> {
    let args = Cli::parse();
    run(args.verbose)
}
