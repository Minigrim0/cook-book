// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use log::info;
use tauri::{Manager, SystemTray, SystemTrayEvent};
use simplelog::{Config, LevelFilter, SimpleLogger};
use std::error::Error;
use models::SharedDatabasePool;
use std::sync::Arc;

mod commands;
mod loader;
mod parser;

#[cfg(test)]
mod tests;

fn main() -> Result<(), Box<dyn Error>> {
    let tray = SystemTray::new();

    let _ = SimpleLogger::init(LevelFilter::Info, Config::default());
    info!("Starting Cook Book");

    // Initialize the SQLite connection pool
    let db_pool: SharedDatabasePool = Arc::new(models::database::get_connection_pool()
        .expect("Failed to create pool"));

    tauri::Builder::default()
        .manage(db_pool)
        .on_window_event(|event| match event.event() {
            tauri::WindowEvent::CloseRequested { api, .. } => {
                event.window().hide().unwrap();
                api.prevent_close();
            }
            _ => {}
        })
        .system_tray(tray)
        .on_system_tray_event(|app, event| match event {
            SystemTrayEvent::LeftClick { .. } => {
                let window = app.get_window("main").unwrap();
                window.show().unwrap();
            }
            _ => (),
        })
        .invoke_handler(tauri::generate_handler![
            commands::load_path,
            commands::recipe_meta,
            commands::filter_recipes,
            commands::load_recipe,
            commands::filter_ingredients,
            commands::reset_database,
            commands::get_job_status,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");

    Ok(())
}
