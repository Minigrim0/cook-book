use diesel::prelude::*;

use crate::database::{wrapper::DBWrapped, connection::establish_connection};

#[derive(Insertable, Debug)]
#[diesel(table_name = crate::database::schema::rating)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct NewRating {
    pub score: f32,
    pub amount: i32,
}

impl DBWrapped for NewRating {
    fn new(data: &serde_json::Value) -> Self {
        NewRating {
            score: data["ratingValue"]
                .as_str()
                .unwrap_or("-1")
                .parse::<f32>()
                .ok()
                .unwrap_or(-1.0),
            amount: data["ratingCount"]
                .as_str()
                .unwrap_or("-1")
                .parse::<i32>()
                .ok()
                .unwrap_or(-1),
        }
    }

    fn exists(&self) -> Option<i32> {
        use crate::database::schema::rating::dsl::*;
        let connection: &mut SqliteConnection = &mut establish_connection();

        rating
            .filter(score.eq(self.score.clone()))
            .filter(amount.eq(self.amount.clone()))
            .select(id)
            .first::<i32>(connection)
            .ok()
    }

    fn save(&self) -> Result<i32, diesel::result::Error> {
        let connection: &mut SqliteConnection = &mut establish_connection();

        diesel::insert_into(crate::database::schema::rating::table)
            .values(self)
            .execute(connection)
            .expect("Error saving new rating");

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

