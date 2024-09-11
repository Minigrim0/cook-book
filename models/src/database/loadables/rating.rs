use diesel::prelude::*;
use log::info;

use super::get_connection;
use crate::models::Rating;

pub fn get_rating(rating_id: i32) -> Result<Rating, String> {
    info!("Loading rating {}", rating_id);
    use crate::schema::rating::dsl::*;

    let conn = &mut get_connection()?;

    match rating
        .select(Rating::as_select())
        .find(rating_id)
        .first(conn)
        .optional()
    {
        Ok(Some(r)) => Ok(r),
        Ok(None) => Err("No rating exists with the given id".to_string()),
        Err(e) => Err(e.to_string()),
    }
}