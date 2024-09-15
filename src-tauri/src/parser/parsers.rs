use log::{error, warn};
use serde_json::json;

use models::SharedDatabasePool;

use models::insertables::{
    NewAuthor, NewCategory, NewRating, NewStep, DBWrapped,
};

use super::natural::parse_natural_ingredient;

pub fn parse_rating(recipe: &serde_json::Value, pool: &SharedDatabasePool) -> Option<i32> {
    if let Some(rating) = recipe.get("aggregateRating") {
        let rating = NewRating::new(rating);
        if let Some(id_) = rating.exists(&pool) {
            Some(id_)
        } else {
            match rating.save(&pool) {
                Ok(id) => Some(id),
                Err(e) => {
                    error!("Error while creating rating: {}", e.to_string());
                    None
                }
            }
        }
    } else {
        warn!("No aggregateRating found");
        None
    }
}

pub fn parse_category(recipe: &serde_json::Value, pool: &SharedDatabasePool) -> Option<i32> {
    let category = NewCategory::new(recipe);
    if let Some(id_) = category.exists(&pool) {
        Some(id_)
    } else {
        category.save(&pool).ok()
    }
}

// Parses the author information and returns the ID of the author in the database
pub fn parse_author(recipe: &serde_json::Value, pool: &SharedDatabasePool) -> Option<i32> {
    // Todo: Add support for multiple authors
    if let Some(authors) = recipe.get("author") {
        let author = &authors[0];
        let recipe_author = NewAuthor::new(author);
        recipe_author
            .exists(&pool)
            .or_else(|| recipe_author.save(&pool).ok())
    } else {
        None
    }
}

pub fn parse_ingredients(
    natural_ingredients: &serde_json::Value,
    recipe_id: i32,
    pool: &SharedDatabasePool,
) {
    // Loop over ingredients
    for ingredient in natural_ingredients.as_array().unwrap().iter() {
        if let Some(recipe_ingredient) =
            parse_natural_ingredient(ingredient.as_str().unwrap_or(""), recipe_id, &pool)
        {
            if let Err(e) = recipe_ingredient.save(&pool) {
                error!("Error while creating recipe ingredient: {}", e.to_string());
            }
            // All went fine
        } else {
            error!(
                "Unable to extract ingredients from {}",
                ingredient.to_string()
            );
        }
    }
}

pub fn parse_instructions(
    instructions: &serde_json::Value,
    recipe_id: i32,
    pool: &SharedDatabasePool,
) {
    let mut step_number = 0;
    for instruction in instructions.as_array().unwrap().iter() {
        let step = NewStep::new(&json!({
            "r_id": recipe_id,
            "step": step_number,
            "data": instruction
        }));
        step_number += 1;
        if let Err(e) = step.save(pool) {
            error!("Error while creating step: {}", e.to_string());
        }
    }
}
