use tauri;
use log::{info, warn};

use crate::loader;

#[tauri::command]
pub fn load_path(window: tauri::Window) -> String {
    info!("Invoking dialog to load a folder");
    tauri::api::dialog::FileDialogBuilder::new().pick_folder(move |folder| match folder {
        Some(folder) => loader::load_folder(folder, window.clone()),
        None => warn!("Canceled folder loading"),
    });

    "ok".to_string()
}
