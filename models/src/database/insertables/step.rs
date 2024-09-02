use diesel::prelude::*;

use crate::database::SharedDatabasePool;
use crate::insertables::DBWrapped;

#[derive(Insertable)]
#[diesel(table_name = crate::database::schema::step)]
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

    fn exists(&self, pool: &SharedDatabasePool) -> Option<i32> {
        use crate::database::schema::step::dsl::*;
        let conn = &mut pool.get().unwrap();

        step.filter(recipe_id.eq(self.recipe_id))
            .filter(number.eq(self.number))
            .select(id)
            .first::<i32>(conn)
            .ok()
    }

    fn save(&self, pool: &SharedDatabasePool) -> Result<i32, diesel::result::Error> {
        let conn = &mut pool.get().unwrap();

        diesel::insert_into(crate::database::schema::step::table)
            .values(self)
            .execute(conn)
            .expect("Error saving new step");

        let last_id: i32 = diesel::select(diesel::dsl::sql::<diesel::sql_types::Integer>(
            "last_insert_rowid()",
        ))
        .get_result(conn)
        .expect("Error getting last insert rowid");
        Ok(last_id)
    }
}
