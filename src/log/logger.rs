use super::level::LogLevel;
use super::target::LogTarget;
use std::sync::Arc;
//NOTE: let's try an non-static logger
pub struct Logger {
    targets: Vec<Arc<dyn LogTarget>>,
}

impl Logger {
    pub fn new() -> Self {
        Self {
            targets: Vec::new(),
        }
    }

    pub fn add_target<T: LogTarget + 'static>(&mut self, target: T) {
        self.targets.push(Arc::new(target));
    }

    pub fn log(&self, level: LogLevel, msg: &str) {
        for t in &self.targets {
            if level <= t.level() {
                t.log(level, msg);
            }
        }
    }

    pub fn error(&self, msg: &str) {
        self.log(LogLevel::Error, msg);
    }
    pub fn warn(&self, msg: &str) {
        self.log(LogLevel::Warn, msg);
    }
    pub fn info(&self, msg: &str) {
        self.log(LogLevel::Info, msg);
    }
    pub fn debug(&self, msg: &str) {
        self.log(LogLevel::Debug, msg);
    }

    pub fn flush(&self) {
        for t in &self.targets {
            t.flush();
        }
    }
}
