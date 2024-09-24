use crate::{models::Ingredient, models::RecipeIngredient, CompleteIngredient};
use diesel::prelude::*;
use log::info;

use crate::database::SharedDatabasePool;
use super::get_unit;

pub fn count_ingredients(pool: &SharedDatabasePool) -> Result<usize, String> {
    info!("Counting ingredients");
    use crate::schema::ingredient::dsl::*;

    let conn = &mut pool.get().map_err(|e| e.to_string())?;

    match ingredient.select(Ingredient::as_select()).load(conn) {
        Ok(d) => Ok(d.len()),
        Err(e) => Err(e.to_string()),
    }
}

pub fn get_ingredient(ingredient_id: i32, pool: &SharedDatabasePool) -> Result<Ingredient, String> {
    info!("Loading ingredient {}", ingredient_id);
    use crate::schema::ingredient::dsl::*;

    let conn = &mut pool.get().map_err(|e| e.to_string())?;

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
    pool: &SharedDatabasePool,
) -> Result<Vec<Result<CompleteIngredient, String>>, String> {
    info!("Loading complete ingredients for recipe {}", arg_recipe_id);
    use crate::schema::recipe_ingredient::dsl::*;

    // Enclose scope to free the connection
    let ri_list = {
        let conn = &mut pool.get().map_err(|e| e.to_string())?;

        recipe_ingredient
            .select(RecipeIngredient::as_select())
            .filter(recipe_id.eq(arg_recipe_id))
            .load(conn)
            .map_err(|e| e.to_string())
    }?;

    let ingredients: Vec<Result<CompleteIngredient, String>> = ri_list
        .iter()
        .map(|ri| {
            let ingredient = get_ingredient(ri.ingredient_id, pool)?;
            let unit = get_unit(ri.unit_id, pool)?;

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
