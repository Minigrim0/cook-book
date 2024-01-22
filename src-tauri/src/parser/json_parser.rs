//! This module contains the functions for parsing JSON files into the database.
use std::fs::DirEntry;

use crate::database::insertables::{NewRecipe, NewAuthor, DBWrapped, NewRating};

// Parses the author information and returns the ID of the author in the database
fn parse_author(recipe: &serde_json::Value) -> Option<i32> {
    // Todo: Add support for multiple authors
    if let Some(authors) = recipe.get("author") {
        let author = &authors[0];
        let recipe_author = NewAuthor::new(author);
        if let Some(id_) = recipe_author.exists() {
            println!("Author {} exists", author["name"]);
            Some(id_)
        } else {
            match recipe_author.save() {
                Ok(id) => {
                    println!("New author {} saved with id {}", author["name"], id);
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

fn parse_rating(recipe: &serde_json::Value) -> Option<i32> {
    if let Some(rating) = recipe.get("aggregateRating") {
        let rating = NewRating::new(rating);
        println!("Working on rating {:?}", rating);
        if let Some(id_) = rating.exists() {
            println!("Rating exists");
            Some(id_)
        } else {
            match rating.save() {
                Ok(id) => {
                    println!("New rating saved with id {}", id);
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

fn parse_category(recipe: &serde_json::Value) -> Option<i32> {
    None
}

pub fn parse_recipe(path: &DirEntry) -> Result<String, String> {
    // Open the file recipe.json and load it
    let file_path = path.path().join(r"recipe.json");
    let file = match std::fs::File::open(file_path.clone()) {
        Ok(file) => file,
        Err(err) => return Err(err.to_string())
    };
    println!("Working on {}", file_path.to_str().unwrap());
    let recipe: serde_json::Value = match serde_json::from_reader(file) {
        Ok(recipe_v) => recipe_v,
        Err(err) => {
            return Err(format!("Error while parsing {}: {}", file_path.to_str().unwrap(), err.to_string()));
        }
    };

    let author_id: Option<i32> = parse_author(&recipe);
    let rating_id: Option<i32> = parse_rating(&recipe);
    let cat_id: Option<i32> = parse_category(&recipe);

    let mut recipe = NewRecipe::new(&recipe);
    if let Some(id) = author_id {
        recipe.author_id = id;
    }
    if let Some(id) = rating_id {
        recipe.rating_id = id;
    }
    if let Some(id) = cat_id {
        recipe.category_id = id;
    }

    Ok(file_path.to_str().unwrap().to_string())
}
