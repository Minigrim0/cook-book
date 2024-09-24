use diesel::prelude::*;

use crate::database::SharedDatabasePool;
use crate::insertables::DBWrapped;
use crate::models::RecipeImage;

#[derive(Insertable)]
#[diesel(table_name = crate::database::schema::recipe_images)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct NewRecipeImage {
    pub recipe_id: i32,
    pub image_id: i32,
}

impl DBWrapped for NewRecipeImage {
    fn new(_data: &serde_json::Value) -> Self {
        NewRecipeImage {
            recipe_id: 0,
            image_id: 0,
        }
    }

    fn exists(&self, pool: &SharedDatabasePool) -> Option<i32> {
        use crate::database::schema::recipe_images::dsl::*;
        let conn = &mut pool.get().unwrap();

        recipe_images
            .filter(recipe_id.eq(self.recipe_id))
            .filter(image_id.eq(self.image_id))
            .select(RecipeImage::as_select())
            .first::<RecipeImage>(conn)
            .map(|_| 0)
            .ok()
    }

    fn save(&self, pool: &SharedDatabasePool) -> Result<i32, diesel::result::Error> {
        let conn = &mut pool.get().unwrap();

        diesel::insert_into(crate::database::schema::recipe_images::table)
            .values(self)
            .execute(conn)?;

        let last_id: i32 = diesel::select(diesel::dsl::sql::<diesel::sql_types::Integer>(
            "last_insert_rowid()",
        ))
        .get_result(conn)?;

        Ok(last_id)
    }
}
