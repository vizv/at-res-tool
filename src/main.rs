use anyhow::Result;
use clap::{Parser, Subcommand};

mod ktex;

/// The Away Team resource tool for dumping and repacking game resources
#[derive(Debug, Parser)]
struct Cli {
  // The resource type to operate on
  #[command(subcommand)]
  command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
  /// Commands for Klei texture files
  Ktex {
    #[command(subcommand)]
    command: KtexCommands,
  },
}

#[derive(Debug, Subcommand)]
enum KtexCommands {
  /// Dump a Klei texture file
  Dump { path: String },
}

#[tokio::main]
async fn main() -> Result<()> {
  let args = Cli::parse();

  match args.command {
    Commands::Ktex { command } => match command {
      KtexCommands::Dump { path } => todo!("Dump Klei texture file at path: {}", path),
    },
  }

  Ok(())
}
