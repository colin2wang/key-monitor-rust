fn main() {
    if std::env::var("TARGET").unwrap().contains("windows") {
        let mut res = winres::WindowsResource::new();
        res.set_icon("resources/app_icon.ico");
        res.compile().unwrap();
    }
}
