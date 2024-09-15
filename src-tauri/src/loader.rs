use log::{error, trace};
use std::{
    fs, io,
    path::PathBuf,
    sync::{Arc, Mutex},
};
use threadpool::ThreadPool;

use super::parser;
use models::models::{Updateable, Job};
use models::database::job;
use models::SharedDatabasePool;

pub fn load_folder(data_path: PathBuf, job_id: Option<i32>, db_pool: SharedDatabasePool) {
    let path_to_load = Arc::new(Mutex::new(data_path.clone()));

    std::thread::spawn(move || {
        let path_to_load = path_to_load.clone();
        let path_to_load = path_to_load.lock().unwrap();

        let mut job: Option<Job> = job_id.map(|id| job::get_job(id, &db_pool).unwrap());

        let paths = match fs::read_dir(&*path_to_load) {
            Ok(p) => match p
                .map(|res| res.map(|e| e.path()))
                .collect::<Result<Vec<_>, io::Error>>()
            {
                Ok(f) => f,
                Err(e) => {
                    error!("An error occured {}", e.to_string());
                    if let Some(job) = &mut job {
                        job.status = "error".to_string();
                        job.details = Some(format!("An error occured {}", e.to_string()));
                        job.update(&db_pool).unwrap();
                    }
                    return;
                }
            },
            Err(e) => {
                error!("An error occured {}", e.to_string());
                if let Some(job) = &mut job {
                    job.status = "error".to_string();
                    job.details = Some(format!("An error occured {}", e.to_string()));
                    job.update(&db_pool).unwrap();
                }
                return;
            }
        };
        let count = paths.len();

        let n_workers = 8;
        let pool = ThreadPool::new(n_workers);

        for (index, path) in paths.iter().enumerate() {
            let job = job.clone();
            let path = Arc::new(Mutex::new(path.clone()));
            let db_pool = db_pool.clone();

            pool.execute(move || {
                let mut job = job.clone();
                let path = match path.lock() {
                    Ok(p) => p,
                    Err(_) => return,
                };

                // Set job to loading
                if let Some(job) = &mut job {
                    job.status = "loading".to_string();
                    job.progress = (index as f32) / (count as f32);
                    job.update(&db_pool).unwrap();
                }

                if let Err(error) = parser::json_parser::parse_recipe(&path, db_pool.clone()) {
                    error!("Error while loading recipe: {}", error.to_string());

                    // Emit error
                    if let Some(job) = &mut job {
                        job.progress = (index as f32) / (count as f32);
                        if let Err(e) = job.add_log(&db_pool, format!("Error while loading recipe: {}", error.to_string())) {
                            error!("Error while adding log: {}", e.to_string());
                        }
                        job.update(&db_pool).unwrap();
                    }
                } else {
                    trace!("Loaded recipe {}", path.to_str().unwrap_or("err"));

                    if let Some(job) = &mut job {
                        if let Err(e) = job.add_log(&db_pool, format!("Loaded recipe {}", path.to_str().unwrap_or("err"))) {
                            error!("Error while adding log: {}", e.to_string());
                        }
                        job.progress = (index as f32) / (count as f32);
                        job.update(&db_pool).unwrap();
                    }
                }
            });
        }

        pool.join();
    });
}
