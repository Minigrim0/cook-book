//! This module contains the functions for parsing JSON files into the database.
use std::fs::DirEntry;

use crate::database::insertables::{NewRecipe, NewAuthor, DBWrapped};

// Parses the author information and returns the ID of the author in the database
fn parse_author(recipe: &serde_json::Value) -> Option<i32> {
    if let Some(authors) = recipe.get("author") {
        let author = authors[0].clone();
        let recipe_author = NewAuthor {
            type_: author["@type"].as_str().unwrap_or("unknown").to_string(),
            name: author["name"].as_str().unwrap_or("unknown").to_string(),
            url: author["url"].as_str().unwrap_or("unknown").to_string()
        };
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

    let mut author_id: Option<i32> = parse_author(&recipe);


    Ok(file_path.to_str().unwrap().to_string())
}
