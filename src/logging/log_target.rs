use log::Record;

/// Allows easy integration with the [Logger].
pub trait LogTarget: Send + Sync {
    /// Process / display the log message.
    fn log(&self, record: &Record);
}
