use crate::logging::LogTarget;
use log::Record;

pub static TERMINAL_LOG_TARGET: TerminalLogTarget = TerminalLogTarget;

pub struct TerminalLogTarget;

impl LogTarget for TerminalLogTarget {
    fn log(&self, record: &Record) {
        println!(
            "{} {}: {}",
            record.level(),
            record.module_path().unwrap_or("wolf_engine"),
            record.args()
        );
    }
}
