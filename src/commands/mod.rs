mod component_get;
mod create_user;
mod init;
mod run;
mod upload;

use clap::{Parser, Subcommand};

use crate::{commands::run::run, errors::TcliError};

use self::{component_get::component_get, create_user::create_user, init::init, upload::upload};

#[derive(Debug, Parser)]
#[clap(version)]
pub struct TcliArgs {
    #[clap(subcommand)]
    pub sub: EntityType,
}

#[derive(Debug, Subcommand)]
pub enum EntityType {
    /// Creates a basic cog in the current directory
    Init { directory: Option<String> },

    /// Install specified TCore components on your system
    ComponentGet { component: String },

    /// List all TCore components
    ComponentLs,

    /// Run specified TCore components on your system
    Run {
        component: String,
        args: Option<Vec<String>>,
    },

    /// Upload cogs to cogstore.tcore.io
    Upload,

    /// Create a new user which can be used to upload cogs to cogstore.tcore.io
    CreateUser,
}

impl EntityType {
    pub fn run(&self) -> Result<(), TcliError> {
        match self {
            EntityType::Init { directory } => return init(directory),
            EntityType::ComponentGet { component } => return component_get(component.as_str()),
            EntityType::ComponentLs => {
                println!("Avaliable components:\nruntime - A dev runtime for TCore cogs");
                return Ok(());
            }
            EntityType::Run { component, args } => return run(component, args),
            EntityType::Upload => return upload(),
            EntityType::CreateUser => return create_user(),
        }
    }
}
