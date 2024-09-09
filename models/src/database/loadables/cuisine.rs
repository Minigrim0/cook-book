use diesel::prelude::*;
use log::{info, warn};
use crate::models::Cuisine;

/// Returns the list of all cuisines
pub fn load_cuisines() -> Vec<Cuisine> {
    warn!("Load cuisine not implemented");
    Vec::new()
}


pub fn count_cuisines() -> Result<usize, String> {
    info!("Counting cuisines");
    use crate::schema::cuisine::dsl::*;

    let conn = &mut super::get_connection()?;

    match cuisine.select(Cuisine::as_select()).load(conn) {
        Ok(data) => Ok(data.len()),
        Err(e) => {
            warn!("Unable to load cuisine: {}", e.to_string());
            Err(e.to_string())
        }
    }
}
