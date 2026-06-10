fn main() {
    #[cfg(target_os = "windows")]
    {
        let mut res = winres::WindowsResource::new();
        res.set_icon("resources/icons/icon.ico");
        res.compile().unwrap();
    }
}
