// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::Window;

#[tauri::command]
fn load_path(path: &str, window: Window) -> String {
    let total = 100000;

    for i in 0..total {
        let window = window.clone();
        std::thread::spawn(move || {
            window
                .emit(
                    "loading://progress",
                    serde_json::json!({
                        "progress": i,
                        "total": total,
                    }),
                )
                .unwrap();
        });
        // Sleep
        std::thread::sleep(std::time::Duration::from_millis(10));
        println!("{}: {}", path, i);
    }

    format!("done with {}", path)
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![load_path])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
