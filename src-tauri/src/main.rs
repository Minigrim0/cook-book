// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::sync::{Arc, Mutex};
use tauri::{Window, Manager};
use std::fs;

pub mod utils;
pub mod database;
pub mod parser;

#[tauri::command]
fn load_path(data_path: &str, window: Window) -> String {
    let path_to_load = Arc::new(Mutex::new(data_path.to_string()));

    std::thread::spawn(move || {
        let path_to_load = path_to_load.clone();
        let path_to_load = path_to_load.lock().unwrap();
        let paths = fs::read_dir(&*path_to_load).unwrap();
        let paths: Vec<_> = paths.collect();
        let count = paths.len();

        for (index, path) in paths.iter().enumerate() {
            let window = window.clone();
            let path = match path {
                Ok(entry) => entry,
                Err(err) => {
                    println!("An error occured while loading path {}", err.to_string());
                    continue;
                }
            };
            if let Some(error) = parser::json_parser::parse_recipe(path).err() {
                println!("Error while loading recipe: {}", error.to_string());
            }

            window
                .emit(
                    "loading://progress",
                    serde_json::json!({
                        "progress": index,
                        "total": count,
                        "path": path.path().to_str().unwrap(),
                    }),
                )
                .unwrap();
        }
    });

    "Started".to_string()
}

fn main() {
    tauri::Builder::default()
        .setup(|app| {
            let splashscreen_window = app.get_window("splashscreen").unwrap();
            let main_window = app.get_window("main").unwrap();
            // we perform the initialization code on a new task so the app doesn't freeze
            tauri::async_runtime::spawn(async move {
                database::init::init_database();
                splashscreen_window.close().unwrap();
                main_window.show().unwrap();
            });
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![load_path])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
