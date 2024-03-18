// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::{sync::{Arc, Mutex}, io};
use tauri::{Window, Manager};
use std::fs;
use threadpool::ThreadPool;

pub mod utils;
pub mod database;
pub mod parser;

#[tauri::command]
fn load_path(data_path: &str, window: Window) -> String {
    let path_to_load = Arc::new(Mutex::new(data_path.to_string()));

    std::thread::spawn(move || {
        let path_to_load = path_to_load.clone();
        let path_to_load = path_to_load.lock().unwrap();

        let paths = match fs::read_dir(&*path_to_load) {
            Ok(p) => match p.map(|res| res.map(|e| e.path()))
                                .collect::<Result<Vec<_>, io::Error>>() {
                                    Ok(f) => f,
                                    Err(e) => {
                                        println!("An error occured {}", e.to_string());
                                        return
                                    }
                                },
            Err(e) => {
                println!("An error occured {}", e.to_string());
                return
            }
        };
        let count = paths.len();

        let n_workers = 8;
        let pool = ThreadPool::new(n_workers);

        for (index, path) in paths.iter().enumerate() {
            let window = window.clone();
            let path = Arc::new(Mutex::new(path.clone()));

            pool.execute(move || {
                let window = window.clone();
                let path = match path.lock() {
                    Ok(p) => p,
                    Err(_) => return
                };
                if let Some(error) = parser::json_parser::parse_recipe(&path).err() {
                    println!("Error while loading recipe: {}", error.to_string());
                }

                window
                    .emit(
                        "loading://progress",
                        serde_json::json!({
                            "progress": index,
                            "total": count,
                            "path": path.to_str().unwrap(),
                        }),
                    )
                    .unwrap();
            });
        }

        pool.join();
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
