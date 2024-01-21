//! This module contains the functions for parsing JSON files into the database.
use serde_json::json;
use diesel::sqlite::SqliteConnection;
use std::fs::DirEntry;

use crate::database::insertables::{NewRecipe, NewAuthor, DBWrapped};
use crate::schema::recipe;


pub fn parse_recipe(path: &DirEntry) -> Result<String, String> {
    // Open the file recipe.json and load it
    let file = match std::fs::File::open(path.path().join(r"recipe.json")) {
        Ok(file) => file,
        Err(err) => return Err(err.to_string())
    };
    println!("Working on {}", path.path().to_str().unwrap());
    let recipe: serde_json::Value = serde_json::from_reader(file).unwrap();

    // Save the image path to the database
    if let Some(author) = recipe.get("author") {
        let recipe_author = NewAuthor {
            type_: author["type"].as_str().unwrap_or("unknown").to_string(),
            name: author["name"].as_str().unwrap_or("unknown").to_string(),
            url: author["url"].as_str().unwrap_or("unknown").to_string()
        };
        if !recipe_author.exists() {
            println!("Author {} does not exist, creating entry", recipe_author.name);
            match recipe_author.save() {
                Ok(id) => println!("New author saved with id {}", id),
                Err(e) => println!("An error occured while saving the author: {}", e.to_string())
            }
        }
    };

    Ok("".to_string())
}
