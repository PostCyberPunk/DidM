use super::LogTarget;
use crate::log::level::LogLevel;

pub struct StdoutLogTarget {
    min_level: LogLevel,
}

impl StdoutLogTarget {
    pub fn new(min_level: LogLevel) -> Self {
        Self { min_level }
    }
}

impl LogTarget for StdoutLogTarget {
    fn level(&self) -> LogLevel {
        self.min_level
    }
    fn log(&self, level: LogLevel, msg: &str) {
        if level <= self.min_level {
            println!("[{}] {}", level.as_str(), msg);
        }
    }
}
