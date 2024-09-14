use diesel::RunQueryDsl;
use log::{info, error};

use models::database::get_connection;

use tauri;

#[tauri::command]
pub fn reset_database(window: tauri::Window) -> Result<(), String> {
    info!("Asking confirmation to reset the database");
    tauri::api::dialog::confirm(Some(&window), "Reset database", "Are you sure you want to reset the database?", |ans| {
        if ans {
            info!("Resetting database");
            let conn = &mut match get_connection() {
                Ok(conn) => conn,
                Err(e) => {
                    error!("Failed to get database connection: {}", e);
                    return;
                }
            };

            if let Err(e) = diesel::sql_query("DELETE FROM recipe_ingredient;")
                .execute(conn) {
                    error!("Failed to delete recipe_ingredient: {}", e);
                    return;
            }
            if let Err(e) = diesel::sql_query("DELETE FROM step;")
                .execute(conn) {
                    error!("Failed to delete step: {}", e);
                    return;
            }
            if let Err(e) = diesel::sql_query("DELETE FROM ingredient;")
                .execute(conn) {
                    error!("Failed to delete ingredient: {}", e);
                    return;
            }
            if let Err(e) = diesel::sql_query("DELETE FROM recipe;")
                .execute(conn) {
                    error!("Failed to delete recipe: {}", e);
                    return;
            }
            info!("Database reset");
        } else {
            info!("Database reset cancelled");
        }
    });

    Ok(())
}