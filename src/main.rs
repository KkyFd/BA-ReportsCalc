#![windows_subsystem = "windows"]
mod character;
mod errors;
mod reports;
mod state;

use character::Character;
use errors::AppError;
use reports::Reports;
use serde::Deserialize;
use state::State;
use thiserror::*;

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
#[derive(Deserialize)]
struct ExpTable {
    exp_needed: Vec<u32>,
}
struct AppState {
    reports: Reports,
    textures: Icons,
    character: Character,
    characters: Char,
    character_selection_text: String,
    exp_table: ExpTable,
    desired_level: String,
    calc_result: Result<String, AppError>,
}

impl AppState {
    fn new(cc: &eframe::CreationContext<'_>, reports: Reports, character: Character) -> Self {
        let mut characters: HashMap<String, Character> = HashMap::new();
        characters.insert(
            String::from("Asuna"),
            Character {
                name: String::from("Asuna"),
                ..Character::default()
            },
        );
        let table = std::fs::File::open("level_table.json").expect("Failed to load JSON");
        let exp_table: ExpTable = serde_json::from_reader(table).unwrap();
        Self {
            reports,
            textures: Self::load_textures(cc),
            character,
            characters,
            character_selection_text: String::new(),
            exp_table,
            desired_level: String::new(),
            calc_result: Ok(String::new()),
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
    fn calculate(&mut self) -> Result<String, AppError> {
        let purple_reports = (self.reports.quantities[0] / 200.0)
            + (self.reports.quantities[1] / 20.0)
            + (self.reports.quantities[2] / 5.0)
            + self.reports.quantities[3];
        let exp = purple_reports * 10000.0;
        self.reports.purple_reports = Some(purple_reports);
        self.reports.exp = Some(exp);

        let desired = self
            .desired_level
            .parse::<u8>()
            .map_err(|_| AppError::InvalidValue)?;
        let desired_index = desired - 1;
        let current_exp = self
            .character
            .current_exp
            .parse::<u32>()
            .map_err(|_| AppError::InvalidValue)?;
        let total_exp = self.exp_table.exp_needed[desired_index as usize] + current_exp;
        if self.character.level < desired_index {
            Ok(format!(
                "Exp needed to reach level {desired}: {}",
                self.exp_table.exp_needed[desired_index as usize] - total_exp
            ))
        } else {
            Err(AppError::SmallerLevel)
        }
    }
}

impl App for AppState {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
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
                        ui.vertical(|ui| {
                            ui.heading("Character Details");
                            ui.horizontal(|ui| {
                                ui.label(format!("Name:"));
                                ui.add(
                                    egui::TextEdit::singleline(&mut self.character_selection_text)
                                        .desired_width(50.0),
                                );
                            });
                            if let Some(character) =
                                self.characters.get(&self.character_selection_text)
                            {
                                ui.label(format!("Level: {}", character.level));
                                ui.horizontal(|ui| {
                                    ui.label(format!("Current Exp:"));
                                    ui.add(
                                        egui::TextEdit::singleline(&mut self.character.current_exp)
                                            .desired_width(50.0)
                                            .hint_text("Level EXP"),
                                    );
                                });
                                ui.horizontal(|ui| {
                                    ui.label(format!("Desired level:"));
                                    ui.add(
                                        egui::TextEdit::singleline(&mut self.desired_level)
                                            .desired_width(50.0)
                                            .hint_text("Level"),
                                    );
                                });
                            }
                        });
                    });
                });

                // Buttons Group
                ui.horizontal(|ui| {
                    if ui.button("Calculate").clicked() {
                        self.calc_result = self.calculate();
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

                //ui.label(&self.calc_result);
            });
        });
    }
}
