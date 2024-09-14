use tauri;

use models::database::job;
use models::JobWithLogs;
use models::SharedDatabasePool;

#[tauri::command]
pub fn get_job_status(state: tauri::State<SharedDatabasePool>, job_id: i32) -> Result<JobWithLogs, String> {
    job::get_job_with_logs(job_id, &state)
}