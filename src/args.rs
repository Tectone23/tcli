use clap::{Args, Parser, Subcommand};

#[derive(Debug, Parser)]
#[clap(version, about)]
pub struct TcliArgs {
  #[clap(subcommand)]
  pub sub: EntityType,
}

#[derive(Debug, Subcommand)]
pub enum EntityType {
  /// Creates a basic cog in the current directory
  Init(Directory),

  /// Add a new action to the cog
  Add(Action),
}

#[derive(Debug, Args)]
pub struct Directory {
  /// Initialise in an different directory
  pub directory: Option<String>,
}
#[derive(Debug, Args)]
pub struct Action {
  /// Add a new action to the cog WIP
  pub action: Option<String>,
}
