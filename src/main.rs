mod api;
mod cipher;
mod commands;
mod component_utils;
mod config_utils;
mod errors;
mod files;
mod init_files;
mod user;

use clap::Parser;
use commands::TcliArgs;

fn main() {
    let arg = TcliArgs::parse().sub;
    match arg.run() {
        Ok(_) => {
        }
        Err(err) => {
            throw!(err);
        }
    }
}
