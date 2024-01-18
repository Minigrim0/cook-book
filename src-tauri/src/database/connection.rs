use diesel::sqlite::SqliteConnection;
use diesel::prelude::*;
use directories::ProjectDirs;

pub fn establish_connection() -> SqliteConnection {
    if let Some(proj_dirs) = ProjectDirs::from("xyz", "Minigrim0",  "Cookbook") {
        let database_path = proj_dirs.data_dir();
        let database_url = format!("{}/cookbook.db", database_path.to_str().unwrap());

        SqliteConnection::establish(&database_url)
            .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
    } else {
        panic!("Error getting project directories");
    }
}
