use clap::{Parser, Subcommand};
use anyhow::Result;
use tracing::{info, Level};
use tracing_subscriber;

mod commands;
mod voicevox;
mod error;

use commands::say::SayCommand;

#[derive(Parser)]
#[command(
    name = "voxmix",
    about = "A CLI tool for high-quality speech synthesis using VOICEVOX engine",
    version
)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Generate speech from text using VOICEVOX engine
    Say(SayCommand),
}

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize tracing
    tracing_subscriber::fmt()
        .with_max_level(Level::INFO)
        .init();

    let cli = Cli::parse();

    match cli.command {
        Commands::Say(say_cmd) => {
            info!("Starting speech synthesis");
            say_cmd.execute().await
        }
    }
}