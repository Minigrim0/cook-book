use tauri;

#[tauri::command]
fn recipe_data() -> String {
    "Lol".to_string()
}
