use anyhow::anyhow;
use anyhow::{Result, Error};
use eframe::{
    egui::{self, RichText},
    epaint::{Color32, Vec2},
};
use std::{path::PathBuf, io};

use crate::make_package;

pub struct FileOrPath {
    pub enabled: bool,
    pub file_or_text: ScriptSource,
    pub from_textbox: String,
    pub picked_path: Option<PathBuf>,
}
impl Default for FileOrPath {
    fn default() -> Self {
        Self {
            enabled: false,
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
pub enum ScriptSource {
    #[default]
    FromPath,
    FromTextfield,
}

#[derive(Default)]
pub struct IpkBuilder {
    pub control_file: FileOrPath,
    pub debian_binary: FileOrPath,
    pub postinst: FileOrPath,
    pub preinst: FileOrPath,
    pub prerm: FileOrPath,
    pub data_path: Option<String>,
    pub output_path: Option<String>,
}

impl IpkBuilder {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        Self {
            debian_binary: FileOrPath {
                enabled: true,
                from_textbox: "2.0".to_owned(),
                file_or_text: ScriptSource::FromTextfield,
                ..Default::default()
            },
            postinst: FileOrPath {
                from_textbox: "#!/bin/bash\n".to_owned(),
                ..Default::default()
            },
            preinst: FileOrPath {
                from_textbox: "#!/bin/bash\n".to_owned(),
                ..Default::default()
            },
            prerm: FileOrPath {
                from_textbox: "#!/bin/bash\n".to_owned(),
                ..Default::default()
            },
            ..Default::default()
        }
    }
}
impl eframe::App for IpkBuilder {
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
                                        Some(path);
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
                                egui::Label::new(RichText::new(picked_path.to_string_lossy()).monospace()).wrap(true),
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
            ui.group(|ui| {
                ui.vertical_centered_justified(|ui| {
                    ui.label("debian binary");
                    ui.checkbox(&mut self.debian_binary.enabled, "default");
                    ui.horizontal(|ui| {
                        if !self.debian_binary.enabled {
                            ui.horizontal(|ui| {
                                if ui
                                    .add(egui::RadioButton::new(
                                        self.debian_binary.file_or_text == ScriptSource::FromPath,
                                        "from file",
                                    ))
                                    .clicked()
                                {
                                    self.debian_binary.file_or_text = ScriptSource::FromPath;
                                }
                                if ui
                                    .add(egui::RadioButton::new(
                                        self.debian_binary.file_or_text
                                            == ScriptSource::FromTextfield,
                                        "from input field",
                                    ))
                                    .clicked()
                                {
                                    self.debian_binary.file_or_text = ScriptSource::FromTextfield;
                                }
                                if self.debian_binary.file_or_text == ScriptSource::FromPath {
                                    if ui.button("Open file...").clicked() {
                                        if let Some(path) = rfd::FileDialog::new().pick_file() {
                                            self.debian_binary.picked_path =
                                                Some(path);
                                        }
                                    }
                                } else {
                                    ui.add_enabled(false, egui::Button::new("Open file..."));
                                };
                            });
                        }
                    });
                    if !self.debian_binary.enabled {
                        if let Some(picked_path) = &self.debian_binary.picked_path {
                            ui.horizontal(|ui| {
                                ui.label("Picked file:");
                                ui.add(
                                    egui::Label::new(RichText::new(picked_path.to_string_lossy()).monospace())
                                        .wrap(true),
                                );
                            });
                        }
                        if self.debian_binary.file_or_text == ScriptSource::FromTextfield {
                            let response = ui.add(
                                egui::TextEdit::multiline(&mut self.debian_binary.from_textbox)
                                    .code_editor(),
                            );
                        }
                    }
                });
            });
            ui.group(|ui| {
                ui.vertical_centered_justified(|ui| {
                    ui.label("postinst script");
                    ui.checkbox(&mut self.postinst.enabled, "use");
                    ui.horizontal(|ui| {
                        if self.postinst.enabled {
                            ui.horizontal(|ui| {
                                if ui
                                    .add(egui::RadioButton::new(
                                        self.postinst.file_or_text == ScriptSource::FromPath,
                                        "from file",
                                    ))
                                    .clicked()
                                {
                                    self.postinst.file_or_text = ScriptSource::FromPath;
                                }
                                if ui
                                    .add(egui::RadioButton::new(
                                        self.postinst.file_or_text == ScriptSource::FromTextfield,
                                        "from input field",
                                    ))
                                    .clicked()
                                {
                                    self.postinst.file_or_text = ScriptSource::FromTextfield;
                                }
                                if self.postinst.file_or_text == ScriptSource::FromPath {
                                    if ui.button("Open file...").clicked() {
                                        if let Some(path) = rfd::FileDialog::new().pick_file() {
                                            self.postinst.picked_path =
                                                Some(path);
                                        }
                                    }
                                } else {
                                    ui.add_enabled(false, egui::Button::new("Open file..."));
                                };
                            });
                        }
                    });

                    if self.postinst.enabled {
                        if let Some(picked_path) = &self.postinst.picked_path {
                            ui.horizontal(|ui| {
                                ui.label("Picked file:");
                                ui.add(
                                    egui::Label::new(RichText::new(picked_path.to_string_lossy()).monospace())
                                        .wrap(true),
                                );
                            });
                        }
                        if self.postinst.file_or_text == ScriptSource::FromTextfield {
                            let response = ui.add(
                                egui::TextEdit::multiline(&mut self.postinst.from_textbox)
                                    .code_editor(),
                            );
                        }
                    }
                });
            });

            ui.group(|ui| {
                ui.vertical_centered_justified(|ui| {
                    ui.label("preinst script");
                    ui.checkbox(&mut self.preinst.enabled, "use");
                    ui.horizontal(|ui| {
                        if self.preinst.enabled {
                            ui.horizontal(|ui| {
                                if ui
                                    .add(egui::RadioButton::new(
                                        self.preinst.file_or_text == ScriptSource::FromPath,
                                        "from file",
                                    ))
                                    .clicked()
                                {
                                    self.preinst.file_or_text = ScriptSource::FromPath;
                                }
                                if ui
                                    .add(egui::RadioButton::new(
                                        self.preinst.file_or_text == ScriptSource::FromTextfield,
                                        "from input field",
                                    ))
                                    .clicked()
                                {
                                    self.preinst.file_or_text = ScriptSource::FromTextfield;
                                }
                                if self.preinst.file_or_text == ScriptSource::FromPath {
                                    if ui.button("Open file...").clicked() {
                                        if let Some(path) = rfd::FileDialog::new().pick_file() {
                                            self.preinst.picked_path =
                                                Some(path);
                                        }
                                    }
                                } else {
                                    ui.add_enabled(false, egui::Button::new("Open file..."));
                                };
                            });
                        }
                    });

                    if self.preinst.enabled {
                        if let Some(picked_path) = &self.preinst.picked_path {
                            ui.horizontal(|ui| {
                                ui.label("Picked file:");
                                ui.add(
                                    egui::Label::new(RichText::new(picked_path.to_string_lossy()).monospace())
                                        .wrap(true),
                                );
                            });
                        }
                        if self.preinst.file_or_text == ScriptSource::FromTextfield {
                            let response = ui.add(
                                egui::TextEdit::multiline(&mut self.preinst.from_textbox)
                                    .code_editor(),
                            );
                        }
                    }
                });
            });

            ui.group(|ui| {
                ui.vertical_centered_justified(|ui| {
                    ui.label("prerm script");
                    ui.checkbox(&mut self.prerm.enabled, "use");
                    ui.horizontal(|ui| {
                        if self.prerm.enabled {
                            ui.horizontal(|ui| {
                                if ui
                                    .add(egui::RadioButton::new(
                                        self.prerm.file_or_text == ScriptSource::FromPath,
                                        "from file",
                                    ))
                                    .clicked()
                                {
                                    self.prerm.file_or_text = ScriptSource::FromPath;
                                }
                                if ui
                                    .add(egui::RadioButton::new(
                                        self.prerm.file_or_text == ScriptSource::FromTextfield,
                                        "from input field",
                                    ))
                                    .clicked()
                                {
                                    self.prerm.file_or_text = ScriptSource::FromTextfield;
                                }
                                if self.prerm.file_or_text == ScriptSource::FromPath {
                                    if ui.button("Open file...").clicked() {
                                        if let Some(path) = rfd::FileDialog::new().pick_file() {
                                            self.prerm.picked_path =
                                                Some(path);
                                        }
                                    }
                                } else {
                                    ui.add_enabled(false, egui::Button::new("Open file..."));
                                };
                            });
                        }
                    });

                    if self.prerm.enabled {
                        if let Some(picked_path) = &self.prerm.picked_path {
                            ui.horizontal(|ui| {
                                ui.label("Picked file:");
                                ui.add(
                                    egui::Label::new(RichText::new(picked_path.to_string_lossy()).monospace())
                                        .wrap(true),
                                );
                            });
                        }
                        if self.prerm.file_or_text == ScriptSource::FromTextfield {
                            let response = ui.add(
                                egui::TextEdit::multiline(&mut self.prerm.from_textbox)
                                    .code_editor(),
                            );
                        }
                    }
                });
            });

            ui.group(|ui| {
                ui.vertical_centered_justified(|ui| {
                    ui.label("Data folder root");
                    ui.horizontal(|ui| {
                        ui.horizontal(|ui| {
                            if ui.button("Set path..").clicked() {
                                if let Some(path) = rfd::FileDialog::new().pick_folder() {
                                    self.data_path = Some(path.display().to_string());
                                }
                            }
                        });
                    });
                });
                if let Some(picked_path) = &self.data_path {
                    ui.horizontal(|ui| {
                        ui.label("Picked path:");
                        ui.add(egui::Label::new(RichText::new(picked_path).monospace()).wrap(true));
                    });
                }
            });

            ui.group(|ui| {
                ui.vertical_centered_justified(|ui| {
                    ui.label("Output folder");
                    ui.horizontal(|ui| {
                        ui.horizontal(|ui| {
                            if ui.button("Set path..").clicked() {
                                if let Some(path) = rfd::FileDialog::new().pick_folder() {
                                    self.output_path = Some(path.display().to_string());
                                }
                            }
                        });
                    });
                });
                if let Some(picked_path) = &self.output_path {
                    ui.horizontal(|ui| {
                        ui.label("Picked path:");
                        ui.add(egui::Label::new(RichText::new(picked_path).monospace()).wrap(true));
                    });
                }
            });

            let mut success_or_not: Result<String, Error> = Err(anyhow!("empty"));
            ui.vertical_centered(|ui| {
                if (self.control_file.picked_path.is_some()
                    || self.control_file.file_or_text == ScriptSource::FromTextfield)
                    && (self.debian_binary.picked_path.is_some()
                        || self.debian_binary.file_or_text == ScriptSource::FromTextfield)
                    || self.debian_binary.enabled && self.output_path.is_some()
                    && self.data_path.is_some() && self.output_path.is_some()
                {
                    if ui
                        .add_sized([120., 40.], egui::Button::new("Build!").fill(Color32::BLUE))
                        .clicked()
                    {
                        success_or_not = make_package(self);
                    }
                } else {
                    ui.add_enabled(
                        false,
                        egui::Button::new("Build!")
                            .fill(Color32::DARK_GRAY)
                            .min_size(Vec2 { x: 120., y: 40. }),
                    );
                };
            });
            if success_or_not.is_ok() {
                ui.label("Success!")
            } else {
                ui.label("Failure!")
            }
        });
    }
}
