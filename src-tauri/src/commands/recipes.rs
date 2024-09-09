use diesel::prelude::*;
use log::{error, info, warn};
use tauri;

use models::{RecipeMeta, PaginatedRecipe, CompleteRecipe};
use models::get_connection_pool;
use models::models::{Recipe, Ingredient, Cuisine};


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
pub fn filter_recipes(pattern: String, limit: i32, offset: i32) -> PaginatedRecipe {
    info!("filtering ingredients");
    use models::schema::recipe::dsl::*;

    let db_pool = match get_connection_pool() {
        Ok(pool) => pool,
        Err(e) => {
            error!("unable to get connection pool: {}", e.to_string());
            return (Vec::new(), 0, 0);
        }
    };
    let conn = &mut match db_pool.get() {
        Ok(conn) => conn,
        Err(e) => {
            error!("unable to get connection pool: {}", e.to_string());
            return (Vec::new(), 0, 0);
        }
    };

    let data = match recipe
        .select(Recipe::as_select())
        .filter(name.like(format!("%{}%", pattern)))
        .load(conn)
    {
        Ok(data) => data,
        Err(e) => {
            warn!("unable to filter recipes: {}", e.to_string());
            Vec::new()
        }
    };

    let total = data.len();
    let data = data
        .into_iter()
        .skip(offset as usize)
        .take(limit as usize)
        .collect();

    (data, total, total / limit as usize + 1)
}

#[tauri::command]
pub fn load_recipe(recipe_id: i32) -> Result<CompleteRecipe, String> {
    info!("Loading recipe {}", recipe_id);

    use models::schema::recipe::dsl::*;

    let db_pool = match get_connection_pool() {
        Ok(pool) => pool,
        Err(e) => {
            error!("unable to get connection pool: {}", e.to_string());
            return Err(e.to_string());
        }
    };
    let conn = &mut match db_pool.get() {
        Ok(conn) => conn,
        Err(e) => {
            error!("unable to get connection pool: {}", e.to_string());
            return Err(e.to_string());
        }
    };


    match recipe
        .select(Recipe::as_select())
        .find(recipe_id)
        .first(conn)
        .optional() {
            Ok(Some(r)) => {
                info!("Loaded recipe {}", r.name);
                Err("Not implemented".to_string())
            },
            Ok(None) => {
                warn!("Unable to find recipe {}", recipe_id);
                Err("not found".to_string())
            },
            Err(e) => {
                error!("Unable to load recipe {}: {}", recipe_id, e.to_string());
                Err(e.to_string())
            }
        }
}
