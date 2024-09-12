use diesel::prelude::*;
use log;

use crate::database::SharedDatabasePool;
use crate::insertables::DBWrapped;


#[derive(Insertable, Debug)]
#[diesel(table_name = crate::database::schema::job_log)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
struct NewJobLog {
    pub job_id: i32,
    pub log_entry: String,
    pub created_at: chrono::NaiveDateTime,
}


impl DBWrapped for NewJobLog {
    fn new(data: &serde_json::Value) -> Self {
        log::debug!("Creating new JobLog with data: {:?}", data);
        NewJobLog {
            job_id: data["job_id"].as_i64().unwrap_or_else(|| {
                log::warn!("job_id not found or invalid, defaulting to 0");
                0
            }) as i32,
            log_entry: data["log_entry"].as_str().unwrap_or_else(|| {
                log::warn!("log_entry not found, defaulting to empty string");
                ""
            }).to_string(),
            created_at: chrono::NaiveDateTime::parse_from_str(
                data["created_at"].as_str().unwrap_or("1970-01-01 00:00:00"),
                "%Y-%m-%d %H:%M:%S",
            )
            .unwrap_or_else(|e| {
                log::error!("Error parsing created_at: {}. Defaulting to epoch", e);
                chrono::NaiveDateTime::from_timestamp(0, 0)
            }),
        }
    }

    fn exists(&self, pool: &SharedDatabasePool) -> Option<i32> {
        use crate::database::schema::job_log::dsl::*;
        let conn = &mut pool.get().unwrap();

        log::debug!("Checking if JobLog exists: job_id={}, log_entry={}, created_at={}", 
                    self.job_id, self.log_entry, self.created_at);

        let result = job_log
            .filter(job_id.eq(self.job_id))
            .filter(log_entry.eq(&self.log_entry))
            .filter(created_at.eq(self.created_at))
            .select(id)
            .first::<Option<i32>>(conn)
            .unwrap_or(None);

        match &result {
            Some(i) => log::info!("Existing JobLog found with id: {}", i),
            None => log::info!("No existing JobLog found"),
        }

        result
    }

    fn save(&self, pool: &SharedDatabasePool) -> Result<i32, diesel::result::Error> {
        let conn = &mut pool.get().unwrap();

        log::info!("Saving new JobLog: job_id={}, log_entry={}, created_at={}", 
                   self.job_id, self.log_entry, self.created_at);

        diesel::insert_into(crate::database::schema::job_log::table)
            .values(self)
            .execute(conn)?;

        let last_id: i32 = diesel::select(diesel::dsl::sql::<diesel::sql_types::Integer>(
            "last_insert_rowid()",
        ))
        .get_result(conn)?;

        log::info!("New JobLog saved with id: {}", last_id);
        Ok(last_id)
    }
}


