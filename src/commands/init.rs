use std::{fs, path::PathBuf};

use crate::{
    errors::{warn, TcliError},
    init_files::{init_cogs, init_hooks},
};

pub fn init(dir: &Option<String>) -> Result<(), TcliError> {
    let path = match dir {
        // If a directory is specified, create a cog in that directory
        Some(dir) => check_dir(dir),
        // If no directory is specified, create a cog in the current directory
        None => {
            warn(
                "Directory not specified",
                Some(vec![
                    "Cog will be created in current directory if empty".into()
                ]),
            );
            check_dir(&String::from("."))
        }
    };

    match path {
        Ok(path) => {
            // create the cog config file
            // cog.config.yaml
            let name = match path.file_name() {
                Some(e) => e.to_str(),
                None => None,
            };

            let name = match name {
                Some(e) => e,
                None => return Err(TcliError::Other("Could not get directory name".into())),
            };

            let hooks_file = path.clone().join(format!("{name}.hooks"));
            let cogs_file = path.clone().join(format!("{name}.cog"));
            let src_dir = path.clone().join("src");

            let write_1 = fs::write(hooks_file, init_hooks(name));
            let write_2 = fs::write(cogs_file, init_cogs(name));
            let write_3 = fs::create_dir(src_dir);

            if write_1.is_err() {
                return Err(TcliError::CouldNotCreateFile(format!("{name}.hooks")));
            }

            if write_2.is_err() {
                return Err(TcliError::CouldNotCreateFile(format!("{name}.cog")));
            }

            if write_3.is_err() {
                return Err(TcliError::CouldNotCreateFile("src".into()));
            }

            // the good ending
            return Ok(());
        }
        Err(err) => return Err(err),
    }
}

fn check_dir(dir: &String) -> Result<PathBuf, TcliError> {
    let path = PathBuf::from(dir);

    if !path.exists() {
        let res = fs::create_dir(&path);
        if res.is_err() {
            return Err(TcliError::DirectoryNotWritable);
        }
    } else if !path.is_dir() {
        return Err(TcliError::DirectoryNotWritable);
    }

    // check if dir is empty
    match path.read_dir() {
        Ok(dir) => {
            if dir.count() != 0 {
                return Err(TcliError::DirectoryNotEmpty);
            }
        }
        Err(err) => return Err(TcliError::Other(err.to_string())),
    }

    // check if dir is writable
    match path.metadata() {
        Ok(metadata) => {
            if metadata.permissions().readonly() {
                return Err(TcliError::DirectoryNotWritable);
            }
        }
        Err(err) => {
            return Err(TcliError::Other(err.to_string()));
        }
    }

    // The good ending
    return Ok(path);
}
