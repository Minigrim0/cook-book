use log::warn;
use diesel::prelude::*;

use crate::models::Recipe;
use crate::database::SharedDatabasePool;

pub fn get_recipe(recipe_id: i32, pool: &SharedDatabasePool) -> Result<Recipe, String> {
    use crate::schema::recipe::dsl::*;

    let conn = &mut pool.get().map_err(|e| e.to_string())?;
    match recipe
        .select(Recipe::as_select())
        .find(recipe_id)
        .first(conn)
        .optional() {
            Ok(Some(r)) => Ok(r),
            Ok(None) => Err("No recipe found".to_string()),
            Err(e) => Err(format!("Unable to load recipe: {}", e.to_string()))
        }
}

pub fn count_recipes(pool: &SharedDatabasePool) -> Result<usize, String> {
    use crate::schema::recipe::dsl::*;

    let conn = &mut pool.get().map_err(|e| e.to_string())?;

    match recipe
        .select(Recipe::as_select())
        .load(conn) {
            Ok(data) => Ok(data.len()),
            Err(e) => {
                warn!("Unable to load recipes: {}", e.to_string());
                Err(format!("Unable to load recipe: {}", e.to_string()))
            }
        }
}

pub fn filter_recipes(pattern: String, pool: &SharedDatabasePool) -> Result<Vec<Recipe>, String> {
    use crate::schema::recipe::dsl::*;

    let conn = &mut pool.get().map_err(|e| e.to_string())?;

    match recipe
        .select(Recipe::as_select())
        .filter(name.like(format!("%{}%", pattern)))
        .load(conn)
    {
        Ok(data) => Ok(data),
        Err(e) => {
            warn!("unable to filter recipes: {}", e.to_string());
            Err(format!("unable to filter recipes: {}", e.to_string()))
        }
    }
}
