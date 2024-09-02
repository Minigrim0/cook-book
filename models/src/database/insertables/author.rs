use diesel::prelude::*;

use crate::database::SharedDatabasePool;
use crate::insertables::DBWrapped;

#[derive(Insertable)]
#[diesel(table_name = crate::database::schema::author)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct NewAuthor {
    pub type_: String,
    pub name: String,
    pub url: String,
}

impl DBWrapped for NewAuthor {
    fn new(data: &serde_json::Value) -> Self {
        NewAuthor {
            type_: data["@type"].as_str().unwrap_or("unknown").to_string(),
            name: data["name"].as_str().unwrap_or("unknown").to_string(),
            url: data["url"].as_str().unwrap_or("unknown").to_string(),
        }
    }

    fn exists(&self, pool: &SharedDatabasePool) -> Option<i32> {
        use crate::database::schema::author::dsl::*;
        let conn = &mut pool.get().unwrap();

        author
            .filter(url.eq(self.url.clone()))
            .select(id)
            .first::<i32>(conn)
            .ok()
    }

    fn save(&self, pool: &SharedDatabasePool) -> Result<i32, diesel::result::Error> {
        let conn = &mut pool.get().unwrap();

        diesel::insert_into(crate::database::schema::author::table)
            .values(self)
            .execute(conn)
            .expect("Error saving new author");

        let last_id: i32 = diesel::select(diesel::dsl::sql::<diesel::sql_types::Integer>(
            "last_insert_rowid()",
        ))
        .get_result(conn)
        .expect("Error getting last insert rowid");
        Ok(last_id)
    }
}
