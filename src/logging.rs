// logging.rs
use log::{Level, LevelFilter, Record};
use std::sync::{Arc, Mutex};
use chrono::Local;
use std::thread;
use std::fmt::Arguments;

pub struct LogView {
    pub logs: Arc<Mutex<Vec<String>>>,
    pub log_level: LevelFilter, // 当前日志显示级别
}

impl LogView {
    pub fn new(logs: Arc<Mutex<Vec<String>>>) -> Self {
        Self {
            logs,
            log_level: LevelFilter::Info, // 默认显示 INFO 及以上级别的日志
        }
    }

    pub fn ui(&mut self, ui: &mut egui::Ui) {
        // 显示日志视图
        egui::ScrollArea::vertical().show(ui, |ui| {
            // 绘制白色背景
            let rect = ui.max_rect();
            ui.painter().rect_filled(rect, 0.0, egui::Color32::WHITE);

            let logs = self.logs.lock().unwrap();
            for log in logs.iter() {
                // 根据日志级别过滤显示
                let log_level = match log {
                    _ if log.contains("[ERROR]") => Level::Error,
                    _ if log.contains("[WARN]") => Level::Warn,
                    _ if log.contains("[INFO]") => Level::Info,
                    _ if log.contains("[DEBUG]") => Level::Debug,
                    _ if log.contains("[TRACE]") => Level::Trace,
                    _ => Level::Info,
                };

                if log_level <= self.log_level {
                    // 根据日志级别设置颜色
                    let color = match log_level {
                        Level::Error => egui::Color32::RED,
                        Level::Warn => egui::Color32::YELLOW,
                        Level::Info => egui::Color32::BLACK,
                        Level::Debug => egui::Color32::GRAY,
                        Level::Trace => egui::Color32::GRAY,
                    };

                    // 显示带颜色的日志
                    ui.colored_label(color, log);
                }
            }
        });
    }

    pub fn clear(&mut self) {
        let mut logs = self.logs.lock().unwrap();
        logs.clear();
    }
}

pub struct Logger {
    logs: Arc<Mutex<Vec<String>>>,
}

impl Logger {
    pub fn new(logs: Arc<Mutex<Vec<String>>>) -> Self {
        Self { logs }
    }
}

impl log::Log for Logger {
    fn enabled(&self, _metadata: &log::Metadata) -> bool {
        true
    }

    fn log(&self, record: &Record) {
        if self.enabled(record.metadata()) {
            // 解引用 record.args()
            let log = format_log(record.level(), *record.args());
            let mut logs = self.logs.lock().unwrap();
            logs.push(log);
        }
    }

    fn flush(&self) {}
}

// 辅助函数，用于生成格式化的日志消息
pub fn format_log(level: Level, args: Arguments) -> String {
    let now = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
    let current_thread = thread::current();
    let thread_name = current_thread.name().unwrap_or("<unnamed>").to_string();
    format!("[{}] [{}] [{}] {}", now, thread_name, level, args)
}