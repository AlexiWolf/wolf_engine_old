use crate::logging::LogTarget;
use chrono::Local;
use colored::*;
use log::{Level, Record};

/// The global [TerminalLogTarget] instance.
pub(crate) static TERMINAL_LOG_TARGET: TerminalLogTarget = TerminalLogTarget;

/// Logs messages to the terminal.
pub struct TerminalLogTarget;

impl LogTarget for TerminalLogTarget {
    fn log(&self, record: &Record) {
        let level_message = Self::colored_level_message(record);
        Self::print_log_message(level_message, record);
    }
}

impl TerminalLogTarget {
    fn colored_level_message(record: &Record) -> ColoredString {
        match record.level() {
            Level::Error => format!("{}", record.level()).red().bold(),
            Level::Warn => format!("{}", record.level()).yellow().bold(),
            Level::Info => format!("{}", record.level()).green().bold(),
            Level::Debug => format!("{}", record.level()).blue().bold(),
            Level::Trace => format!("{}", record.level()).white().bold(),
        }
    }

    fn print_log_message(level_message: ColoredString, record: &Record) {
        println!(
            "[{} {} {}] {}",
            Local::now().format("%F %H:%M:%S"),
            level_message,
            record.module_path().unwrap_or(""),
            record.args()
        );
    }
}
