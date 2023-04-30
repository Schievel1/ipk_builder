use std::path::PathBuf;

use eframe::{
    egui::{self, RichText},
    epaint::{Color32, Vec2},
    run_native, NativeOptions,
};

pub struct FileOrPath {
    file_or_text: ScriptSource,
    from_textbox: String,
    picked_path: Option<String>,
}
impl Default for FileOrPath {
    fn default() -> Self {
        Self {
            file_or_text: ScriptSource::FromPath,
            from_textbox: "Package: example_package
Version: 1.3.3.7
Architecture: varam335x
Maintainer: user@domain.tld
Description: This is an example
Priority: optional
Depends: other_package"
                .to_owned(),
            picked_path: None,
        }
    }
}

#[derive(PartialEq, Default)]
enum ScriptSource {
    #[default]
    FromPath,
    FromTextfield,
}

#[derive(Default)]
struct IpkBuilderGui {
    control_file: FileOrPath,
}

impl IpkBuilderGui {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        Self::default()
    }
}
impl eframe::App for IpkBuilderGui {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            // The top panel is often a good place for a menu bar:
            egui::menu::bar(ui, |ui| {
                ui.menu_button("File", |ui| {
                    if ui.button("Quit").clicked() {
                        frame.close();
                    }
                });
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.group(|ui| {
                ui.vertical_centered_justified(|ui| {
                    ui.label("control file");
                    ui.horizontal(|ui| {
                        if ui
                            .add(egui::RadioButton::new(
                                self.control_file.file_or_text == ScriptSource::FromPath,
                                "from file",
                            ))
                            .clicked()
                        {
                            self.control_file.file_or_text = ScriptSource::FromPath;
                        }
                        if ui
                            .add(egui::RadioButton::new(
                                self.control_file.file_or_text == ScriptSource::FromTextfield,
                                "from input field",
                            ))
                            .clicked()
                        {
                            self.control_file.file_or_text = ScriptSource::FromTextfield;
                        }
                        if self.control_file.file_or_text == ScriptSource::FromPath {
                            if ui.button("Open file...").clicked() {
                                if let Some(path) = rfd::FileDialog::new().pick_file() {
                                    self.control_file.picked_path =
                                        Some(path.display().to_string());
                                }
                            }
                        } else {
                            ui.add_enabled(false, egui::Button::new("Open file..."));
                        };
                    });
                    if let Some(picked_path) = &self.control_file.picked_path {
                        ui.horizontal(|ui| {
                            ui.label("Picked file:");
                            ui.add(
                                egui::Label::new(RichText::new(picked_path).monospace()).wrap(true),
                            );
                        });
                    }
                    if self.control_file.file_or_text == ScriptSource::FromTextfield {
                        let response = ui.add(
                            egui::TextEdit::multiline(&mut self.control_file.from_textbox)
                                .code_editor(),
                        );
                    }
                });
            });
            ui.with_layout(egui::Layout::bottom_up(egui::Align::Center), |ui| {
                if self.control_file.picked_path.is_some()
                    || self.control_file.file_or_text == ScriptSource::FromTextfield
                {
                    if ui.add_sized([120., 40.],egui::Button::new("Build!").fill(Color32::RED)).clicked() {
                        todo!();
                    }
                } else {
                    ui.add_enabled(false, egui::Button::new("Build!").fill(Color32::DARK_GRAY).min_size(Vec2{ x: 120.,  y: 40.,}));
                };
            });
        });
    }
}

fn main() {
    let _app: IpkBuilderGui = Default::default();
    let options = eframe::NativeOptions {
        decorated: true,
        initial_window_size: Some(egui::vec2(500.0, 320.0)),
        resizable: true,
        ..Default::default()
    };
    run_native(
        "IPK Package Builder",
        options,
        Box::new(|cc| Box::new(IpkBuilderGui::new(cc))),
    );
}
