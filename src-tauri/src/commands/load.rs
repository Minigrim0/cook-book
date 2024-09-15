use tauri;
use log::{info, warn};

use models::insertables::{NewJob, DBWrapped};
use models::database::job;
use models::models::Updateable;
use crate::loader;
use models::SharedDatabasePool;

#[tauri::command]
pub fn load_path(state: tauri::State<SharedDatabasePool>) -> Option<i32> {
    info!("Invoking dialog to load a folder");
    let db_pool = state.inner().clone();

    // Create a new job to load the folder
    let job = NewJob {
        status: "picking folder".to_string(),
        progress: 0.0,
        details: Some("waiting for the user to pick a folder".to_string()),
        created_at: chrono::Utc::now().naive_utc(),
        updated_at: chrono::Utc::now().naive_utc(),
    };
    let job_id = job.save(&db_pool).ok();

    tauri::api::dialog::FileDialogBuilder::new().pick_folder(move |folder| match folder {
        Some(folder) => {
            // Update job status
            if let Some(id) = job_id {
                let job = job::get_job(id, &db_pool).ok();

                if let Some(mut job) = job {
                    job.status = "loading".to_string();
                    job.details = Some("loading recipes".to_string());
                    job.update(&db_pool).unwrap();
                }
            }

            loader::load_folder(folder, job_id, db_pool);
        }
        None => warn!("Canceled folder loading"),
    });

    job_id
}
