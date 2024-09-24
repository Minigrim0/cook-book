use diesel::prelude::*;

use crate::models::*;
use crate::schema::*;

use crate::SharedDatabasePool;

pub fn get_recipe_images(rid: i32, pool: &SharedDatabasePool) -> Result<Vec<Image>, String> {

    let conn = &mut pool.get().unwrap();

    let recipe: Recipe = recipe::table
        .select(Recipe::as_select())
        .find(rid)
        .first(conn).map_err(|e| e.to_string())?;

    let images = RecipeImage::belonging_to(&recipe)
        .inner_join(image_blobs::table)
        .select(Image::as_select())
        .load(conn).map_err(|e| e.to_string())?;

    Ok(images)
}


/// Find an image by its hash and return its id
/// Returns None if the image is not found
pub fn find_image_by_hash(hash: &str, pool: &SharedDatabasePool) -> Option<i32> {
    let conn = &mut pool.get().unwrap();

    let image = image_blobs::table
        .select(Image::as_select())
        .filter(image_blobs::hash.eq(hash))
        .first(conn)
        .ok()?;

    Some(image.id)
}