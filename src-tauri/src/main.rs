// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use database::{get_connection_pool, DatabasePool};
use log::{error, info, trace};
use std::fs;
use std::{
    io,
    sync::{Arc, Mutex},
};
use tauri::{Manager, Window};
use threadpool::ThreadPool;

use simplelog::{Config, LevelFilter, SimpleLogger};

pub mod database;
pub mod parser;
pub mod utils;

type SharedDatabasePool = Arc<DatabasePool>;

#[tauri::command]
fn load_path(data_path: &str, window: Window) -> String {
    let path_to_load = Arc::new(Mutex::new(data_path.to_string()));

    std::thread::spawn(move || {
        let path_to_load = path_to_load.clone();
        let path_to_load = path_to_load.lock().unwrap();

        let paths = match fs::read_dir(&*path_to_load) {
            Ok(p) => match p
                .map(|res| res.map(|e| e.path()))
                .collect::<Result<Vec<_>, io::Error>>()
            {
                Ok(f) => f,
                Err(e) => {
                    error!("An error occured {}", e.to_string());
                    return;
                }
            },
            Err(e) => {
                error!("An error occured {}", e.to_string());
                return;
            }
        };
        let count = paths.len();

        let n_workers = 8;
        let pool = ThreadPool::new(n_workers);
        let db_pool = get_connection_pool().unwrap();

        for (index, path) in paths.iter().enumerate() {
            let window = window.clone();
            let path = Arc::new(Mutex::new(path.clone()));
            let db_pool = db_pool.clone();

            pool.execute(move || {
                let window = window.clone();
                let path = match path.lock() {
                    Ok(p) => p,
                    Err(_) => return,
                };

                // Emit start
                window
                    .emit(
                        "loading://starting",
                        serde_json::json!({
                            "path": path.to_str().unwrap(),
                            "id": index,
                            "total": count
                        }),
                    )
                    .unwrap();

                if let Err(error) = parser::json_parser::parse_recipe(&path, Arc::new(db_pool)) {
                    error!("Error while loading recipe: {}", error.to_string());

                    // Emit error
                    window
                        .emit(
                            "loading://error",
                            serde_json::json!({
                                "id": index,
                                "error": error.to_string()
                            }),
                        )
                        .unwrap();
                } else {
                    trace!("Loaded recipe {}", path.to_str().unwrap_or("err"));
                    // Emit completed
                    window
                        .emit(
                            "loading://completed",
                            serde_json::json!({
                                "id": index,
                            }),
                        )
                        .unwrap();
                }
            });
        }

        pool.join();
    });

    "Started".to_string()
}

fn main() {
    let _ = SimpleLogger::init(LevelFilter::Info, Config::default());
    info!("Starting cook-book application");

    tauri::Builder::default()
        .setup(|app| {
            let splashscreen_window = app.get_window("splashscreen").unwrap();
            let main_window = app.get_window("main").unwrap();
            // we perform the initialization code on a new task so the app doesn't freeze
            tauri::async_runtime::spawn(async move {
                // TODO: Implement stuff here

                splashscreen_window.close().unwrap();
                main_window.show().unwrap();
            });

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![load_path])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
