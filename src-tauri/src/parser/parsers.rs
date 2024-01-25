use regex::Regex;

use crate::database::insertables::{NewCategory, NewIngredient, NewRating, NewAuthor, DBWrapped};


pub fn parse_rating(recipe: &serde_json::Value) -> Option<i32> {
    if let Some(rating) = recipe.get("aggregateRating") {
        let rating = NewRating::new(rating);
        if let Some(id_) = rating.exists() {
            Some(id_)
        } else {
            match rating.save() {
                Ok(id) => {
                    Some(id)
                },
                Err(e) => {
                    println!("Error while creating rating: {}", e.to_string());
                    None
                }
            }
        }
    } else {
        println!("No aggregateRating found");
        None
    }
}

pub fn parse_ingredients(natural_ingredients: &serde_json::Value, recipe_id: i32) {
    // Loop over ingredients
    for ingredient in natural_ingredients.as_array().unwrap().iter() {
        println!("Working on {}", ingredient.as_str().unwrap_or("oops"));
    }
}

pub fn parse_category(recipe: &serde_json::Value) -> Option<i32> {
    let category = NewCategory::new(recipe);
    if let Some(id_) = category.exists() {
        Some(id_)
    } else {
        match category.save() {
            Ok(id) => {
                Some(id)
            },
            Err(e) => {
                println!("Error while creating new category '{}': {}", category.name, e.to_string());
                None
            }
        }
    }
}


// Parses the author information and returns the ID of the author in the database
pub fn parse_author(recipe: &serde_json::Value) -> Option<i32> {
    // Todo: Add support for multiple authors
    if let Some(authors) = recipe.get("author") {
        let author = &authors[0];
        let recipe_author = NewAuthor::new(author);
        if let Some(id_) = recipe_author.exists() {
            Some(id_)
        } else {
            match recipe_author.save() {
                Ok(id) => {
                    Some(id)
                },
                Err(e) => {
                    println!("Error while creating author {}: {}", author["name"], e.to_string());
                    None
                }
            }
        }
    } else {
        None
    }
}
