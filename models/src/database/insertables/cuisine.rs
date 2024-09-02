use diesel::prelude::*;

use crate::database::SharedDatabasePool;
use crate::insertables::DBWrapped;

#[derive(Insertable)]
#[diesel(table_name = crate::database::schema::cuisine)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct NewCuisine {
    pub name: String,
}

impl DBWrapped for NewCuisine {
    fn new(data: &serde_json::Value) -> Self {
        NewCuisine {
            name: data["name"].as_str().unwrap_or("unknown").to_string(),
        }
    }

    fn exists(&self, pool: &SharedDatabasePool) -> Option<i32> {
        use crate::database::schema::cuisine::dsl::*;
        let conn = &mut pool.get().unwrap();

        cuisine
            .filter(name.eq(self.name.clone()))
            .select(id)
            .first::<i32>(conn)
            .ok()
    }

    fn save(&self, pool: &SharedDatabasePool) -> Result<i32, diesel::result::Error> {
        let conn = &mut pool.get().unwrap();

        diesel::insert_into(crate::database::schema::cuisine::table)
            .values(self)
            .execute(conn)
            .expect("Error saving new cuisine");

        let last_id: i32 = diesel::select(diesel::dsl::sql::<diesel::sql_types::Integer>(
            "last_insert_rowid()",
        ))
        .get_result(conn)
        .expect("Error getting last insert rowid");
        Ok(last_id)
    }
}
