use diesel::prelude::*;
use log::{error, info, warn};
use tauri;

use models::{RecipeMeta, PaginatedRecipe, CompleteRecipe};
use models::get_connection_pool;
use models::models::{Recipe, Ingredient, Cuisine};


#[tauri::command]
/// Loads various metadata about the recipes like the amount of recipes in the
/// database, the amount of cuisines, ingredients, ...
pub fn recipe_meta() -> Result<RecipeMeta, String> {
    info!("getting recipe meta");

    Ok(RecipeMeta {
        recipe_amount: models::database::loadables::count_recipes()? as i32,
        cuisine_amount: models::database::loadables::count_cuisines()? as i32,
        ingredients_amount: models::database::loadables::count_ingredients()? as i32,
    })
}

#[tauri::command]
/// Filters the recipes on their name, given a pattern. This command also supports pagination
/// of the results.
pub fn filter_recipes(pattern: String, limit: i32, offset: i32) -> Result<PaginatedRecipe, String> {
    info!("filtering ingredients");
    let data = models::database::loadables::filter_recipes(pattern)?;

    let total = data.len();
    let data = data
        .into_iter()
        .skip(offset as usize)
        .take(limit as usize)
        .collect();

    Ok((data, total, total / limit as usize + 1))
}

#[tauri::command]
/// This commands loads a recipe and all its dependencies into a `CompleteRecipe` structure that
/// Is used in the frontend to display the recipe.
pub fn load_recipe(recipe_id: i32) -> Result<CompleteRecipe, String> {
    info!("Loading recipe {}", recipe_id);

    use models::schema::recipe::dsl::*;

    let base_recipe = models::database::loadables::get_recipe(recipe_id)?;

    Ok(CompleteRecipe {
        id: base_recipe.id,
        name: base_recipe.name,
        cook_time: base_recipe.cook_time,
        prep_time: base_recipe.prep_time,
        yield_: base_recipe.yield_,
        author: None,
        rating: None,
        category: None,
        image: None,
        ingredients: Vec::new(),
    })
}
