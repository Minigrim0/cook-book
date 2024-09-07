use tauri;
use diesel::prelude::*;
use log::{info, warn, error};

use models::models::*;
use models::get_connection_pool;

#[tauri::command]
/// Get a list of ingredients. The limit and offset are used for pagination.
/// The limit is the amount of items to return, and the offset is the amount of items to skip.
/// The limit and offset are both 0-indexed.
/// The return value is a tuple of the list of ingredients (in the current page) and the total amount of ingredients.
pub fn get_ingredients(limit: i32, offset: i32) -> PaginatedIngredients {
    info!("getting ingredients");
    use models::schema::ingredient::dsl::*;

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

    let data: Vec<Ingredient> = match ingredient.select(Ingredient::as_select()).load(conn) {
        Ok(data) => data,
        Err(e) => {
            warn!("unable to load ingredients: {}", e.to_string());
            Vec::new()
        }
    };

    let total: usize = data.len();
    info!("total ingredients: {}", total);
    let data: Vec<Ingredient> = data.into_iter().skip(offset as usize).take(limit as usize).collect();
    info!("returning {} ingredients", data.len());

    (data, total, total / limit as usize + 1)
}

#[tauri::command]
/// Get a list of ingredients that match the given pattern.
/// The limit and offset are used for pagination.
/// The limit is the amount of items to return, and the offset is the amount of items to skip.
/// The limit and offset are both 0-indexed.
/// The return value is a tuple of the list of ingredients (in the current page) and the total amount of ingredients.
pub fn filter_ingredients(pattern: String, limit: i32, offset: i32) -> PaginatedIngredients {
    info!("filtering ingredients");
    use models::schema::ingredient::dsl::*;

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

    let data = match ingredient
        .select(Ingredient::as_select())
        .filter(name.like(format!("%{}%", pattern)))
        .load(conn)
    {
        Ok(data) => data,
        Err(e) => {
            warn!("unable to filter ingredients: {}", e.to_string());
            Vec::new()
        }
    };

    let total = data.len();
    let data = data.into_iter().skip(offset as usize).take(limit as usize).collect();

    (data, total, total / limit as usize + 1)
}
