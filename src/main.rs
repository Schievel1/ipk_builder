use eframe::{
        egui,
        run_native
    };
use ipkbuilder::ui::IpkBuilder;

fn main() {
    env_logger::init();
    let _app: IpkBuilder = Default::default();
    let options = eframe::NativeOptions {
        decorated: true,
        initial_window_size: Some(egui::vec2(500.0, 950.0)),
        resizable: true,
        ..Default::default()
    };
    let _ = run_native(
        "IPK Package Builder",
        options,
        Box::new(|cc| Box::new(IpkBuilder::new(cc))),
    );
}
