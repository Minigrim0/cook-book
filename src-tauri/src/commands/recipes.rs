use diesel::prelude::*;
use log::{error, info, warn};
use tauri;

use models::get_connection_pool;
use models::models::{Cuisine, Ingredient, Recipe};
use models::{CompleteRecipe, PaginatedRecipe, RecipeMeta};

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

    let base_recipe = models::database::loadables::get_recipe(recipe_id)?;
    let recipe_autor = match models::database::loadables::get_author(base_recipe.author_id) {
        Ok(author) => Some(author),
        Err(e) => {
            warn!("Unable to load recipe author: {}", e);
            None
        }
    };
    let recipe_rating = match models::database::loadables::get_rating(base_recipe.rating_id) {
        Ok(rating) => Some(rating),
        Err(e) => {
            warn!("Unable to load recipe rating: {}", e);
            None
        }
    };
    let recipe_category = match models::database::loadables::get_category(base_recipe.category_id) {
        Ok(category) => Some(category),
        Err(e) => {
            warn!("Unable to load recipe category: {}", e.to_string());
            None
        }
    };
    let ingredients = match models::database::loadables::load_complete_ingredients(base_recipe.id) {
        Ok(ingredients) => ingredients,
        Err(e) => {
            warn!("Unable to load recipe ingredients: {}", e.to_string());
            Vec::new()
        }
    };

    Ok(CompleteRecipe {
        id: base_recipe.id,
        name: base_recipe.name,
        cook_time: base_recipe.cook_time,
        prep_time: base_recipe.prep_time,
        yield_: base_recipe.yield_,
        author: recipe_autor,
        rating: recipe_rating,
        category: recipe_category,
        image: None,
        ingredients,
    })
}
