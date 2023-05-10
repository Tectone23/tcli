use crate::{component_utils::install_component, errors::TcliError};

pub fn component_get(component: &str) -> Result<(), TcliError> {
    return install_component(component);
}
