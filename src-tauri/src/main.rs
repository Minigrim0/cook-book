// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use log::info;
use tauri::Manager;

use simplelog::{Config, LevelFilter, SimpleLogger};

mod commands;
mod loader;
mod parser;

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
        .invoke_handler(tauri::generate_handler![
            commands::load_path,
            commands::recipe_data,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
