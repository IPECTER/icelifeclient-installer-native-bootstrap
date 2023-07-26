fn main() {
    // Include our program resources for visual styling and DPI awareness on windows
    if cfg!(windows) {
        let result = winres::WindowsResource::new()
            .set_icon("res/icon.ico")
            .set("ProductName", "IceLifeClient Installer")
            .set("CompanyName", "IceLife & IPECTER & Fabric Team")
            .set("FileDescription", "IceLifeClient Installer With Fabric")
            .set("LegalCopyright", "Copyright (C) 2023")
            .set_manifest_file("platform/windows/program.manifest")
            .compile();
        if let Err(_) = result {
            panic!("Failed to set windows resources");
        }
    }
}
