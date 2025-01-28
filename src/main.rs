#![windows_subsystem = "windows"]
// main.rs
mod keyboard_input;
mod logging;

use egui::{ComboBox, Layout, Align, Window};
use log::{LevelFilter, info};
use std::sync::{mpsc, Arc, Mutex};
use eframe;
use logging::{LogView, setup_logger};
use keyboard_input::{start_keyboard_input, stop_keyboard_input};

fn main() {
    let logs = Arc::new(Mutex::new(Vec::new()));

    // 调用 logging.rs 中的 setup_logger 函数进行日志配置
    setup_logger(logs.clone());

    info!("Application started.");

    // 获取 Cargo.toml 中的版本号
    let version = env!("CARGO_PKG_VERSION");
    // 构建包含版本号的窗口标题
    let window_title = format!("Log Viewer v{}", version);

    // 使用 eframe 创建窗口
    let options = eframe::NativeOptions::default();
    let _ = eframe::run_native(
        &window_title,
        options,
        Box::new(|_cc| Ok(Box::new(MyApp::new(logs))))
    );
}

struct MyApp {
    log_view: LogView,
    key_press_sender: Option<mpsc::Sender<()>>,
    is_key_press_running: bool,
    show_trace_warning: bool,
    previous_log_level: LevelFilter,
}

impl MyApp {
    fn new(logs: Arc<Mutex<Vec<String>>>) -> Self {
        let log_view = LogView::new(logs);
        Self {
            log_view: log_view.clone(),
            key_press_sender: None,
            is_key_press_running: false,
            show_trace_warning: false,
            previous_log_level: log_view.log_level,
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
                    info!("Info button clicked.");
                }
                if ui.button("Debug").clicked() {
                    log::debug!("This is a debug message");
                }
                if ui.button("Error").clicked() {
                    log::error!("This is an error message");
                }
                if ui.button("Clear Logs").clicked() {
                    info!("Clear Logs button clicked.");
                    self.log_view.clear();
                }

                if self.is_key_press_running {
                    if ui.button("Stop Key Press").clicked() {
                        info!("Stop Key Press button clicked.");
                        if let Some(sender) = self.key_press_sender.take() {
                            stop_keyboard_input(sender);
                            self.is_key_press_running = false;
                        }
                    }
                } else {
                    if ui.button("Start Key Press").clicked() {
                        info!("Start Key Press button clicked.");
                        let sender = start_keyboard_input();
                        self.key_press_sender = Some(sender);
                        self.is_key_press_running = true;
                    }
                }

                // 右侧放置日志级别下拉框
                ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                    ui.label("Log Level:");
                    ComboBox::from_id_salt("log_level")
                        .selected_text(format!("{:?}", self.log_view.log_level))
                        .show_ui(ui, |ui| {
                            let mut new_log_level = self.log_view.log_level;
                            for level in [LevelFilter::Error, LevelFilter::Warn, LevelFilter::Info, LevelFilter::Debug, LevelFilter::Trace] {
                                let response = ui.selectable_value(&mut new_log_level, level, format!("{:?}", level));
                                if response.clicked() {
                                    if level == LevelFilter::Trace {
                                        self.previous_log_level = self.log_view.log_level;
                                        self.show_trace_warning = true;
                                    } else {
                                        self.log_view.log_level = level;
                                    }
                                }
                            }
                        });
                });
            });

            // 添加一个分隔线
            ui.separator();

            // 显示日志视图
            self.log_view.ui(ui);
        });

        if self.show_trace_warning {
            Window::new("Warning")
                .collapsible(false)
                .resizable(false)
                .show(ctx, |ui| {
                    ui.label("Switching to Trace level will generate a large number of logs. Are you sure?");
                    if ui.button("Yes").clicked() {
                        self.log_view.log_level = LevelFilter::Trace;
                        self.show_trace_warning = false;
                    }
                    if ui.button("No").clicked() {
                        self.log_view.log_level = self.previous_log_level;
                        self.show_trace_warning = false;
                    }
                });
        }
    }
}