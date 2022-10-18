#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use eframe::egui;
use egui::{Align, Layout};
use egui_extras::{Size, TableBuilder};
use walkdir::{DirEntry, WalkDir};

static ROOT_PATH: &str = "/Users/juharu/.local/share/gopass/stores/root";

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
    entries: Vec<String>,
}

impl Default for MyApp {
    fn default() -> Self {
        find_entries();
        Self {
            name: "".to_owned(),
            entries: find_entries(),
        }
    }
}

fn is_hidden(entry: &DirEntry) -> bool {
    entry
        .file_name()
        .to_str()
        .map(|s| s.starts_with("."))
        .unwrap_or(false)
}

pub fn find_entries() -> Vec<String> {
    let mut entries: Vec<String> = Vec::new();
    let walker = WalkDir::new(ROOT_PATH).follow_links(true).into_iter();
    for entry in walker.filter_entry(|e| !is_hidden(e)) {
        let unwrapped = entry.unwrap();
        if unwrapped.file_type().is_file() {
            if let Ok(relative) = unwrapped.path().strip_prefix(ROOT_PATH) {
                let as_string = relative.as_os_str().to_str().unwrap();
                entries.push(as_string.to_string());
            }
        }
    }
    return entries;
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
                    for entry in &mut self.entries {
                        body.row(20.0, |mut row| {
                            row.col(|ui| {
                                if ui.button(entry.to_string()).clicked() {
                                    println!("button clicked");
                                }
                            });
                        });
                    }
                })
        });
    }
}
