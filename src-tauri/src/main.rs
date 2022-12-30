#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use std::path::PathBuf;

use rustypass::{
    config::{config_dir::ConfigDir, ConfigStore, ConfigStoreInterface, SCAN_DIRECTORY_VALUE},
    scanner,
};
use tauri::Manager;
use tracing::{event, Level};
use tracing_subscriber;

struct AppTraceWriter {
    window: tauri::Window,
}
#[derive(Clone, serde::Serialize)]
struct TracePayload {
    message: String,
}
impl std::io::Write for AppTraceWriter {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        let s = String::from_utf8_lossy(buf);
        self.window
            .emit(
                "TRACE",
                TracePayload {
                    message: s.to_string(),
                },
            )
            .unwrap();
        Ok(s.len())
    }

    fn flush(&mut self) -> std::io::Result<()> {
        Ok(())
    }
}

#[derive(Clone, serde::Serialize)]
struct ItemPayload {
    path: String,
}

#[tauri::command]
fn start_scanning(window: tauri::Window) {
    let c = ConfigDir;
    let mut bd = ConfigStore {
        config_dir: Box::new(c),
    };
    let fallback = shellexpand::tilde("~/.local/share/gopass/stores");
    let scan_dir = match bd.get_config(SCAN_DIRECTORY_VALUE) {
        Some(value) => value,
        None => fallback.to_string(),
    };
    let rx = scanner::do_start_scanning(&PathBuf::from(&scan_dir));
    for received in rx {
        let rcv_path = PathBuf::from(received.path.clone());
        match received.result {
            Ok(result) => {
                if result == true {
                    let payload = format!(
                        "Found file: {:?}",
                        rcv_path.strip_prefix(&scan_dir).unwrap()
                    );
                    event!(Level::INFO, payload);
                    window
                        .emit(
                            "ITEM_FOUND",
                            ItemPayload {
                                path: received.path,
                            },
                        )
                        .unwrap();
                } else {
                    let payload = format!("Unknown media type: {}", received.path);
                    event!(Level::INFO, payload);
                }
            }
            Err(err) => {
                let payload = format!("{}: {:?}", rcv_path.display(), err);
                event!(Level::ERROR, payload);
            }
        }
    }
}

fn main() {
    tauri::Builder::default()
        .setup(|app| {
            let main_window = app.get_window("main").unwrap();
            let make_my_writer = move || -> Box<dyn std::io::Write> {
                Box::new(AppTraceWriter {
                    window: main_window.clone(),
                })
            };
            let format = tracing_subscriber::fmt::format().json();
            tracing_subscriber::fmt()
                .event_format(format)
                .with_writer(make_my_writer)
                .init();
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![start_scanning])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
