use log::{info, warn};
use tauri;
use base64::engine::general_purpose::STANDARD;
use base64::engine::Engine;

use models::{CompleteRecipe, PaginatedRecipe, RecipeMeta};
use models::SharedDatabasePool;

#[tauri::command]
/// Loads various metadata about the recipes like the amount of recipes in the
/// database, the amount of cuisines, ingredients, ...
pub fn recipe_meta(state: tauri::State<SharedDatabasePool>) -> Result<RecipeMeta, String> {
    info!("getting recipe meta");

    Ok(RecipeMeta {
        recipe_amount: models::database::loadables::count_recipes(&state)? as i32,
        cuisine_amount: models::database::loadables::count_cuisines(&state)? as i32,
        ingredients_amount: models::database::loadables::count_ingredients(&state)? as i32,
    })
}

#[tauri::command]
pub fn get_recipe_image(state: tauri::State<SharedDatabasePool>, recipe_id: i32) -> Result<String, String> {
    info!("getting recipe image for recipe {}", recipe_id);

    let image = models::database::loadables::get_recipe_images(recipe_id, &state)?;

    let image = image.first().ok_or("No image found")?;
    let encoded_image = STANDARD.encode(&image.image_data);
    Ok(encoded_image)
}



/// Filters the recipes on their name, given a pattern. This command also supports pagination
#[tauri::command]
/// Filters the recipes on their name, given a pattern. This command also supports pagination
/// of the results.
pub fn filter_recipes(state: tauri::State<SharedDatabasePool>, pattern: String, limit: i32, offset: i32) -> Result<PaginatedRecipe, String> {
    info!("filtering ingredients");
    let data = models::database::loadables::filter_recipes(pattern, &state)?;

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
pub fn load_recipe(state: tauri::State<SharedDatabasePool>, recipe_id: i32) -> Result<CompleteRecipe, String> {
    info!("Loading recipe {}", recipe_id);

    let base_recipe = models::database::loadables::get_recipe(recipe_id, &state)?;
    let recipe_autor = match models::database::loadables::get_author(base_recipe.author_id, &state) {
        Ok(author) => Some(author),
        Err(e) => {
            warn!("Unable to load recipe author: {}", e);
            None
        }
    };
    let recipe_rating = match models::database::loadables::get_rating(base_recipe.rating_id, &state) {
        Ok(rating) => Some(rating),
        Err(e) => {
            warn!("Unable to load recipe rating: {}", e);
            None
        }
    };
    let recipe_category = match models::database::loadables::get_category(base_recipe.category_id, &state) {
        Ok(category) => Some(category),
        Err(e) => {
            warn!("Unable to load recipe category: {}", e.to_string());
            None
        }
    };
    let ingredients = match models::database::loadables::load_complete_ingredients(base_recipe.id, &state) {
        Ok(ingredients) => ingredients,
        Err(e) => {
            warn!("Unable to load recipe ingredients: {}", e.to_string());
            Vec::new()
        }
    };
    let steps = match models::database::loadables::get_recipe_steps(base_recipe.id, &state) {
        Ok(steps) => steps,
        Err(e) => {
            warn!("Unable to load recipe steps: {}", e.to_string());
            Vec::new()
        }
    };

    let images = match models::database::loadables::get_recipe_images(base_recipe.id, &state) {
        Ok(images) => images,
        Err(e) => {
            warn!("Unable to load recipe images: {}", e.to_string());
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
        steps,
        images: images,
    })
}
