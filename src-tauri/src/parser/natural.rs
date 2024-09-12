use log::error;
use regex::Regex;
use serde_json::json;
use lazy_static::lazy_static;

use models::SharedDatabasePool;

use models::insertables::{
    ingredient::NewIngredient, recipe_ingredient::NewRecipeIngredient, unit::NewUnit, DBWrapped,
};

fn normalize_units(line: &str) -> String {
    let mut new_line = line.replace("tbsp", "tablespoon");
    new_line = new_line.replace("tsp", "teaspoon");

    // TODO: remove plurals

    new_line
}

lazy_static! {
    static ref INGREDIENT_REGEX: Regex = Regex::new(
        r"^((?:\d+(?:\.\d+)?|\d+\/\d+)\s*(?:-\s*(?:\d+(?:\.\d+)?|\d+\/\d+)\s*)?)?(\w+(?:\s+\w+)?)?\s*(.+?)(?:,\s*(.+))?$"
    ).unwrap();
}

#[derive(Debug)]
struct ParsedIngredient {
    original: String,
    amount: Option<f32>,
    unit: Option<String>,
    name: Option<String>,
    details: Option<String>,
}

fn parse_ingredient(ingredient: &str) -> ParsedIngredient {
    let ingredient = ingredient.trim();
    
    if let Some(captures) = INGREDIENT_REGEX.captures(ingredient) {
        ParsedIngredient {
            original: ingredient.to_string(),
            amount: captures.get(1).and_then(|m| parse_amount(m.as_str())),
            unit: captures.get(2).map(|m| m.as_str().to_lowercase()),
            name: captures.get(3).map(|m| m.as_str().trim().to_string()),
            details: captures.get(4).map(|m| m.as_str().trim().to_string()),
        }
    } else {
        ParsedIngredient {
            original: ingredient.to_string(),
            amount: None,
            unit: None,
            name: Some(ingredient.to_string()),
            details: None,
        }
    }
}

fn parse_amount(amount: &str) -> Option<f32> {
    if amount.contains('-') {
        let parts: Vec<&str> = amount.split('-').collect();
        if parts.len() == 2 {
            let min = parse_fraction(parts[0]);
            let max = parse_fraction(parts[1]);
            Some((min + max) / 2.0)
        } else {
            None
        }
    } else {
        Some(parse_fraction(amount))
    }
}

fn parse_fraction(fraction: &str) -> f32 {
    if fraction.contains('/') {
        let parts: Vec<&str> = fraction.split('/').collect();
        if parts.len() == 2 {
            let numerator: f32 = parts[0].trim().parse().unwrap_or(0.0);
            let denominator: f32 = parts[1].trim().parse().unwrap_or(1.0);
            numerator / denominator
        } else {
            0.0
        }
    } else {
        fraction.trim().parse().unwrap_or(0.0)
    }
}

pub fn get_or_create_ingredient(
    name: Option<&str>,
    unit: Option<&str>,
    pool: &SharedDatabasePool,
) -> Option<i32> {
    let new_ingredient = NewIngredient::new(&json!({
        "unit": unit,
        "name": name
    }));

    if let Some(id) = new_ingredient.exists(pool) {
        Some(id)
    } else {
        match new_ingredient.save(pool) {
            Ok(id) => Some(id),
            Err(e) => {
                error!(
                    "Error while creating new ingredient '{}': {}",
                    new_ingredient.name,
                    e.to_string()
                );
                return None;
            }
        }
    }
}

fn get_or_create_unit(
    name: Option<&str>,
    unit: Option<&str>,
    pool: &SharedDatabasePool,
) -> Option<i32> {
    let new_unit = NewUnit::new(&json!({
        "name": name.unwrap_or(""),
        "unit": unit.unwrap_or("")
    }));

    if let Some(id) = new_unit.exists(pool) {
        Some(id)
    } else {
        match new_unit.save(pool) {
            Ok(id) => Some(id),
            Err(e) => {
                error!(
                    "Error while creating new unit '{}': {}",
                    new_unit.name,
                    e.to_string()
                );
                return None;
            }
        }
    }
}

pub fn parse_natural_ingredient(
    line: &str,
    r_id: i32,
    pool: &SharedDatabasePool,
) -> Option<NewRecipeIngredient> {
    let parsed = parse_ingredient(line);
    
    let ingredient_id = get_or_create_ingredient(parsed.name.as_deref(), parsed.unit.as_deref(), pool).unwrap_or(-1);
    let unit_id = get_or_create_unit(parsed.name.as_deref(), parsed.unit.as_deref(), pool).unwrap_or(-1);

    let recipe_ingredient = NewRecipeIngredient::new(&json!({
        "r_id": r_id,
        "i_id": ingredient_id,
        "u_id": unit_id,
        "amount": parsed.amount,
        "details": parsed.details,
        "full": parsed.original,
    }));

    Some(recipe_ingredient)
}
