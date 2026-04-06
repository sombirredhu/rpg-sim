//! Game Logger - File-based logging system for debugging
//!
//! Usage:
//! ```rust
//! use logger::log_error;
//! log_error!("Something failed: {}", error);
//! ```
//!
//! Logs are written to `logs/game_YYYY-MM-DD_HH-MM-SS.log`

use std::fs::{self, File, OpenOptions};
use std::io::{BufWriter, Write};
use std::path::PathBuf;
use std::sync::Mutex;
use log::{Log, Metadata, Record, LevelFilter};
use chrono::Local;

/// File logger that writes logs to disk with timestamps
struct FileLogger {
    writer: Mutex<BufWriter<File>>,
    log_path: PathBuf,
}

impl FileLogger {
    /// Create a new file logger
    fn new(log_dir: &str, prefix: &str) -> Result<Self, std::io::Error> {
        // Create logs directory if it doesn't exist
        fs::create_dir_all(log_dir)?;

        // Generate timestamped filename
        let timestamp = Local::now().format("%Y-%m-%d_%H-%M-%S").to_string();
        let filename = format!("{}_{}.log", prefix, timestamp);
        let log_path = PathBuf::from(log_dir).join(filename);

        // Open file for appending
        let file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(&log_path)?;

        let writer = BufWriter::new(file);

        Ok(Self { writer: Mutex::new(writer), log_path })
    }

    /// Get the path to the log file
    fn log_path(&self) -> &PathBuf {
        &self.log_path
    }
}

impl Log for FileLogger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() <= log::Level::Info
    }

    fn log(&self, record: &Record) {
        if self.enabled(record.metadata()) {
            let timestamp = Local::now().format("%Y-%m-%d %H:%M:%S%.3f").to_string();
            let level = record.level();
            let target = record.target();
            let args = record.args();

            if let Ok(mut writer) = self.writer.lock() {
                let _ = writeln!(
                    writer,
                    "[{}] [{}] [{}] {}",
                    timestamp,
                    level,
                    target,
                    args
                );
                let _ = writer.flush();
            }
        }
    }

    fn flush(&self) {
        if let Ok(mut writer) = self.writer.lock() {
            let _ = writer.flush();
        }
    }
}

/// Initialize the logger and return the log file path
/// Call this at the very beginning of main()
pub fn init_logger() -> Result<PathBuf, std::io::Error> {
    // Check if logger is already initialized to avoid panic
    if log::max_level() != LevelFilter::Off {
        return Ok(PathBuf::from("logs/already_initialized.log"));
    }

    let logger = FileLogger::new("logs", "game")?;
    let log_path = logger.log_path().clone();

    // Set the global logger
    log::set_max_level(LevelFilter::Info);
    log::set_boxed_logger(Box::new(logger))
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;

    // Log startup message
    info!("=== Realm of Bounties - Game Logger initialized ===");
    info!("Log file: {}", log_path.display());

    Ok(log_path)
}

/// Convenience macros for logging
#[macro_export]
macro_rules! log_error {
    ($($arg:tt)*) => {{
        error!($($arg)*);
    }};
}

#[macro_export]
macro_rules! log_warn {
    ($($arg:tt)*) => {{
        warn!($($arg)*);
    }};
}

#[macro_export]
macro_rules! log_info {
    ($($arg:tt)*) => {{
        info!($($arg)*);
    }};
}

#[macro_export]
macro_rules! log_debug {
    ($($arg:tt)*) => {{
        debug!($($arg)*);
    }};
}

#[macro_export]
macro_rules! log_trace {
    ($($arg:tt)*) => {{
        trace!($($arg)*);
    }};
}
