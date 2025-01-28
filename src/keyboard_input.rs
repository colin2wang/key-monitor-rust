use std::sync::mpsc;
use std::thread;
use std::time::Duration;
use winapi::um::winuser::{keybd_event, VkKeyScanA, MapVirtualKeyA, KEYEVENTF_KEYUP};

// 启动键盘输入线程并返回一个停止信号发送器
pub fn start_keyboard_input() -> mpsc::Sender<()> {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        loop {
            // 检查是否收到停止信号
            if let Ok(_) = rx.try_recv() {
                break;
            }

            // 按下并释放 Scroll Lock 键两次
            for _ in 0..2 {
                let vk_scroll_lock;
                unsafe {
                    vk_scroll_lock = VkKeyScanA((b'S' as i32).try_into().unwrap()) & 0xff;
                }
                let vk_scroll_lock_u32 = match vk_scroll_lock.try_into() {
                    Ok(val) => val,
                    Err(_) => {
                        eprintln!("Failed to convert vk_scroll_lock to u32");
                        continue;
                    }
                };
                let vk_scroll_lock_u8 = match vk_scroll_lock.try_into() {
                    Ok(val) => val,
                    Err(_) => {
                        eprintln!("Failed to convert vk_scroll_lock to u8");
                        continue;
                    }
                };
                let map_virtual_key_result_u8;
                unsafe {
                    map_virtual_key_result_u8 = match MapVirtualKeyA(vk_scroll_lock_u32, 0).try_into() {
                        Ok(val) => val,
                        Err(_) => {
                            eprintln!("Failed to convert MapVirtualKeyA result to u8");
                            continue;
                        }
                    };
                }
                unsafe {
                    keybd_event(vk_scroll_lock_u8, map_virtual_key_result_u8, 0, 0);
                    keybd_event(vk_scroll_lock_u8, map_virtual_key_result_u8, KEYEVENTF_KEYUP, 0);
                }
            }

            // 等待 2 分钟
            thread::sleep(Duration::from_secs(120));
        }
    });

    tx
}

// 停止键盘输入线程
pub fn stop_keyboard_input(sender: mpsc::Sender<()>) {
    let _ = sender.send(());
}