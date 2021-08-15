use crate::logging::LogTarget;
use log::{Record, Level};
use chrono::Local;
use colored::*;


pub static TERMINAL_LOG_TARGET: TerminalLogTarget = TerminalLogTarget;

pub struct TerminalLogTarget;

impl LogTarget for TerminalLogTarget {
    fn log(&self, record: &Record) {
        let level_message;
        match record.level() {
            Level::Error => level_message = format!("{}", record.level()).red().bold(),
            Level::Warn => level_message = format!("{}", record.level()).yellow().bold(),
            Level::Info => level_message = format!("{}", record.level()).green().bold(),
            Level::Debug => level_message = format!("{}", record.level()).blue().bold(),
            Level::Trace => level_message = format!("{}", record.level()).white().bold(),
        }
        println!(
            "[{} {} {}] {}",
            Local::now().format("%F %H:%M:%S"),
            level_message,
            record.module_path().unwrap_or(""),
            record.args()
        );
    }
}
