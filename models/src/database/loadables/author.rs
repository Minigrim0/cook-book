use diesel::prelude::*;
use log::info;

use super::get_connection;
use crate::models::Author;

pub fn get_author(author_id: i32) -> Result<Author, String> {
    info!("Loading author {}", author_id);
    let conn = &mut get_connection()?;

    use crate::schema::author::dsl::*;
    match author
        .select(Author::as_select())
        .find(author_id)
        .first(conn)
        .optional()
    {
        Ok(Some(a)) => Ok(a),
        Ok(None) => Err("No author exists with the given id".to_string()),
        Err(e) => Err(e.to_string()),
    }
}
