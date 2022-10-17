#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use eframe::egui;
use egui::{Align, Layout};
use egui_extras::{Size, TableBuilder};

fn main() {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "Rustypass",
        options,
        Box::new(|_cc| Box::new(MyApp::default())),
    );
}

struct MyApp {
    name: String,
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            name: "".to_owned(),
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            let main_dir = Layout::main_dir(&Layout::top_down(Align::Center));
            let cross_align = Align::Min;
            let layout = Layout::from_main_dir_and_cross_align(main_dir, cross_align)
                .with_cross_justify(true);
            ui.horizontal(|ui| {
                ui.label("Filter:");
                ui.text_edit_singleline(&mut self.name);
            });
            ui.separator();
            TableBuilder::new(ui)
                .cell_layout(layout)
                .striped(true)
                .column(Size::remainder().at_least(100.0))
                .header(20.0, |mut header| {
                    header.col(|ui| {
                        ui.heading("Stored passwords");
                    });
                })
                .body(|mut body| {
                    body.row(20.0, |mut row| {
                        row.col(|ui| {
                            if ui.button("ansible/become").clicked() {
                                println!("button clicked");
                            }
                        });
                    });
                })
        });
    }
}
