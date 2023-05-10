use crate::{errors::TcliError, user::add_user};

pub fn create_user() -> Result<(), TcliError> {
    add_user();
    return Ok(());
}
