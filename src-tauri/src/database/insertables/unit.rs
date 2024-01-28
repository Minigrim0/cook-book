use diesel::prelude::*;

use crate::database::{wrapper::DBWrapped, connection::establish_connection};

#[derive(Insertable)]
#[diesel(table_name = crate::database::schema::unit)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct NewUnit {
    pub name: String,
}

impl DBWrapped for NewUnit {
    fn new(data: &serde_json::Value) -> Self {
        NewUnit {
            name: if data["name"].as_str().unwrap_or("").to_string() == "" {  // If there is no name, unit is piece
                "piece".to_string()
            } else {
                data["unit"].as_str().unwrap_or("unknown").to_string()
            },
        }
    }

    fn exists(&self) -> Option<i32> {
        use crate::database::schema::unit::dsl::*;
        let connection: &mut SqliteConnection = &mut establish_connection();

        unit
            .filter(name.eq(self.name.clone()))
            .select(id)
            .first::<i32>(connection)
            .ok()
    }

    fn save(&self) -> Result<i32, diesel::result::Error> {
        let connection: &mut SqliteConnection = &mut establish_connection();

        diesel::insert_into(crate::database::schema::unit::table)
            .values(self)
            .execute(connection)
            .expect("Error saving new unit");

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
