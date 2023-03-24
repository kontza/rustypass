#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use tauri::Manager;
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
        .invoke_handler(tauri::generate_handler![
            rustypass::commands::start_scanning,
            rustypass::commands::process_secret,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
