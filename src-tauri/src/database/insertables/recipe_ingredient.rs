use diesel::prelude::*;

use crate::database::{wrapper::DBWrapped, connection::establish_connection};

#[derive(Insertable)]
#[diesel(table_name = crate::schema::recipe_ingredient)]
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

    fn exists(&self) -> Option<i32> {
        use crate::schema::recipe_ingredient::dsl::*;
        let connection: &mut SqliteConnection = &mut establish_connection();

        recipe_ingredient
            .filter(recipe_id.eq(self.recipe_id.clone()))
            .filter(ingredient_id.eq(self.ingredient_id.clone()))
            .select(id)
            .first::<i32>(connection)
            .ok()
    }

    fn save(&self) -> Result<i32, diesel::result::Error> {
        let connection: &mut SqliteConnection = &mut establish_connection();

        diesel::insert_into(crate::schema::recipe_ingredient::table)
            .values(self)
            .execute(connection)
            .expect("Error saving new recipe <-> ingredient");

            let last_id: i32 = diesel::select(
                diesel::dsl::sql::<diesel::sql_types::Integer>(
                    "last_insert_rowid()"
                )
            )
                .get_result(connection)
                .expect("Error getting last insert rowid");
        Ok(last_id)
    }
}
