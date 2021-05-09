#[cfg(feature = "logger_slog")]
mod slog;

#[cfg(feature = "logger_slog")]
pub use self::slog::{Log, stdout};
