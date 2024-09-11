use crate::{models::Ingredient, models::RecipeIngredient, CompleteIngredient};
use diesel::prelude::*;

use super::{get_connection, get_unit};

pub fn count_ingredients() -> Result<usize, String> {
    use crate::schema::ingredient::dsl::*;

    let conn = &mut get_connection()?;

    match ingredient.select(Ingredient::as_select()).load(conn) {
        Ok(d) => Ok(d.len()),
        Err(e) => Err(e.to_string()),
    }
}

pub fn get_ingredient(ingredient_id: i32) -> Result<Ingredient, String> {
    use crate::schema::ingredient::dsl::*;

    let conn = &mut get_connection()?;

    match ingredient
        .select(Ingredient::as_select())
        .find(ingredient_id)
        .first(conn)
        .optional()
    {
        Ok(Some(i)) => Ok(i),
        Ok(None) => Err("Unable to find ingredient".to_string()),
        Err(e) => Err(e.to_string()),
    }
}

pub fn load_complete_ingredients(
    arg_recipe_id: i32,
) -> Result<Vec<Result<CompleteIngredient, String>>, String> {
    use crate::schema::recipe_ingredient::dsl::*;

    let conn = &mut get_connection()?;

    let ri_list = recipe_ingredient
        .select(RecipeIngredient::as_select())
        .filter(recipe_id.eq(arg_recipe_id))
        .load(conn)
        .map_err(|e| e.to_string())?;

    let ingredients: Vec<Result<CompleteIngredient, String>> = ri_list
        .iter()
        .map(|ri| {
            let ingredient = get_ingredient(ri.ingredient_id)?;
            let unit = get_unit(ri.unit_id)?;

            Ok(CompleteIngredient {
                id: ri.id,
                ingredient,
                ingredient_image: None,
                unit,
                amount: ri.amount,
                details: ri.details.clone(),
            })
        })
        .collect();

    Ok(ingredients)
}
