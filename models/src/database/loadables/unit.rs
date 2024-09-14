use diesel::prelude::*;

use crate::database::SharedDatabasePool;
use crate::models::Unit;

pub fn get_unit(unit_id: i32, pool: &SharedDatabasePool) -> Result<Unit, String> {
    use crate::schema::unit::dsl::*;

    let conn = &mut pool.get().map_err(|e| e.to_string())?;

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
