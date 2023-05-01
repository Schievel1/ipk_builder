use eframe::{
        egui::{self, RichText},
        epaint::{Color32, Vec2},
        run_native, NativeOptions,
    };
use ipkbuilder::ui::IpkBuilder;

fn main() {
    let _app: IpkBuilder = Default::default();
    let options = eframe::NativeOptions {
        decorated: true,
        initial_window_size: Some(egui::vec2(500.0, 800.0)),
        resizable: true,
        ..Default::default()
    };
    run_native(
        "IPK Package Builder",
        options,
        Box::new(|cc| Box::new(IpkBuilder::new(cc))),
    );
}
