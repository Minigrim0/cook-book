use diesel::prelude::*;

use crate::database::SharedDatabasePool;
use crate::insertables::DBWrapped;

#[derive(Insertable, Debug)]
#[diesel(table_name = crate::database::schema::job)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct NewJob {
    pub status: String,
    pub progress: f32,
    pub details: Option<String>,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}


impl DBWrapped for NewJob {
    fn new(data: &serde_json::Value) -> Self {
        NewJob {
            status: data["status"].as_str().unwrap_or("unknown").to_string(),
            progress: data["progress"].as_f64().unwrap_or(0.0) as f32,
            details: data["details"].as_str().map_or(None, |s| Some(s.to_string())),
            created_at: chrono::NaiveDateTime::parse_from_str(
                data["created_at"].as_str().unwrap_or("1970-01-01 00:00:00"),
                "%Y-%m-%d %H:%M:%S",
            ).unwrap(),
            updated_at: chrono::NaiveDateTime::parse_from_str(
                data["updated_at"].as_str().unwrap_or("1970-01-01 00:00:00"),
                "%Y-%m-%d %H:%M:%S",
            ).unwrap(),
        }
    }

    fn exists(&self, pool: &SharedDatabasePool) -> Option<i32> {
        use crate::database::schema::job::dsl::*;
        let conn = &mut pool.get().unwrap();

        let result = job
            .filter(progress.eq(self.progress))
            .filter(status.eq(self.status.clone()))
            .filter(created_at.eq(self.created_at))
            .filter(updated_at.eq(self.updated_at))
            .select(id)
            .first::<i32>(conn);

        match &result {
            Ok(i) => log::info!("Existing Job found with id: {}", i),
            Err(e) => log::info!("No existing Job found: {}", e),
        }

        result.ok()
    }

    fn save(&self, pool: &SharedDatabasePool) -> Result<i32, diesel::result::Error> {
        let conn = &mut pool.get().unwrap();

        diesel::insert_into(crate::database::schema::job::table)
            .values(self)
            .execute(conn)?;

        let last_id: i32 = diesel::select(diesel::dsl::sql::<diesel::sql_types::Integer>(
            "last_insert_rowid()",
        ))
        .get_result(conn)?;
        Ok(last_id)
    }
}
