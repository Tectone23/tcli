use colored::*;
use thiserror::Error;
use std::process::exit;

#[deprecated]
pub fn throw(error: &str) {
    let f_error = error.red().bold();

    println!("{f_error}");

    exit(1);
}

pub fn error(error: &str) {
    let f_error = error.red().bold();

    println!("{f_error}");
}

pub fn success(message: &str) {
    let f_prefix = "SUCCESS".green().bold().reversed();
    let f_content = message.green().bold();

    println!("{f_prefix} {f_content}");
}

pub fn info(message: &str) {
    let f_msg = message.blue().bold();

    println!("{f_msg}");
}

pub fn warn(message: &str, extras: Option<Vec<String>>) {
    let f_prefix = "WARN".yellow().bold().reversed();
    let f_content = message.yellow().bold();
    println!("{f_prefix} {f_content}");

    if let Some(extras) = extras {
        for extra in extras {
            let f_extra = format!("  -> {extra}").yellow().bold();
            println!("{f_extra}");
        }
    }
}

#[derive(Error, Debug, PartialEq)]
pub enum TcliError {
    #[error("Directory is not empty")]
    DirectoryNotEmpty,
    #[error("Directory is not writable")]
    DirectoryNotWritable,
    #[error("Could not create file: {0}")]
    CouldNotCreateFile(String),
    #[error("Windows not supported")]
    WindowsNotSupported,
    #[error("Unknown platform detected")]
    UnknownPlatform,
    #[error("Request error {0}")]
    RequestError(String),
    #[error("User error: password is incorrect")]
    IncorrectPassword,
    #[error("Component does not exist")]
    ComponentDoesNotExist,
    #[error("Unknown error: {0}")]
    Other(String),
}


impl TcliError {
    pub fn print_err(&self) {
        let f_error = "ERROR".red().bold().reversed();
        let f_content = self.to_string().red().bold();
        println!("{f_error} {f_content}");
    }
}

#[macro_export]
macro_rules! throw {
    ($x: expr) => {
        $x.print_err();
        std::process::exit(1);
    };
}
