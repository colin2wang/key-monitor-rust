use log::{Level, LevelFilter};
use std::sync::{Arc, Mutex};
use env_logger::{Builder, fmt::Formatter};
use std::io::Write;
use egui;
use chrono;

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
        egui::ScrollArea::vertical()
            .max_height(f32::INFINITY) // 设置最大高度为无穷大，让滚动区域可以填充可用空间
            .max_width(ui.available_width()) // 设置最大宽度为可用宽度
            .auto_shrink([false, false]) // 禁止自动缩小
            .show(ui, |ui| {
                if let Ok(logs) = self.logs.lock() {
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
                }
            });
    }

    pub fn clear(&mut self) {
        if let Ok(mut logs) = self.logs.lock() {
            logs.clear();
        }
    }
}

// 自定义日志收集器
struct LogCollector(Arc<Mutex<Vec<String>>>);

impl Write for LogCollector {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        let log = String::from_utf8_lossy(buf).to_string();
        if let Ok(mut logs) = self.0.lock() {
            logs.push(log);
        }
        Ok(buf.len())
    }

    fn flush(&mut self) -> std::io::Result<()> {
        Ok(())
    }
}

// 配置 env_logger 的函数
pub fn setup_logger(logs: Arc<Mutex<Vec<String>>>) {
    let mut builder = Builder::new();
    builder.format(|buf: &mut Formatter, record| {
        let now = chrono::Local::now().format("%Y-%m-%d %H:%M:%S");
        let current_thread = std::thread::current();
        let thread_name = current_thread.name().unwrap_or("<unnamed>");
        writeln!(buf, "[{}] [{}] [{}] {}", now, thread_name, record.level(), record.args())
    });
    builder.filter(None, LevelFilter::Trace); // 允许所有级别的日志
    builder.target(env_logger::Target::Pipe(Box::new(LogCollector(logs))));
    builder.init();
}