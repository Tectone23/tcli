use clap::{Args, Parser, Subcommand};

#[derive(Debug, Parser)]
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

    /// Install specified TCore components on your system
    ComponentGet(Components),

    /// List all TCore components
    ComponentLs,

    /// Install specified TCore components on your system
    Run(Components),

    /// Upload cogs to midgard
    Upload,

    /// Set the current user for midgard
    User(UserOptions),
}

#[derive(Debug, Args)]
pub struct UserOptions {
    pub username: String,
    pub password: String
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

#[derive(Debug, Args)]
pub struct Components {
    pub component: Vec<String>,
}

#[derive(Debug, Subcommand)]
pub enum ComponentList {
    Runtime,
}
