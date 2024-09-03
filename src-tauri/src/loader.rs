use log::{error, trace};
use std::{
    fs, io,
    path::PathBuf,
    sync::{Arc, Mutex},
};
use tauri::Window;
use threadpool::ThreadPool;

use super::parser;
use models::get_connection_pool;

pub fn load_folder(data_path: PathBuf, window: Window) {
    let path_to_load = Arc::new(Mutex::new(data_path));

    std::thread::spawn(move || {
        let path_to_load = path_to_load.clone();
        let path_to_load = path_to_load.lock().unwrap();

        let paths = match fs::read_dir(&*path_to_load) {
            Ok(p) => match p
                .map(|res| res.map(|e| e.path()))
                .collect::<Result<Vec<_>, io::Error>>()
            {
                Ok(f) => f,
                Err(e) => {
                    error!("An error occured {}", e.to_string());
                    return;
                }
            },
            Err(e) => {
                error!("An error occured {}", e.to_string());
                return;
            }
        };
        let count = paths.len();

        let n_workers = 8;
        let pool = ThreadPool::new(n_workers);
        let db_pool = get_connection_pool().unwrap();

        for (index, path) in paths.iter().enumerate() {
            let window = window.clone();
            let path = Arc::new(Mutex::new(path.clone()));
            let db_pool = db_pool.clone();

            pool.execute(move || {
                let window = window.clone();
                let path = match path.lock() {
                    Ok(p) => p,
                    Err(_) => return,
                };

                // Emit start
                window
                    .emit(
                        "loading://starting",
                        serde_json::json!({
                            "path": path.to_str().unwrap(),
                            "id": index as i32,
                            "total": count
                        }),
                    )
                    .unwrap();

                if let Err(error) = parser::json_parser::parse_recipe(&path, Arc::new(db_pool)) {
                    error!("Error while loading recipe: {}", error.to_string());

                    // Emit error
                    window
                        .emit(
                            "loading://error",
                            serde_json::json!({
                                "id": index as i32,
                                "error": error.to_string()
                            }),
                        )
                        .unwrap();
                } else {
                    trace!("Loaded recipe {}", path.to_str().unwrap_or("err"));
                    // Emit completed
                    window
                        .emit(
                            "loading://completed",
                            serde_json::json!({
                                "id": index as i32,
                            }),
                        )
                        .unwrap();
                }
            });
        }

        pool.join();
    });
}
