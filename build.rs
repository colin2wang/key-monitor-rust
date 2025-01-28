// build.rs
#![allow(unused)] // 可选：忽略未使用代码的警告

// 添加虚拟模块声明（仅用于消除 IDE 警告）
mod dummy {}

fn main() {
    #[cfg(windows)]
    {
        // Windows 资源编译逻辑
        use winres::WindowsResource;
        let mut res = WindowsResource::new();
        res.set_icon("resources/app_icon.ico");
        res.compile().unwrap();
    }
}