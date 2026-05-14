use std::sync::mpsc;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::thread;
use std::time::Duration;
use winapi::um::winuser::{keybd_event, VK_NUMLOCK, KEYEVENTF_KEYUP};
use log::info;

/// Thread counter for generating unique thread names
static THREAD_COUNTER: AtomicUsize = AtomicUsize::new(0);

/// Key press interval in milliseconds
const KEY_PRESS_INTERVAL_MS: u64 = 500;

/// Loop wait time in seconds
const LOOP_WAIT_SECONDS: u64 = 120;

/// Number of presses per loop
const PRESSES_PER_LOOP: u32 = 2;

/// Simulate pressing the NumLock key
fn press_numlock() {
    unsafe {
        keybd_event(VK_NUMLOCK as u8, 0, 0, 0);
        keybd_event(VK_NUMLOCK as u8, 0, KEYEVENTF_KEYUP, 0);
    }
    thread::sleep(Duration::from_millis(KEY_PRESS_INTERVAL_MS));
}

/// Start keyboard input thread to periodically press NumLock key to prevent system sleep
pub fn start_keyboard_input() -> mpsc::Sender<()> {
    let (tx, rx) = mpsc::channel();

    // Generate unique thread ID
    let thread_id = THREAD_COUNTER.fetch_add(1, Ordering::SeqCst);
    let thread_name = format!("KeyboardInputThread-{}", thread_id);
    let thread_name_clone = thread_name.clone();

    thread::Builder::new()
        .name(thread_name_clone.clone())
        .spawn(move || {
            info!("Thread '{}' started", thread_name_clone);

            while rx.try_recv().is_err() {
                for _ in 0..PRESSES_PER_LOOP {
                    press_numlock();
                }
                info!("Completed {} NumLock key presses", PRESSES_PER_LOOP);
                thread::sleep(Duration::from_secs(LOOP_WAIT_SECONDS));
            }

            info!("Thread '{}' is stopping", thread_name_clone);
        })
        .expect("Failed to create keyboard input thread");

    info!("Keyboard input thread '{}' started", thread_name);

    tx
}

/// Stop keyboard input thread
pub fn stop_keyboard_input(sender: mpsc::Sender<()>) {
    info!("Sending stop signal to keyboard input thread");
    let _ = sender.send(());
}
