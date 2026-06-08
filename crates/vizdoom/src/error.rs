//! Error and result types for the safe ViZDoom API.

use std::ffi::CStr;

use vizdoom_sys as sys;

/// Convenience alias for results returned by this crate.
pub type Result<T> = std::result::Result<T, Error>;

/// Errors surfaced from the ViZDoom C ABI.
#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// A handle or argument passed across FFI was invalid (e.g. a null pointer
    /// or a string containing an interior NUL byte).
    #[error("invalid argument: {0}")]
    InvalidArgument(String),

    /// The operation requires a running game but the game was not running.
    #[error("game is not running")]
    NotRunning,

    /// A config or resource file could not be found.
    #[error("file not found: {0}")]
    FileNotFound(String),

    /// A generic ViZDoom/engine error.
    #[error("vizdoom error: {0}")]
    Engine(String),
}

impl Error {
    /// Builds an [`Error`] from a non-OK [`sys::VzdStatus`], attaching the
    /// thread-local message recorded by the C ABI.
    pub(crate) fn from_status(status: sys::VzdStatus) -> Self {
        let message = last_error_message();
        match status {
            sys::VzdStatus::Ok => {
                Error::Engine("unexpected OK status converted to error".to_string())
            }
            sys::VzdStatus::InvalidArgument => Error::InvalidArgument(message),
            sys::VzdStatus::NotRunning => Error::NotRunning,
            sys::VzdStatus::FileNotFound => Error::FileNotFound(message),
            sys::VzdStatus::Error => Error::Engine(message),
        }
    }
}

/// Reads the thread-local error message from the C ABI.
fn last_error_message() -> String {
    // SAFETY: the C ABI guarantees a valid, NUL-terminated, thread-local
    // string that is never null.
    unsafe {
        let ptr = sys::vzd_last_error_message();
        if ptr.is_null() {
            String::new()
        } else {
            CStr::from_ptr(ptr).to_string_lossy().into_owned()
        }
    }
}

/// Converts a [`sys::VzdStatus`] into `Result<()>`.
pub(crate) fn check(status: sys::VzdStatus) -> Result<()> {
    match status {
        sys::VzdStatus::Ok => Ok(()),
        other => Err(Error::from_status(other)),
    }
}
