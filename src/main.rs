#![windows_subsystem = "windows"]

use eframe::{egui, App};
use egui::TextureHandle;
use image::ImageReader;
use serde::{Deserialize, Serialize};
use std::{
    fs,
    io::{Read, Write},
};

fn main() {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "BA Reports",
        options,
        Box::new(|cc| {
            let file_path = "reports.json";
            let reports = Reports::load_from_file(file_path).unwrap_or_default();
            let textures = Textures::load(cc);
            Ok(Box::new(AppState { reports, textures }) as Box<dyn App>)
        }),
    );
}

#[derive(Serialize, Deserialize, Default)]
struct Reports {
    quantities: [f32; 4],
    purple_reports: Option<f32>,
    exp: Option<f32>,
}

impl Reports {
    fn load_from_file(file_path: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let mut file = fs::File::open(file_path)?;
        let mut data = String::new();
        file.read_to_string(&mut data)?;
        let mut reports: Reports = serde_json::from_str(&data)?;
        reports.purple_reports = None;
        reports.exp = None;
        Ok(reports)
    }

    fn save_to_file(&self, file_path: &str) -> Result<(), Box<dyn std::error::Error>> {
        let data = serde_json::to_string_pretty(self)?;
        let mut file = fs::File::create(file_path)?;
        file.write_all(data.as_bytes())?;
        Ok(())
    }
}

struct Textures {
    white_report: TextureHandle,
    blue_report: TextureHandle,
    orange_report: TextureHandle,
    purple_report: TextureHandle,
}

impl Textures {
    fn load(cc: &eframe::CreationContext<'_>) -> Self {
        let load_texture = |path: &str| {
            let image = ImageReader::open(path)
                .unwrap()
                .decode()
                .unwrap()
                .to_rgba8();
            let size = [image.width() as usize, image.height() as usize];
            let pixels = image.into_raw();
            cc.egui_ctx.load_texture(
                path,
                egui::ColorImage::from_rgba_unmultiplied(size, &pixels),
                egui::TextureOptions::default(),
            )
        };

        Self {
            white_report: load_texture("Icons/white_report.png"),
            blue_report: load_texture("Icons/blue_report.png"),
            orange_report: load_texture("Icons/orange_report.png"),
            purple_report: load_texture("Icons/purple_report.png"),
        }
    }
}

struct AppState {
    reports: Reports,
    textures: Textures,
}

impl App for AppState {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Reports Amount");

            let labels = [
                ("White Reports", &self.textures.white_report),
                ("Blue Reports", &self.textures.blue_report),
                ("Orange Reports", &self.textures.orange_report),
                ("Purple Reports", &self.textures.purple_report),
            ];

            for (i, (label, texture)) in labels.iter().enumerate() {
                ui.horizontal(|ui| {
                    ui.image((texture.id(), egui::Vec2::new(128.0, 120.0)));
                    ui.add(egui::Slider::new(
                        &mut self.reports.quantities[i],
                        0.0..=50000.0,
                    ));
                    ui.label(format!("{}: {}", label, self.reports.quantities[i]));
                });
            }

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
                    let file_path = "reports.json";
                    if let Err(err) = self.reports.save_to_file(file_path) {
                        eprintln!("Failed to save reports: {}", err);
                    }
                }
            });

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
    }
}
