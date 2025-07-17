// src/main.rs
/*
 * Main executable for IntelliSenseAdvisorSystems
 */

use clap::Parser;
use intellisenseadvisorsystems::{Result, run};

#[derive(Parser)]
#[command(version, about = "IntelliSenseAdvisorSystems - A Rust implementation")]
struct Cli {
    /// Enable verbose output
    #[arg(short, long)]
    verbose: bool,
}

fn main() -> Result<()> {
    let args = Cli::parse();
    run(args.verbose)
}
