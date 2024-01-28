use diesel::prelude::*;

use crate::database::{wrapper::DBWrapped, connection::establish_connection};
use crate::utils::iso8601_to_seconds;

#[derive(Insertable)]
#[diesel(table_name = crate::database::schema::recipe)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct NewRecipe {
    pub name: String,
    pub cook_time: i32,
    pub prep_time: i32,
    pub yield_: i32,
    pub author_id: i32,
    pub rating_id: i32,
    pub category_id: i32,
}

impl DBWrapped for NewRecipe {
    fn new(data: &serde_json::Value) -> Self {
        NewRecipe {
            name: data["name"].as_str().unwrap_or("unknown").to_string(),
            cook_time: if let Some(time) = data["cookTime"].as_str() {
                if time != "" { iso8601_to_seconds(time.to_string()) }
                else { -1 }
            } else {
                -1
            },
            prep_time: if let Some(time) = data["prepTime"].as_str() {
                if time != "" { iso8601_to_seconds(time.to_string()) }
                else { -1 }
            } else {
                -1
            },
            yield_: data["recipeYield"]
                .as_i64()
                .unwrap_or(-1) as i32,
            author_id: -1,
            rating_id: -1,
            category_id: -1,
        }
    }

    fn exists(&self) -> Option<i32> {
        use crate::database::schema::recipe::dsl::*;
        let connection: &mut SqliteConnection = &mut establish_connection();

        recipe
            .filter(name.eq(self.name.clone()))
            .filter(cook_time.eq(self.cook_time.clone()))
            .filter(prep_time.eq(self.prep_time.clone()))
            .filter(yield_.eq(self.yield_.clone()))
            .select(id)
            .first::<i32>(connection)
            .ok()
    }

    fn save(&self) -> Result<i32, diesel::result::Error> {
        let connection: &mut SqliteConnection = &mut establish_connection();

        diesel::insert_into(crate::database::schema::recipe::table)
            .values(self)
            .execute(connection)
            .expect("Error saving new recipe");

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
