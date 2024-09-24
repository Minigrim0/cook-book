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
