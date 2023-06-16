#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

// use std::alloc::System;

use tauri::{CustomMenuItem, SystemTray, SystemTrayEvent, SystemTrayMenu, SystemTrayMenuItem};
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

fn main() {
    const QUIT: &str = "quit";
    const SHOW: &str = "show";
    const QUIT_LABEL: &str = "Quit";
    const SHOW_LABEL: &str = "Show";

    let quit = CustomMenuItem::new(QUIT.to_string(), QUIT_LABEL);
    let show = CustomMenuItem::new(SHOW.to_string(), SHOW_LABEL);
    let tray_menu = SystemTrayMenu::new()
        .add_item(show)
        .add_native_item(SystemTrayMenuItem::Separator)
        .add_item(quit);
    let tray = SystemTray::new().with_menu(tray_menu);
    tauri::Builder::default()
        .system_tray(tray)
        .on_system_tray_event(|app, event| {
            match event {
                SystemTrayEvent::MenuItemClick { id, .. } => match id.as_str() {
                    QUIT => {
                        std::process::exit(0);
                    }
                    SHOW => {
                        let main_window = tauri::WindowBuilder::new(
                            app,
                            "pääikkuna",
                            tauri::WindowUrl::App("index.html".into()),
                        )
                        .build()
                        .unwrap();
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
                    }
                    _ => {}
                },
                _ => {}
            }
            ()
        })
        .setup(|_| Ok(()))
        .invoke_handler(tauri::generate_handler![
            rustypass::commands::start_scanning,
            rustypass::commands::process_secret,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
