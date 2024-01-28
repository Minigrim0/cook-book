use serde_json::json;
use regex::Regex;

use crate::database::insertables::{DBWrapped, NewIngredient, NewRecipeIngredient, NewUnit};

fn normalize_units(line: &str) -> &str {
    // line.replace(from, to)
    ""
}

pub fn get_or_create_ingredient(name: Option<&str>, unit: Option<&str>) -> Option<i32> {
    let new_ingredient = NewIngredient::new(&json!({
        "unit": unit,
        "name": name
    }));

    if let Some(id) = new_ingredient.exists() {
        Some(id)
    } else {
        match new_ingredient.save() {
            Ok(id) => Some(id),
            Err(e) => {
                println!("Error while creating new ingredient '{}': {}", new_ingredient.name, e.to_string());
                return None;
            }
        }
    }
}

fn get_or_create_unit(name: Option<&str>, unit: Option<&str>) -> Option<i32> {
    let new_unit = NewUnit::new(&json!({
        "name": name.unwrap_or(""),
        "unit": unit.unwrap_or("")
    }));

    if let Some(id) = new_unit.exists() {
        Some(id)
    } else {
        match new_unit.save() {
            Ok(id) => Some(id),
            Err(e) => {
                println!("Error while creating new unit '{}': {}", new_unit.name, e.to_string());
                return None;
            }
        }
    }
}

pub fn parse_natural_ingredient(line: &str, r_id: i32) -> Option<NewRecipeIngredient> {
    let basic_re = Regex::new(r"^(?P<amount>\d+\.?\d*) (?:\((?P<alt>.*)\))?\s*(?P<unit>\w+)(?:\s*(?P<name>[.[^,\n]]*))?(?:,(?P<details>.*))?$").unwrap();
    let basic_match = basic_re.captures(line);
    if basic_match.is_none() {
        return None;
    }

    let basic_match = basic_match.unwrap();
    let amount: Option<&str> = basic_match.name("amount").map_or(None, |s| Some(s.as_str()));
    let _alt = basic_match.name("alt").map_or(None, |s| Some(s.as_str()));
    let unit = basic_match.name("unit").map_or(None, |s| Some(s.as_str()));
    let name = basic_match.name("name").map_or(None, |s| Some(s.as_str()));
    let details = basic_match.name("details").map_or(None, |s| Some(s.as_str()));

    let ingredient_id = get_or_create_ingredient(name, unit).unwrap_or(-1);
    let unit_id = get_or_create_unit(name, unit).unwrap_or(-1);

    let recipe_ingredient = NewRecipeIngredient::new(&json!({
        "r_id": r_id,
        "i_id": ingredient_id,
        "u_id": unit_id,
        "amount": amount,
        // alt: alt.map_or(None, |s| Some(s.to_string())),
        "details": details,
        "full": line.to_string(),
    }));

    Some(recipe_ingredient)
}
