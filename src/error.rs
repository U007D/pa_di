pub mod initialization;
use crate::consts::msg;
use thiserror::Error;

pub type Result<T, E = Error> = std::result::Result<T, E>;

#[derive(Debug, Error, Eq, Hash, PartialEq)]
pub enum Error {
    #[error(transparent)]
    Initialization(#[from] initialization::Error),
}
