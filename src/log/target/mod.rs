mod file;
mod stdout;
use super::level::LogLevel;

pub use file::FileLogTarget;
pub use stdout::StdoutLogTarget;

pub trait LogTarget: Send + Sync {
    fn level(&self) -> LogLevel;
    fn log(&self, level: LogLevel, msg: &str);
    fn flush(&self) {}
}
