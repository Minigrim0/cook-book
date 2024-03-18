//! This module contains the functions for parsing JSON files into the database.
use std::path::PathBuf;

use crate::database::{insertables::recipe::NewRecipe, wrapper::DBWrapped};

use crate::parser::parsers::{
    parse_author,
    parse_rating,
    parse_category,
    parse_ingredients,
    parse_instructions,
};

pub fn parse_recipe(path: &PathBuf) -> Result<String, String> {
    // Open the file recipe.json and load it
    let file_path = path.join(r"recipe.json");
    let file = match std::fs::File::open(file_path.clone()) {
        Ok(file) => file,
        Err(err) => return Err(err.to_string())
    };
    let recipe_json: serde_json::Value = match serde_json::from_reader(file) {
        Ok(recipe_v) => recipe_v,
        Err(err) => {
            return Err(format!("Error while parsing {}: {}", file_path.to_str().unwrap(), err.to_string()));
        }
    };

    let author_id: Option<i32> = parse_author(&recipe_json);
    let rating_id: Option<i32> = parse_rating(&recipe_json);
    let cat_id: Option<i32> = parse_category(&recipe_json);

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

    let recipe_id = if let Some(id) = recipe.exists() {
        id
    } else {
        match recipe.save() {
            Ok(id) => {
                id
            },
            Err(e) => {
                return Err(format!("Unable to save recipe {}; {}", recipe.name, e.to_string()));
            }
        }
    };

    // Parse the ingredients
    if let Some(ingredients) = recipe_json.get("recipeIngredient") {
        parse_ingredients(ingredients, recipe_id);
    } else {
        return Err("Unable to parse ingredients, no key 'recipeIngredient' found.".to_string());
    }

    // Parse the steps
    if let Some(steps) = recipe_json.get("recipeInstructions") {
        parse_instructions(steps, recipe_id);
    } else {
        return Err("Unable to parse instructions, no key 'recipeInstructions' found.".to_string());
    }

    Ok(file_path.to_str().unwrap().to_string())
}
