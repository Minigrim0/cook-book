use regex::Regex;

use crate::database::insertables::{DBWrapped, NewIngredient, NewRecipeIngredient, NewUnit};

fn normalize_units(line: &str) -> &str {
    // line.replace(from, to)
    ""
}

pub fn get_or_create_ingredient(name: Option<&str>, unit: Option<&str>) -> Option<i32> {
    let new_ingredient = NewIngredient {
        name: match name {
            Some(name) => if name.to_string() == "" {
                match unit {
                    Some(unit) => unit.to_string(),
                    None => "unknown".to_string(),
                }
            } else {
                name.to_string()
            },
            None => "unknown".to_string(),
        },
    };

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

pub fn parse_natural_ingredient(line: &str) -> Option<NewRecipeIngredient> {
    let basic_re = Regex::new(r"^(?P<amount>\d+\.?\d*) (?:\((?P<alt>.*)\))?\s*(?P<unit>\w+)(?:\s*(?P<name>[.[^,\n]]*))?(?:,(?P<details>.*))?$").unwrap();
    let basic_match = basic_re.captures(line);
    if basic_match.is_none() {
        return None;
    }
    let basic_match = basic_match.unwrap();
    let amount: Option<&str> = basic_match.name("amount").map_or(None, |s| Some(s.as_str()));
    let alt = basic_match.name("alt").map_or(None, |s| Some(s.as_str()));
    let unit = basic_match.name("unit").map_or(None, |s| Some(s.as_str()));
    let name = basic_match.name("name").map_or(None, |s| Some(s.as_str()));
    let details = basic_match.name("details").map_or(None, |s| Some(s.as_str()));

    let new_ingredient = NewIngredient {
        name: match name {
            Some(name) => if name.to_string() == "" {
                match unit {
                    Some(unit) => unit.to_string(),
                    None => "unknown".to_string(),
                }
            } else {
                name.to_string()
            },
            None => "unknown".to_string(),
        },
    };

    let ingredient_id = get_or_create_ingredient(name, unit).unwrap_or(-1);

    let new_unit = NewUnit {
        name: if name.unwrap_or("").to_string() == "" {  // If there is no name, unit is piece
                "piece".to_string()
            } else {
                unit.unwrap_or("unknown").to_string()
            },
    };

    let unit_id = if let Some(id) = new_unit.exists() {
        id
    } else {
        match new_unit.save() {
            Ok(id) => id,
            Err(e) => {
                println!("Error while creating new unit '{}': {}", new_unit.name, e.to_string());
                return None;
            }
        }
    };

    let recipe_ingredient = NewRecipeIngredient {
        recipe_id: -1,
        ingredient_id,
        unit_id,
        amount: match amount {
            Some(amount) => amount.parse::<f32>().unwrap(),
            None => 1.0,
        },
        // alt: alt.map_or(None, |s| Some(s.to_string())),
        details: details.map_or(None, |s| Some(s.to_string())),
        full: Some(line.to_string()),
    };

    Some(recipe_ingredient)
}
