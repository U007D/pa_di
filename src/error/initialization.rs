use crate::consts::msg;
use thiserror::Error;

pub type Result<T, E = Error> = std::result::Result<T, E>;

#[derive(Clone, Debug, Eq, Error, Hash, PartialEq)]
pub enum Error {
    #[error("{}: {0}", msg::ERR_LOGGER_INITIALIZATION)]
    LoggerInitialization(String),
}