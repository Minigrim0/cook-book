use yew::Callback;
use log::{error, trace};
use models::{PaginatedIngredients, PaginatedRecipe, CompleteRecipe};
use wasm_bindgen_futures::spawn_local;

use crate::glue;

/// Spawns a future on the yew runtime to filter the list of ingredients from the backend
pub fn filter_ingredients(pattern: String, page: i32, callback: Callback<Result<PaginatedIngredients, String>>) {
    spawn_local(async move {
        match glue::filter_ingredients(pattern, 24, page * 24).await {
            Ok(response) => {
                let data: Result<PaginatedIngredients, String> = serde_wasm_bindgen::from_value(response)
                    .map_err(|e| format!("Conversion error: {}", e.to_string()));
                callback.emit(data);
            }
            Err(e) => {
                callback.emit(Err(serde_wasm_bindgen::from_value(e).unwrap()));
            }
        }
    });
}


/// Spawns a future on the yew runtime to filter the list of recipes from the backend
pub fn filter_recipes(pattern: String, page: i32, callback: Callback<Result<PaginatedRecipe, String>>) {
    spawn_local(async move {
        match glue::filter_recipes(pattern, 24, page * 24).await {
            Ok(response) => {
                trace!("Successfully loaded recipe list, converting");
                let data: Result<PaginatedRecipe, String> = serde_wasm_bindgen::from_value(response)
                    .map_err(|e| format!("Conversion error: {}", e.to_string()));
                callback.emit(data);
            },
            Err(e) => callback.emit(Err(serde_wasm_bindgen::from_value(e).unwrap()))
        }
    });
}

/// Spawns a future on the yew runtime to load a recipe from the backend
pub fn load_recipe(recipe_id: i32, callback: Callback<Result<CompleteRecipe, String>>) {
    spawn_local(async move {
        match glue::load_recipe(recipe_id).await {
            Ok(response) => {
                trace!("Successfully loaded recipe, converting");
                let data: Result<CompleteRecipe, String> = serde_wasm_bindgen::from_value(response)
                    .map_err(|e| format!("Conversion error: {}", e.to_string()));
                callback.emit(data);
            },
            Err(e) => {
                error!("Error calling load_recipe: {:?}", e);
                callback.emit(Err(serde_wasm_bindgen::from_value(e).unwrap()));
            }
        }
    });
}

/// Spawns a future on the yew runtime to load a recipe image from the backend
pub fn get_recipe_image(recipe_id: i32, callback: Callback<(i32, Result<String, String>)>) {
    spawn_local(async move {
        match glue::get_recipe_image(recipe_id).await {
            Ok(response) => callback.emit((recipe_id, Ok(serde_wasm_bindgen::from_value(response).unwrap()))),
            Err(e) => callback.emit((recipe_id, Err(serde_wasm_bindgen::from_value(e).unwrap()))),
        };
    });
}