use std::sync::mpsc;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::thread;
use std::time::{Duration};
use winapi::um::winuser::{keybd_event, VK_SCROLL, KEYEVENTF_KEYUP};
use log::{info, Level};
use crate::logging::format_log;

// 静态原子计数器，用于生成唯一的线程名
static THREAD_COUNTER: AtomicUsize = AtomicUsize::new(0);

fn press_scroll_lock() {
    unsafe {
        // 使用 VK_SCROLL 作为 Scroll Lock 键的虚拟键码
        keybd_event(VK_SCROLL as u8, 0, 0, 0); // 按下 Scroll Lock 键
        keybd_event(VK_SCROLL as u8, 0, KEYEVENTF_KEYUP, 0); // 释放 Scroll Lock 键
    }
    thread::sleep(Duration::from_millis(500)); // 暂停 0.5 秒
}

pub fn start_keyboard_input() -> mpsc::Sender<()> {
    let (tx, rx) = mpsc::channel();

    // 使用原子计数器生成唯一的线程名
    let thread_id = THREAD_COUNTER.fetch_add(1, Ordering::SeqCst);
    let thread_name = format!("KeyboardInputThread-{}", thread_id);

    // 克隆 thread_name 用于闭包
    let thread_name_clone = thread_name.clone();

    thread::Builder::new()
        .name(thread_name_clone.clone()) // 设置线程名
        .spawn(move || {
            info!("{}", format_log(Level::Info, format_args!("Thread '{}' started.", thread_name_clone)));

            while rx.try_recv().is_err() {
                for _ in 0..2 {
                    press_scroll_lock();
                }
                info!("{}", format_log(Level::Info, format_args!("Performed Scroll Lock key press and release twice.")));
                thread::sleep(Duration::from_secs(120)); // 等待 2 分钟
            }

            info!("{}", format_log(Level::Info, format_args!("Thread '{}' is stopping.", thread_name_clone)));
        })
        .unwrap();

    info!("{}", format_log(Level::Info, format_args!("Keyboard input thread '{}' has started.", thread_name)));

    tx
}

pub fn stop_keyboard_input(sender: mpsc::Sender<()>) {
    info!("{}", format_log(Level::Info, format_args!("Sending stop signal to keyboard input thread.")));
    let _ = sender.send(());
}