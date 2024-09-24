//! This module contains the functions for parsing JSON files into the database.
use std::path::PathBuf;

use models::insertables::{NewRecipe, DBWrapped};
use models::SharedDatabasePool;

use crate::parser::parsers::{
    parse_author, parse_category, parse_ingredients, parse_instructions, parse_rating,
};

/// Loads images from the folder of the recipes and saves them to the database
fn load_images(recipe_id: i32, recipe_path: &PathBuf, pool: SharedDatabasePool) -> Result<(), String> {
    use std::fs;
    use models::insertables::{NewImage, NewRecipeImage};
    use sha2::{Sha256, Digest};

    // Get all files in the recipe folder
    let entries = fs::read_dir(recipe_path).map_err(|e| e.to_string())?;

    for entry in entries {
        let entry = entry.map_err(|e| e.to_string())?;
        let path = entry.path();

        // Check if the file is an image (you might want to add more image extensions)
        if let Some(extension) = path.extension() {
            if extension == "jpg" || extension == "jpeg" || extension == "png" {
                // Read the image file
                let image_data = fs::read(&path).map_err(|e| e.to_string())?;

                // Calculate hash
                let mut hasher = Sha256::new();
                hasher.update(&image_data);
                let hash = format!("{:x}", hasher.finalize());

                if let Some(existing_image) = models::database::loadables::find_image_by_hash(&hash, &pool) {
                    log::info!("Image with hash {} already exists: {}", hash, existing_image);

                    // Attach the image to the recipe
                    let new_recipe_image = NewRecipeImage {
                        recipe_id: recipe_id,
                        image_id: existing_image,
                    };

                    if let Err(e) = new_recipe_image.save(&pool) {
                        log::error!("Failed to associate image with recipe: {}", e);
                    }
                    continue;
                }
                log::info!("Image with hash {} does not exist", hash);

                // Create new image
                let new_image = NewImage {
                    image_data: image_data,
                    hash: hash,
                };

                // Save image and get its ID
                let image_id = match new_image.save(&pool) {
                    Ok(id) => id,
                    Err(e) => {
                        log::error!("Failed to save image: {}", e);
                        continue;
                    }
                };

                // Create recipe-image association
                let new_recipe_image = NewRecipeImage {
                    recipe_id: recipe_id,
                    image_id: image_id,
                };

                // Save recipe-image association
                if let Err(e) = new_recipe_image.save(&pool) {
                    log::error!("Failed to associate image with recipe: {}", e);
                }
            }
        }
    }
    
    Ok(())
}

pub fn parse_recipe(path: &PathBuf, pool: SharedDatabasePool) -> Result<String, String> {
    // Open the file recipe.json and load it
    let file_path = path.join(r"recipe.json");
    let file = match std::fs::File::open(file_path.clone()) {
        Ok(file) => file,
        Err(err) => return Err(err.to_string()),
    };

    let recipe_json: serde_json::Value = match serde_json::from_reader(file) {
        Ok(recipe_v) => recipe_v,
        Err(err) => {
            return Err(format!(
                "Error while parsing {}: {}",
                file_path.to_str().unwrap(),
                err.to_string()
            ));
        }
    };

    let author_id: Option<i32> = parse_author(&recipe_json, &pool);
    let rating_id: Option<i32> = parse_rating(&recipe_json, &pool);
    let cat_id: Option<i32> = parse_category(&recipe_json, &pool);

    let mut recipe = NewRecipe::new(&recipe_json);
    if let Some(id) = author_id {
        recipe.author_id = id;
    }
    if let Some(id) = rating_id {
        recipe.rating_id = id;
    }
    if let Some(id) = cat_id {
        recipe.category_id = id;
    }

    let recipe_id = if let Some(id) = recipe.exists(&pool) {
        id
    } else {
        match recipe.save(&pool) {
            Ok(id) => id,
            Err(e) => {
                return Err(format!(
                    "Unable to save recipe {}; {}",
                    recipe.name,
                    e.to_string()
                ));
            }
        }
    };

    // Parse the ingredients
    if let Some(ingredients) = recipe_json.get("recipeIngredient") {
        parse_ingredients(ingredients, recipe_id, &pool);
    } else {
        return Err("Unable to parse ingredients, no key 'recipeIngredient' found.".to_string());
    }

    // Parse the steps
    if let Some(steps) = recipe_json.get("recipeInstructions") {
        parse_instructions(steps, recipe_id, &pool);
    } else {
        return Err("Unable to parse instructions, no key 'recipeInstructions' found.".to_string());
    }

    // Try to find images
    if let Err(e) = load_images(recipe_id, path, pool) {
        log::error!("Unable to load images for recipe {}: {}", recipe.name, e);
    }

    Ok(file_path.to_str().unwrap().to_string())
}
