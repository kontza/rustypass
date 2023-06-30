#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use tauri::{
    CustomMenuItem, Manager, SystemTray, SystemTrayEvent, SystemTrayMenu, SystemTrayMenuItem,
};
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
    const HIDE: &str = "hide";
    const QUIT: &str = "quit";
    const SHOW: &str = "show";
    const MAIN_WINDOW_LABEL: &str = "main";

    let hide_cmd = CustomMenuItem::new(
        HIDE,
        format!(
            "{}{}",
            HIDE[0..1].to_uppercase().to_string(),
            HIDE[1..].to_string()
        ),
    );
    let quit_cmd = CustomMenuItem::new(
        QUIT,
        format!(
            "{}{}",
            QUIT[0..1].to_uppercase().to_string(),
            QUIT[1..].to_string()
        ),
    );
    let show_cmd = CustomMenuItem::new(
        SHOW,
        format!(
            "{}{}",
            SHOW[0..1].to_uppercase().to_string(),
            SHOW[1..].to_string()
        ),
    );
    let tray_menu = SystemTrayMenu::new()
        .add_item(hide_cmd)
        .add_item(show_cmd)
        .add_native_item(SystemTrayMenuItem::Separator)
        .add_item(quit_cmd);
    let tray = SystemTray::new().with_menu(tray_menu);
    tauri::Builder::default()
        .system_tray(tray)
        .on_system_tray_event(|app, event| {
            match event {
                SystemTrayEvent::MenuItemClick { id, .. } => match id.as_str() {
                    HIDE => {
                        let main_window = app.get_window(MAIN_WINDOW_LABEL).unwrap();
                        main_window.hide().unwrap();
                    }
                    QUIT => {
                        std::process::exit(0);
                    }
                    SHOW => {
                        let main_window = app.get_window(MAIN_WINDOW_LABEL).unwrap();
                        main_window.show().unwrap();
                    }
                    _ => {}
                },
                _ => {}
            }
            ()
        })
        .setup(|app| {
            let main_window = app.get_window(MAIN_WINDOW_LABEL).unwrap();
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
