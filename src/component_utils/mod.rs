pub mod component;

use crate::{errors::TcliError, files::AppFiles};
use std::str::FromStr;

use self::component::TComponent;

pub fn install_component(component_name: &str) -> Result<(), TcliError> {
    if cfg!(windows) {
        return Err(TcliError::WindowsNotSupported);
    } else if cfg!(unix) {
        return install_runtime_unix(component_name);
    } else {
        return Err(TcliError::UnknownPlatform);
    }
}

// TODO
fn install_runtime_unix(component_name: &str) -> Result<(), TcliError> {
    // make the required files/directories
    let files = AppFiles::new();
    files.check_and_generate();

    let _tre = TComponent::get_new(String::from_str(component_name).unwrap());

    return Ok(());
}
