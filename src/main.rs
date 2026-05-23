use anyhow::Result;
use clap::{Parser, Subcommand};
use flexi_logger::{LevelFilter, LogSpecBuilder, Logger};

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

const MOD_NAME: &str = "at_res_tool";

#[tokio::main]
async fn main() -> Result<()> {
  let args = Cli::parse();

  let log_spec = LogSpecBuilder::new().default(LevelFilter::Info).module(MOD_NAME, LevelFilter::Debug).build();
  Logger::with(log_spec).log_to_stdout().start().unwrap();

  match args.command {
    Commands::Ktex { command } => match command {
      KtexCommands::Dump { path } => ktex::dump(path)?,
    },
  }

  Ok(())
}
