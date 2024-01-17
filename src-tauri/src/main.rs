// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::Window;
use std::fs;


#[tauri::command]
fn load_path(data_path: &str, window: Window) -> String {
    let path_to_load = data_path.clone();

    std::thread::spawn(move || {
        let paths = fs::read_dir(path_to_load).unwrap();

        for path in paths {
            println!("Name: {}", path.unwrap().path().display())
        }

        let total = 100000;
        for i in 0..total {
            let window = window.clone();
            window
                .emit(
                    "loading://progress",
                    serde_json::json!({
                        "progress": i,
                        "total": total,
                    }),
                )
                .unwrap();
            std::thread::sleep(std::time::Duration::from_millis(10));
        }
    });

    "Started".to_string()
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![load_path])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
