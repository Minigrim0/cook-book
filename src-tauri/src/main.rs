// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use log::info;
use tauri::{Manager, SystemTray, SystemTrayEvent};

use simplelog::{Config, LevelFilter, SimpleLogger};

mod commands;
mod loader;
mod parser;

fn main() {
    let tray = SystemTray::new();

    let _ = SimpleLogger::init(LevelFilter::Info, Config::default());
    info!("Starting Cook Book");

    tauri::Builder::default()
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
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
