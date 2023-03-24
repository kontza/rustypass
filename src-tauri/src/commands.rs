use crate::config::{
    config_dir::ConfigDir, ConfigStore, ConfigStoreInterface, SCAN_DIRECTORY_VALUE,
};
use crate::scanner;
use copypasta::{ClipboardContext, ClipboardProvider};
use std::path::PathBuf;
use std::process::Command;
use tracing::{event, Level};

#[derive(Clone, serde::Serialize)]
struct ItemPayload {
    path: String,
}

fn get_scan_dir() -> String {
    let c = ConfigDir;
    let mut bd = ConfigStore {
        config_dir: Box::new(c),
    };
    let fallback = shellexpand::tilde("~/.local/share/gopass/stores");
    match bd.get_config(SCAN_DIRECTORY_VALUE) {
        Some(value) => value,
        None => fallback.to_string(),
    }
}

#[allow(dead_code)]
#[tauri::command]
pub fn start_scanning(window: tauri::Window) {
    let scan_dir = get_scan_dir();
    println!("Start scanning in '{}'...", &scan_dir);
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
                                path: rcv_path
                                    .strip_prefix(&scan_dir)
                                    .unwrap()
                                    .display()
                                    .to_string(),
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

#[allow(dead_code)]
#[tauri::command]
pub fn process_secret(secret: String) {
    let secret_file: PathBuf = [get_scan_dir(), secret].iter().collect();

    let result = Command::new("gpg")
        .arg("-qd")
        .arg(secret_file.as_path().display().to_string())
        .output()
        .unwrap();
    if !result.status.success() {
        println!("Would show an error.");
    } else {
        match std::str::from_utf8(&result.stdout) {
            Ok(stdout_string) => {
                let mut lines = stdout_string.lines();
                let mut ctx = ClipboardContext::new().unwrap();
                ctx.set_contents(lines.next().unwrap().to_string()).unwrap();
            }
            Err(e) => println!("Would show an error: {}", e),
        };
    }
}
