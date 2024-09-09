use diesel::prelude::*;
use crate::models::Ingredient;

pub fn count_ingredients() -> Result<usize, String> {
    use crate::schema::ingredient::dsl::*;

    let conn = &mut super::get_connection()?;

    match ingredient.select(Ingredient::as_select()).load(conn) {
        Ok(d) => Ok(d.len()),
        Err(e) => Err(e.to_string())
    }
}
