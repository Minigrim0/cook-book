use diesel::prelude::*;

use crate::database::insertables::DBWrapped;
use crate::SharedDatabasePool;

#[derive(Insertable)]
#[diesel(table_name = crate::database::schema::ingredient)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct NewIngredient {
    pub name: String,
}

impl DBWrapped for NewIngredient {
    fn new(data: &serde_json::Value) -> Self {
        NewIngredient {
            name: if data["name"].as_str().unwrap_or("").to_string() == "" {
                data["unit"].as_str().unwrap_or("unknown").to_string()
            } else {
                data["name"].as_str().unwrap_or("unknown").to_string()
            },
        }
    }

    fn exists(&self, pool: &SharedDatabasePool) -> Option<i32> {
        use crate::database::schema::ingredient::dsl::*;
        let conn = &mut pool.get().unwrap();

        ingredient
            .filter(name.eq(self.name.clone()))
            .select(id)
            .first::<i32>(conn)
            .ok()
    }

    fn save(&self, pool: &SharedDatabasePool) -> Result<i32, diesel::result::Error> {
        let conn = &mut pool.get().unwrap();

        diesel::insert_into(crate::database::schema::ingredient::table)
            .values(self)
            .execute(conn)
            .expect("Error saving new ingredient");

        let last_id: i32 = diesel::select(diesel::dsl::sql::<diesel::sql_types::Integer>(
            "last_insert_rowid()",
        ))
        .get_result(conn)
        .expect("Error getting last insert rowid");
        Ok(last_id)
    }
}
