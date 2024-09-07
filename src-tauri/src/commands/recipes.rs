use tauri;
use diesel::prelude::*;
use log::{info, warn};

use models::models::*;
use models::{get_connection_pool, RecipeMeta};


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
