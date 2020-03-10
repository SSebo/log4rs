//! Rollers

use std::{error::Error, fmt, path::Path};

#[cfg(feature = "file")]
use crate::file::Deserializable;

#[cfg(feature = "delete_roller")]
pub mod delete;
#[cfg(feature = "fixed_window_roller")]
pub mod fixed_window;
#[cfg(feature = "time_based_roller")]
pub mod time_based;

/// A trait which processes log files after they have been rolled over.
pub trait Roll: fmt::Debug + Send + Sync + 'static {
    /// Processes the log file.
    ///
    /// At the time that this method has been called, the log file has already
    /// been closed.
    ///
    /// If this method returns successfully, there *must* no longer be a file
    /// at the specified location.
    fn roll(&self, file: &Path) -> Result<(), Box<dyn Error + Sync + Send>>;
}

#[cfg(feature = "file")]
impl Deserializable for dyn Roll {
    fn name() -> &'static str {
        "roller"
    }
}
