mod keyboard_input;

use egui::{ScrollArea, Ui, Color32, ComboBox, Layout, Align};
use log::{Level, LevelFilter, Record};
use std::sync::{Arc, Mutex};
use eframe;

struct LogView {
    logs: Arc<Mutex<Vec<String>>>,
    log_level: LevelFilter, // 当前日志显示级别
}

impl LogView {
    fn new(logs: Arc<Mutex<Vec<String>>>) -> Self {
        Self {
            logs,
            log_level: LevelFilter::Info, // 默认显示 INFO 及以上级别的日志
        }
    }

    fn ui(&mut self, ui: &mut Ui) {
        // 显示日志视图
        ScrollArea::vertical().show(ui, |ui| {
            // 绘制白色背景
            let rect = ui.max_rect();
            ui.painter().rect_filled(rect, 0.0, Color32::WHITE);

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
                        Level::Error => Color32::RED,
                        Level::Warn => Color32::YELLOW,
                        Level::Info => Color32::BLACK,
                        Level::Debug => Color32::GRAY,
                        Level::Trace => Color32::GRAY,
                    };

                    // 显示带颜色的日志
                    ui.colored_label(color, log);
                }
            }
        });
    }

    fn clear(&mut self) {
        let mut logs = self.logs.lock().unwrap();
        logs.clear();
    }
}

struct Logger {
    logs: Arc<Mutex<Vec<String>>>,
}

impl Logger {
    fn new(logs: Arc<Mutex<Vec<String>>>) -> Self {
        Self { logs }
    }
}

impl log::Log for Logger {
    fn enabled(&self, _metadata: &log::Metadata) -> bool {
        true
    }

    fn log(&self, record: &Record) {
        if self.enabled(record.metadata()) {
            // 添加日志级别标识
            let log = format!("[{}] {}", record.level(), record.args());
            let mut logs = self.logs.lock().unwrap();
            logs.push(log);
        }
    }

    fn flush(&self) {}
}

fn main() {
    let logs = Arc::new(Mutex::new(Vec::new()));
    let logger = Logger::new(logs.clone());

    log::set_boxed_logger(Box::new(logger)).unwrap();
    log::set_max_level(LevelFilter::Trace); // 允许所有级别的日志

    // 使用 eframe 创建窗口
    let options = eframe::NativeOptions::default();
    let _ = eframe::run_native(
        "Log Viewer",
        options,
        Box::new(|_cc| Box::new(MyApp::new(logs))),
    );
}

struct MyApp {
    log_view: LogView,
}

impl MyApp {
    fn new(logs: Arc<Mutex<Vec<String>>>) -> Self {
        Self {
            log_view: LogView::new(logs),
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            // 将按钮和日志级别下拉框放在同一行
            ui.horizontal(|ui| {
                // 左侧放置按钮
                if ui.button("Info").clicked() {
                    log::info!("This is an info message");
                }
                if ui.button("Debug").clicked() {
                    log::debug!("This is a debug message");
                }
                if ui.button("Error").clicked() {
                    log::error!("This is an error message");
                }
                if ui.button("Clear Logs").clicked() {
                    self.log_view.clear();
                }

                // 右侧放置日志级别下拉框
                ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                    ui.label("Log Level:");
                    ComboBox::from_id_source("log_level")
                        .selected_text(format!("{:?}", self.log_view.log_level))
                        .show_ui(ui, |ui| {
                            ui.selectable_value(&mut self.log_view.log_level, LevelFilter::Error, "Error");
                            ui.selectable_value(&mut self.log_view.log_level, LevelFilter::Warn, "Warn");
                            ui.selectable_value(&mut self.log_view.log_level, LevelFilter::Info, "Info");
                            ui.selectable_value(&mut self.log_view.log_level, LevelFilter::Debug, "Debug");
                            ui.selectable_value(&mut self.log_view.log_level, LevelFilter::Trace, "Trace");
                        });
                });
            });

            // 添加一个分隔线
            ui.separator();

            // 显示日志视图
            self.log_view.ui(ui);
        });
    }
}