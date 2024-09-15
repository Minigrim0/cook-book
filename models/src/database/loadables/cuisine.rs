use diesel::prelude::*;
use log::{info, warn};

use crate::models::Cuisine;
use crate::database::SharedDatabasePool;

/// Returns the list of all cuisines
pub fn load_cuisines(pool: &SharedDatabasePool) -> Result<Vec<Cuisine>, String> {
    warn!("Load cuisine not implemented");
    Ok(Vec::new())
}

pub fn count_cuisines(pool: &SharedDatabasePool) -> Result<usize, String> {
    info!("Counting cuisines");
    use crate::schema::cuisine::dsl::*;

    let conn = &mut pool.get().map_err(|e| e.to_string())?;

    match cuisine.select(Cuisine::as_select()).load(conn) {
        Ok(data) => Ok(data.len()),
        Err(e) => {
            warn!("Unable to load cuisine: {}", e.to_string());
            Err(e.to_string())
        }
    }
}
