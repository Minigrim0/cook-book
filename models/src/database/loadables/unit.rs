use diesel::prelude::*;

use super::get_connection;
use crate::models::Unit;

pub fn get_unit(unit_id: i32) -> Result<Unit, String> {
    use crate::schema::unit::dsl::*;

    let conn = &mut get_connection()?;

    match unit
        .select(Unit::as_select())
        .find(unit_id)
        .first(conn)
        .optional()
    {
        Ok(Some(u)) => Ok(u),
        Ok(None) => Err("Unable to find unit".to_string()),
        Err(e) => Err(e.to_string()),
    }
}
