use diesel::prelude::*;

use super::get_connection;
use crate::models::Step;

pub fn get_recipe_steps(r_id: i32) -> Result<Vec<Step>, String> {
    use crate::schema::step::dsl::*;

    let conn = &mut get_connection()?;

    step.select(Step::as_select())
        .filter(recipe_id.eq(r_id))
        .order_by(number)
        .load(conn)
        .map_err(|e| e.to_string())
}
