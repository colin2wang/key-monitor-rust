// main.rs
mod keyboard_input;
mod logging;

use egui::{ComboBox, Layout, Align};
use log::{LevelFilter, info, Level};
use std::sync::{mpsc, Arc, Mutex};
use eframe;
use logging::{LogView, Logger, format_log};
use keyboard_input::{start_keyboard_input, stop_keyboard_input};

fn main() {
    let logs = Arc::new(Mutex::new(Vec::new()));
    let logger = Logger::new(logs.clone());

    log::set_boxed_logger(Box::new(logger)).unwrap();
    log::set_max_level(LevelFilter::Trace); // 允许所有级别的日志

    info!("{}", format_log(Level::Info, format_args!("Application started.")));

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
    key_press_sender: Option<mpsc::Sender<()>>,
    is_key_press_running: bool,
}

impl MyApp {
    fn new(logs: Arc<Mutex<Vec<String>>>) -> Self {
        Self {
            log_view: LogView::new(logs),
            key_press_sender: None,
            is_key_press_running: false,
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
                    info!("{}", format_log(Level::Info, format_args!("Info button clicked.")));
                    log::info!("This is an info message");
                }
                if ui.button("Debug").clicked() {
                    info!("{}", format_log(Level::Info, format_args!("Debug button clicked.")));
                    log::debug!("This is a debug message");
                }
                if ui.button("Error").clicked() {
                    info!("{}", format_log(Level::Info, format_args!("Error button clicked.")));
                    log::error!("This is an error message");
                }
                if ui.button("Clear Logs").clicked() {
                    info!("{}", format_log(Level::Info, format_args!("Clear Logs button clicked.")));
                    self.log_view.clear();
                }

                if self.is_key_press_running {
                    if ui.button("Stop Key Press").clicked() {
                        info!("{}", format_log(Level::Info, format_args!("Stop Key Press button clicked.")));
                        if let Some(sender) = self.key_press_sender.take() {
                            stop_keyboard_input(sender);
                            self.is_key_press_running = false;
                        }
                    }
                } else {
                    if ui.button("Start Key Press").clicked() {
                        info!("{}", format_log(Level::Info, format_args!("Start Key Press button clicked.")));
                        let sender = start_keyboard_input();
                        self.key_press_sender = Some(sender);
                        self.is_key_press_running = true;
                    }
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