use diesel::prelude::*;

use crate::database::SharedDatabasePool;
use crate::models::Step;

pub fn get_recipe_steps(r_id: i32, pool: &SharedDatabasePool) -> Result<Vec<Step>, String> {
    use crate::schema::step::dsl::*;

    let conn = &mut pool.get().map_err(|e| e.to_string())?;

    step.select(Step::as_select())
        .filter(recipe_id.eq(r_id))
        .order_by(number)
        .load(conn)
        .map_err(|e| e.to_string())
}
