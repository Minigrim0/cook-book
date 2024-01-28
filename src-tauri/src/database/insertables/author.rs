use diesel::prelude::*;

use crate::database::{wrapper::DBWrapped, connection::establish_connection};

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
            url: data["url"].as_str().unwrap_or("unknown").to_string()
        }
    }

    fn exists(&self) -> Option<i32> {
        use crate::database::schema::author::dsl::*;

        let connection: &mut SqliteConnection = &mut establish_connection();
        author
            .filter(url.eq(self.url.clone()))
            .select(id)
            .first::<i32>(connection)
            .ok()
    }

    fn save(&self) -> Result<i32, diesel::result::Error> {
        let connection: &mut SqliteConnection = &mut establish_connection();

        diesel::insert_into(crate::database::schema::author::table)
            .values(self)
            .execute(connection)
            .expect("Error saving new author");

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
