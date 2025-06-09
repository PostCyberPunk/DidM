use std::fs::{File, OpenOptions};
use std::io::{self, Write};
use std::sync::Mutex;

use super::LogTarget;
use crate::log::level::LogLevel;

pub struct FileLogTarget {
    min_level: LogLevel,
    buffer: Mutex<Vec<String>>,
    path: String,
}

impl FileLogTarget {
    pub fn new(path: &str, min_level: LogLevel) -> io::Result<Self> {
        Ok(Self {
            min_level,
            buffer: Mutex::new(Vec::new()),
            path: path.to_string(),
        })
    }

    pub fn flush(&self) {
        let mut buffer = self.buffer.lock().unwrap();
        if buffer.is_empty() {
            return;
        }
        let mut file = OpenOptions::new()
            .append(true)
            .create(true)
            .open(&self.path)
            .unwrap();
        for entry in buffer.iter() {
            let _ = writeln!(file, "{entry}");
        }
        buffer.clear();
    }
}

impl LogTarget for FileLogTarget {
    fn level(&self) -> LogLevel {
        self.min_level
    }
    fn log(&self, level: LogLevel, msg: &str) {
        if level <= self.min_level {
            let line = format!("[{}] {}", level.as_str(), msg);
            let mut buffer = self.buffer.lock().unwrap();
            buffer.push(line);
        }
    }
    fn flush(&self) {
        self.flush();
    }
}
