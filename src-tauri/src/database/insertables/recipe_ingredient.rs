use diesel::prelude::*;

use crate::database::insertables::DBWrapped;
use crate::SharedDatabasePool;

#[derive(Insertable)]
#[diesel(table_name = crate::database::schema::recipe_ingredient)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct NewRecipeIngredient {
    pub recipe_id: i32,
    pub ingredient_id: i32,
    pub unit_id: i32,
    pub amount: f32,
    pub details: Option<String>,
    pub full: Option<String>,
}

impl DBWrapped for NewRecipeIngredient {
    fn new(data: &serde_json::Value) -> Self {
        NewRecipeIngredient {
            recipe_id: data["r_id"].as_i64().unwrap_or(-1) as i32,
            ingredient_id: data["i_id"].as_i64().unwrap_or(-1) as i32,
            unit_id: data["u_id"].as_i64().unwrap_or(-1) as i32,
            amount: data["amount"].as_f64().unwrap_or(-1.0) as f32,
            details: data["details"].as_str().map(|s| s.to_string()),
            full: data["full"].as_str().map(|s| s.to_string()),
        }
    }

    fn exists(&self, pool: &SharedDatabasePool) -> Option<i32> {
        use crate::database::schema::recipe_ingredient::dsl::*;
        let conn = &mut pool.get().unwrap();

        recipe_ingredient
            .filter(recipe_id.eq(self.recipe_id.clone()))
            .filter(ingredient_id.eq(self.ingredient_id.clone()))
            .select(id)
            .first::<i32>(conn)
            .ok()
    }

    fn save(&self, pool: &SharedDatabasePool) -> Result<i32, diesel::result::Error> {
        let conn = &mut pool.get().unwrap();

        diesel::insert_into(crate::database::schema::recipe_ingredient::table)
            .values(self)
            .execute(conn)
            .expect("Error saving new recipe <-> ingredient");

        let last_id: i32 = diesel::select(diesel::dsl::sql::<diesel::sql_types::Integer>(
            "last_insert_rowid()",
        ))
        .get_result(conn)
        .expect("Error getting last insert rowid");
        Ok(last_id)
    }
}
