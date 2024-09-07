use diesel::prelude::*;
use log::{info, warn};
use tauri;

use super::loader;
use models::models::*;
use models::{get_connection_pool, RecipeMeta};

#[tauri::command]
pub fn get_ingredients() -> Vec<Ingredient> {
    info!("getting ingredients");

    use models::schema::ingredient::dsl::*;

    let db_pool = get_connection_pool().unwrap();
    let conn = &mut db_pool.get().unwrap();

    match ingredient.select(Ingredient::as_select()).order_by(name).load(conn) {
        Ok(data) => data,
        Err(e) => {
            warn!("unable to load ingredients: {}", e.to_string());
            Vec::new()
        }
    }
}

#[tauri::command]
pub fn recipe_meta() -> RecipeMeta {
    info!("getting recipe meta");

    let db_pool = get_connection_pool().unwrap();
    let conn = &mut db_pool.get().unwrap();

    use models::schema::recipe::dsl::*;
    let recipes_amount = match recipe.select(Recipe::as_select()).load(conn) {
        Ok(data) => data.len(),
        Err(e) => {
            warn!("Unable to load recipes: {}", e.to_string());
            0
        }
    };

    use models::schema::cuisine::dsl::*;
    let cuisine_amount = match cuisine.select(Cuisine::as_select()).load(conn) {
        Ok(data) => data.len(),
        Err(e) => {
            warn!("Unable to load cuisine: {}", e.to_string());
            0
        }
    };

    use models::schema::ingredient::dsl::*;
    let ingredients_amount = match ingredient.select(Ingredient::as_select()).load(conn) {
        Ok(data) => data.len(),
        Err(e) => {
            warn!("unable to load ingredients: {}", e.to_string());
            0
        }
    };

    RecipeMeta {
        recipe_amount: recipes_amount as i32,
        cuisine_amount: cuisine_amount as i32,
        ingredients_amount: ingredients_amount as i32,
    }
}

#[tauri::command]
pub fn load_path(window: tauri::Window) -> String {
    info!("Invoking dialog to load a folder");
    tauri::api::dialog::FileDialogBuilder::new().pick_folder(move |folder| match folder {
        Some(folder) => loader::load_folder(folder, window.clone()),
        None => info!("Canceled folder loading"),
    });

    "ok".to_string()
}
