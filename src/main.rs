#![windows_subsystem = "windows"]
mod character;
mod reports;
mod state;

use character::Character;
use reports::Reports;
use state::State;

use std::collections::HashMap;

use eframe::egui;
use eframe::App;
use image::ImageReader;

type Icons = HashMap<String, egui::TextureHandle>;
type Char = HashMap<String, Character>;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let wrapper = AppStateWrapper {
        reports: Reports::load_from_file("reports.json").unwrap_or_default(),
        character: Character::load_from_file("characters.json").unwrap_or_default(),
    };
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder {
            inner_size: Some(egui::vec2(800.0, 650.0)),
            ..Default::default()
        },
        ..Default::default()
    };
    let _ = eframe::run_native(
        "BA Reports",
        options,
        Box::new(|cc| {
            Ok(Box::new(AppState::new(cc, wrapper.reports, wrapper.character)) as Box<dyn App>)
        }),
    );
    Ok(())
}

struct AppStateWrapper {
    reports: Reports,
    character: Character,
}

struct AppState {
    reports: Reports,
    textures: Icons,
    character: Character,
}

impl AppState {
    fn new(cc: &eframe::CreationContext<'_>, reports: Reports, character: Character) -> Self {
        Self {
            reports,
            textures: Self::load_textures(cc),
            character,
        }
    }
    fn load_textures(cc: &eframe::CreationContext<'_>) -> Icons {
        let paths = [
            ("white_report", "Icons/white_report.png"),
            ("blue_report", "Icons/blue_report.png"),
            ("orange_report", "Icons/orange_report.png"),
            ("purple_report", "Icons/purple_report.png"),
        ];

        paths
            .iter()
            .map(|(key, path)| {
                let image = ImageReader::open(path)
                    .unwrap()
                    .decode()
                    .unwrap()
                    .to_rgba8();
                let size = [image.width() as usize, image.height() as usize];
                let pixels = image.into_raw();
                let texture = cc.egui_ctx.load_texture(
                    *path,
                    egui::ColorImage::from_rgba_unmultiplied(size, &pixels),
                    egui::TextureOptions::default(),
                );
                (key.to_string(), texture)
            })
            .collect()
    }
}

impl App for AppState {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            // Create a vertical layout for the main content
            ui.vertical(|ui| {
                ui.horizontal(|ui| {
                    // Reports Group
                    ui.group(|ui| {
                        let labels = [
                            ("White Reports", "white_report"),
                            ("Blue Reports", "blue_report"),
                            ("Orange Reports", "orange_report"),
                            ("Purple Reports", "purple_report"),
                        ];
                        // Reports Text Boxes
                        ui.vertical(|ui| {
                            ui.heading("Reports Amount");
                            for (i, (label, key)) in labels.iter().enumerate() {
                                ui.horizontal(|ui| {
                                    if let Some(texture) = self.textures.get(*key) {
                                        ui.image((texture.id(), egui::Vec2::new(120.0, 120.0)));
                                    }
                                    let mut quantity_str = self.reports.quantities[i].to_string();
                                    ui.add(
                                        egui::TextEdit::singleline(&mut quantity_str)
                                            .desired_width(50.0),
                                    );
                                    if let Ok(value) = quantity_str.parse::<f32>() {
                                        self.reports.quantities[i] = value;
                                    }
                                    ui.label(format!("{}: {}", label, self.reports.quantities[i]));
                                });
                            }
                        });
                    });

                    // Characters Group
                    ui.group(|ui| {
                        let mut desired_level = String::with_capacity(2);
                        ui.vertical(|ui| {
                            if let None = &self.character.name {
                                ui.add(
                                    egui::TextEdit::singleline(&mut desired_level).desired_width(50.0),
                                );
                            }
                            if let Ok(level) = desired_level.parse::<u8>() {
                                desired_level = level.to_string();
                            }
                            if let (Some(name), Some(level)) =
                                (&self.character.name, &self.character.level)
                            {
                                ui.heading("Character Details");
                                ui.label(format!("{:?}", name));
                                ui.label(format!("Level: {}", level));
                                ui.heading(format!("Inputs"));
                                ui.add(
                                    egui::TextEdit::singleline(&mut desired_level).desired_width(50.0),
                                );
                            }
                        });
                    });
                });

                // Buttons Group (placed below both groups)
                ui.horizontal(|ui| {
                    if ui.button("Convert").clicked() {
                        let purple_reports = (self.reports.quantities[0] / 200.0)
                            + (self.reports.quantities[1] / 20.0)
                            + (self.reports.quantities[2] / 5.0)
                            + self.reports.quantities[3];
                        let exp = purple_reports * 10000.0;
                        self.reports.purple_reports = Some(purple_reports);
                        self.reports.exp = Some(exp);
                    }

                    if ui.button("Clear").clicked() {
                        self.reports.purple_reports = None;
                        self.reports.exp = None;
                    }

                    if ui.button("Save").clicked() {
                        let reports_path = "reports.json";
                        let character_path = "characters.json";
                        if let Err(err) = self.reports.save_to_file(reports_path) {
                            eprintln!("Failed to save reports: {}", err);
                        }
                        if let Err(err) = self.character.save_to_file(character_path) {
                            eprintln!("Failed to save characters: {}", err);
                        }
                    }
                });

                // Bottom Results
                if let Some(purple_reports) = self.reports.purple_reports {
                    ui.separator();
                    ui.label(format!(
                        "Quantity of Orange Reports: {:.2}",
                        purple_reports * 5.
                    ));
                    ui.label(format!("Quantity of Purple Reports: {:.2}", purple_reports));
                }

                if let Some(exp) = self.reports.exp {
                    ui.label(format!("Quantity of EXP: {}", exp));
                }
            });
        });
    }
}