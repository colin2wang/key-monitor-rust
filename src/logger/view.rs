use log::{Level, LevelFilter};
use std::sync::{Arc, Mutex};

/// Log view component for displaying logs in the UI
pub struct LogView {
    pub logs: Arc<Mutex<Vec<String>>>,
    pub log_level: LevelFilter,
}

impl LogView {
    /// Create a new log view
    pub fn new(logs: Arc<Mutex<Vec<String>>>) -> Self {
        Self {
            logs,
            log_level: LevelFilter::Info, // Default to INFO and above
        }
    }

    /// Render the log view UI
    pub fn ui(&mut self, ui: &mut egui::Ui) {
        egui::ScrollArea::vertical()
            .max_height(f32::INFINITY)
            .max_width(ui.available_width())
            .auto_shrink([false, false])
            .stick_to_bottom(true) // Auto-scroll to bottom to show latest logs
            .show(ui, |ui| {
                if let Ok(logs) = self.logs.lock() {
                    for log in logs.iter() {
                        if let Some((log_level, color)) = self.parse_log_entry(log) {
                            if log_level <= self.log_level {
                                ui.colored_label(color, log);
                            }
                        }
                    }
                }
            });
    }

    /// Parse a log entry and return the log level with corresponding color
    fn parse_log_entry(&self, log: &str) -> Option<(Level, egui::Color32)> {
        let (level, color) = if log.contains("[ERROR]") {
            (Level::Error, egui::Color32::RED)
        } else if log.contains("[WARN]") {
            (Level::Warn, egui::Color32::YELLOW)
        } else if log.contains("[INFO]") {
            (Level::Info, egui::Color32::BLACK)
        } else if log.contains("[DEBUG]") {
            (Level::Debug, egui::Color32::GRAY)
        } else if log.contains("[TRACE]") {
            (Level::Trace, egui::Color32::GRAY)
        } else {
            return None;
        };
        
        Some((level, color))
    }

    /// Clear all logs
    pub fn clear(&mut self) {
        if let Ok(mut logs) = self.logs.lock() {
            logs.clear();
        }
    }
}

// 为 LogView 实现 Clone 特性
impl Clone for LogView {
    fn clone(&self) -> Self {
        Self {
            logs: self.logs.clone(),
            log_level: self.log_level,
        }
    }
}
