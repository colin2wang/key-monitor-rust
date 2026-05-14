use std::sync::{Arc, Mutex};
use std::io::Write;
use chrono;

/// Log collector that captures logs and stores them in a shared Vec
pub struct LogCollector {
    logs: Arc<Mutex<Vec<String>>>,
}

impl LogCollector {
    /// Create a new log collector
    pub fn new(logs: Arc<Mutex<Vec<String>>>) -> Self {
        Self { logs }
    }
}

impl Write for LogCollector {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        let log = String::from_utf8_lossy(buf).to_string();
        if let Ok(mut logs) = self.logs.lock() {
            // Append new log to the end of the list
            logs.push(log);
        }
        Ok(buf.len())
    }

    fn flush(&mut self) -> std::io::Result<()> {
        Ok(())
    }
}

/// Configure and initialize the logging system
pub fn setup_logger(logs: Arc<Mutex<Vec<String>>>) {
    use env_logger::{Builder, fmt::Formatter};
    use log::LevelFilter;

    let mut builder = Builder::new();
    
    // Set log format: timestamp [thread_name] [level] message
    builder.format(|buf: &mut Formatter, record| {
        let now = chrono::Local::now().format("%Y-%m-%d %H:%M:%S");
        let current_thread = std::thread::current();
        let thread_name = current_thread.name().unwrap_or("<unnamed>");
        write!(buf, "[{}] [{}] [{}] {}", now, thread_name, record.level(), record.args())
    });
    
    // Allow all log levels through the filter
    builder.filter(None, LevelFilter::Trace);
    
    // Use custom log collector as output target
    builder.target(env_logger::Target::Pipe(Box::new(LogCollector::new(logs))));
    
    builder.init();
}
