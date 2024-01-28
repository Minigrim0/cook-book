use diesel::prelude::*;

use crate::database::{wrapper::DBWrapped, connection::establish_connection};

#[derive(Insertable)]
#[diesel(table_name = crate::schema::step)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct NewStep {
    pub recipe_id: i32,
    pub number: i32,
    pub description: String,
}

impl DBWrapped for NewStep {
    fn new(data: &serde_json::Value) -> Self {
        NewStep {
            recipe_id: data["r_id"].as_i64().unwrap_or(-1) as i32,
            number: data["step"].as_i64().unwrap_or(-1) as i32,
            description: data["data"].as_str().unwrap_or("unknown").to_string(),
        }
    }

    fn exists(&self) -> Option<i32> {
        use crate::schema::step::dsl::*;
        let connection: &mut SqliteConnection = &mut establish_connection();

        step
            .filter(recipe_id.eq(self.recipe_id))
            .filter(number.eq(self.number))
            .select(id)
            .first::<i32>(connection)
            .ok()
    }

    fn save(&self) -> Result<i32, diesel::result::Error> {
        let connection: &mut SqliteConnection = &mut establish_connection();

        diesel::insert_into(crate::schema::step::table)
            .values(self)
            .execute(connection)
            .expect("Error saving new step");

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
