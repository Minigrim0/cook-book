use diesel::prelude::*;
use log::info;

use super::get_connection;
use crate::models::Category;

/// Returns a category based on its id. If no category with the given id is found, returns an error
pub fn get_category(category_id: i32) -> Result<Category, String> {
    info!("Loading category {}", category_id);
    let conn = &mut get_connection()?;

    use crate::schema::category::dsl::*;

    match category
        .select(Category::as_select())
        .find(category_id)
        .first(conn)
        .optional()
    {
        Ok(Some(c)) => Ok(c),
        Ok(None) => Err("No categry exists with the given id".to_string()),
        Err(e) => Err(e.to_string()),
    }
}
