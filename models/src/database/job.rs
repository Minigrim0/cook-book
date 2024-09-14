use diesel::prelude::*;

use crate::insertables::*;
use crate::models::{Job, JobLog};
use crate::shared::JobWithLogs;
use crate::database::SharedDatabasePool;

/// Create a new job
/// 
/// # Arguments
/// 
/// * `new_job` - The job to create
/// * `pool` - The shared database pool
/// 
/// # Returns
/// 
/// A `Result` containing the created job if successful, or an error message if not
pub fn create_job(new_job: NewJob, pool: &SharedDatabasePool) -> Result<Job, String> {
    use crate::database::schema::job::dsl::*;

    let conn = &mut pool.get().map_err(|e| e.to_string())?;
    diesel::insert_into(job)
        .values(&new_job)
        .execute(conn)
        .map_err(|e| e.to_string())?;

    job.order(id.desc()).first(conn).map_err(|e| e.to_string())
}

/// Update a job
/// 
/// # Arguments
/// 
/// * `job_id` - The id of the job to update
/// * `updated_job` - The new job data
/// * `pool` - The shared database pool
/// 
/// # Returns
/// 
/// A `Result` containing the updated job if successful, or an error message if not
pub fn update_job(job_id: i32, updated_job: NewJob, pool: &SharedDatabasePool) -> Result<Job, String> {
    use crate::database::schema::job::dsl::*;

    let conn = &mut pool.get().map_err(|e| e.to_string())?;
    diesel::update(job.filter(id.eq(job_id)))
        .set((
            status.eq(updated_job.status),
            progress.eq(updated_job.progress),
            updated_at.eq(updated_job.updated_at),
            details.eq(updated_job.details),
        ))
        .execute(conn)
        .map_err(|e| e.to_string())?;

    job.find(job_id).first(conn).map_err(|e| e.to_string())
}

/// Add a log to a job
/// 
/// # Arguments
/// 
/// * `new_log` - The log to add
/// * `pool` - The shared database pool
/// 
/// # Returns
/// 
/// A `Result` containing the added log if successful, or an error message if not
pub fn add_job_log(new_log: NewJobLog, pool: &SharedDatabasePool) -> Result<JobLog, String> {
    use crate::database::schema::job_log::dsl::*;

    let conn = &mut pool.get().map_err(|e| e.to_string())?;
    diesel::insert_into(job_log)
        .values(&new_log)
        .execute(conn)
        .map_err(|e| e.to_string())?;

    job_log.order(id.desc()).first(conn).map_err(|e| e.to_string())
}

/// Get the logs of a job
/// 
/// # Arguments
/// 
/// * `jid` - The id of the job to get the logs of
/// * `pool` - The shared database pool
/// 
/// # Returns
/// 
/// A `Result` containing the logs if successful, or an error message if not
pub fn get_job_logs(jid: i32, pool: &SharedDatabasePool) -> Result<Vec<JobLog>, String> {
    use crate::database::schema::job_log::dsl::*;

    let conn = &mut pool.get().map_err(|e| e.to_string())?;

    job_log.filter(job_id.eq(jid))
        .load::<JobLog>(conn)
        .map_err(|e| e.to_string())
}


/// Get a job
/// 
/// # Arguments
/// 
/// * `job_id` - The id of the job to get
/// * `pool` - The shared database pool
/// 
/// # Returns
/// 
/// A `Result` containing the job if successful, or an error message if not
pub fn get_job(job_id: i32, pool: &SharedDatabasePool) -> Result<Job, String> {
    let conn = &mut pool.get().map_err(|e| e.to_string())?;

    let job_result = crate::schema::job::dsl::job
        .find(job_id)
        .first::<Job>(conn)
        .map_err(|e| e.to_string())?;

    Ok(job_result)
}



/// Get a job with its logs
/// 
/// # Arguments
/// 
/// * `job_id` - The id of the job to get
/// * `pool` - The shared database pool
/// 
/// # Returns
/// 
/// A `Result` containing the job with its logs if successful, or an error message if not
pub fn get_job_with_logs(job_id: i32, pool: &SharedDatabasePool) -> Result<JobWithLogs, String> {
    let conn = &mut pool.get().map_err(|e| e.to_string())?;

    let job_result = crate::schema::job::dsl::job
        .find(job_id)
        .first::<Job>(conn)
        .map_err(|e| e.to_string())?;
    let logs_result = crate::schema::job_log::dsl::job_log
        .filter(crate::database::schema::job_log::job_id.eq(job_id))
        .load::<JobLog>(conn).map_err(|e| e.to_string())?;

    Ok(JobWithLogs {
        job: job_result,
        logs: logs_result,
    })
}
