#![windows_subsystem = "windows"]

mod logger;
mod keyboard_controller;

use egui::{ComboBox, Layout, Align, RichText, Window};
use log::{LevelFilter, info};
use std::sync::{mpsc, Arc, Mutex};
use eframe;
use logger::{LogView, setup_logger};
use keyboard_controller::{start_keyboard_input, stop_keyboard_input};

fn main() {
    let logs = Arc::new(Mutex::new(Vec::new()));

    // Initialize logging system
    setup_logger(logs.clone());

    info!("Application started");

    // Get version number
    let version = env!("CARGO_PKG_VERSION");
    let window_title = format!("Log Viewer v{}", version);

    // Create and run application window
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
            // Top control bar: buttons and log level selector
            ui.horizontal(|ui| {
                let btn_width = 100.0;
                let btn_height = ui.spacing().interact_size.y;

                if ui.add_sized([btn_width, btn_height], egui::Button::new("Clear Logs").rounding(4.0)).clicked() {
                    info!("Clear Logs button clicked");
                    self.log_view.clear();
                }

                if self.is_key_press_running {
                    let btn = egui::Button::new(
                        RichText::new("Stop Key Press").color(egui::Color32::from_rgb(200, 50, 50))
                    ).rounding(4.0);
                    if ui.add_sized([btn_width, btn_height], btn).clicked() {
                        info!("Stop Key Press button clicked");
                        if let Some(sender) = self.key_press_sender.take() {
                            stop_keyboard_input(sender);
                            self.is_key_press_running = false;
                        }
                    }
                } else {
                    let btn = egui::Button::new(
                        RichText::new("Start Key Press").color(egui::Color32::from_rgb(50, 160, 50))
                    ).rounding(4.0);
                    if ui.add_sized([btn_width, btn_height], btn).clicked() {
                        info!("Start Key Press button clicked");
                        let sender = start_keyboard_input();
                        self.key_press_sender = Some(sender);
                        self.is_key_press_running = true;
                    }
                }

                // Right side: log level selector
                ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
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
                    ui.label("Log Level:");
                });
            });

            ui.separator();

            // Display log view
            self.log_view.ui(ui);
        });

        // Trace level warning dialog
        if self.show_trace_warning {
            Window::new("Warning")
                .collapsible(false)
                .resizable(false)
                .show(ctx, |ui| {
                    ui.label("Switching to Trace level will generate a large number of logs. Are you sure?");
                    let btn_width = 100.0;
                    if ui.add_sized([btn_width, 0.0], egui::Button::new("Yes")).clicked() {
                        self.log_view.log_level = LevelFilter::Trace;
                        self.show_trace_warning = false;
                    }
                    if ui.add_sized([btn_width, 0.0], egui::Button::new("No")).clicked() {
                        self.log_view.log_level = self.previous_log_level;
                        self.show_trace_warning = false;
                    }
                });
        }
    }
}