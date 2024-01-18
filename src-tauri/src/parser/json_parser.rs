//! This module contains the functions for parsing JSON files into the database.
use diesel::sqlite::SqliteConnection;
use std::fs::DirEntry;

use crate::database::insertables::NewRecipe;
use crate::schema::recipe;


pub fn parse_recipe(path: DirEntry) -> Result<String, String> {
    Ok("".to_string())
}
