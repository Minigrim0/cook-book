use diesel::prelude::*;
use crate::models::{Job, JobLog};
use super::get_connection;

pub fn create_job(new_job: NewJob) -> Result<Job, diesel::result::Error> {
    use crate::database::schema::job::dsl::*;

    let conn = &mut get_connection();
    diesel::insert_into(job)
        .values(&new_job)
        .execute(conn)?;

    job.order(id.desc()).first(conn)
}

pub fn update_job(job_id: i32, updated_job: NewJob) -> Result<Job, diesel::result::Error> {
    use crate::database::schema::job::dsl::*;

    let conn = &mut get_connection();
    diesel::update(job.find(job_id))
        .set(&updated_job)
        .execute(conn)?;

    job.find(job_id).first(conn)
}

pub fn add_job_log(new_log: NewJobLog) -> Result<JobLog, diesel::result::Error> {
    use crate::database::schema::job_log::dsl::*;

    let conn = &mut get_connection();
    diesel::insert_into(job_log)
        .values(&new_log)
        .execute(conn)?;

    job_log.order(id.desc()).first(conn)
}

pub fn get_job_logs(job_id: i32) -> Result<Vec<JobLog>, diesel::result::Error> {
    use crate::database::schema::job_log::dsl::*;

    let conn = &mut get_connection();
    job_log.filter(crate::database::schema::job_log::job_id.eq(job_id))
        .load::<JobLog>(conn)
}

pub fn get_job(job_id: i32) -> Result<JobWithLogs, diesel::result::Error> {
    use crate::database::schema::job::dsl::*;
    use crate::database::schema::job_log::dsl::*;

    let conn = &mut get_connection();
    let job_result = job.find(job_id).first::<Job>(conn)?;
    let logs_result = job_log
        .filter(crate::database::schema::job_log::job_id.eq(job_id))
        .load::<JobLog>(conn)?;

    Ok(JobWithLogs {
        job: job_result,
        logs: logs_result,
    })
}

