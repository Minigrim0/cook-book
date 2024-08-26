use log::error;
use regex::Regex;
use serde_json::json;

use crate::database::insertables::DBWrapped;
use crate::SharedDatabasePool;

use crate::database::insertables::{
    ingredient::NewIngredient, recipe_ingredient::NewRecipeIngredient, unit::NewUnit,
};

fn normalize_units(line: &str) -> String {
    let mut new_line = line.replace("tbsp", "tablespoon");
    new_line = new_line.replace("tsp", "teaspoon");

    // TODO: remove plurals

    new_line
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
    let basic_re = Regex::new(r"^(?P<amount>\d+\.?\d*) (?:\((?P<alt>.*)\))?\s*(?P<unit>\w+)(?:\s*(?P<name>[.[^,\n]]*))?(?:,(?P<details>.*))?$").unwrap();
    let new_line = normalize_units(line);
    let basic_match = basic_re.captures(new_line.as_str());
    if basic_match.is_none() {
        // TODO: create 'error' models with all data to ask user later

        return None;
    }

    let basic_match = basic_match.unwrap();
    let amount: Option<&str> = basic_match
        .name("amount")
        .map_or(None, |s| Some(s.as_str()));
    let _alt = basic_match.name("alt").map_or(None, |s| Some(s.as_str()));
    let unit = basic_match.name("unit").map_or(None, |s| Some(s.as_str()));
    let name = basic_match.name("name").map_or(None, |s| Some(s.as_str()));
    let details = basic_match
        .name("details")
        .map_or(None, |s| Some(s.as_str()));

    let ingredient_id = get_or_create_ingredient(name, unit, pool).unwrap_or(-1);
    let unit_id = get_or_create_unit(name, unit, pool).unwrap_or(-1);

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
