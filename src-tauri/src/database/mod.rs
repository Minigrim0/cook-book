use diesel::r2d2::{ConnectionManager, Pool};
use diesel::SqliteConnection;
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
use directories::ProjectDirs;

mod pool;

pub mod insertables;
pub mod models;
pub mod schema;

pub use pool::get_connection_pool;

pub type DatabasePool = Pool<ConnectionManager<SqliteConnection>>;

pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!();

pub fn database_url() -> Result<String, String> {
    if let Some(proj_dirs) = ProjectDirs::from("xyz", "Minigrim0", "Cookbook") {
        let database_path = proj_dirs.data_dir();
        Ok(format!("{}/cookbook.db", database_path.to_str().unwrap()))
    } else {
        Err("Unable to find the project data directory.".to_string())
    }
}

fn run_migrations(connection: &mut diesel::sqlite::SqliteConnection) -> Result<(), String> {
    match connection.run_pending_migrations(MIGRATIONS) {
        Ok(_m) => Ok(()),
        Err(e) => Err(format!("Error running migrations: {:?}", e)),
    }
}

/// Initializes paths to the database by creating the required folders in the
/// host filesystem. The path is built using the `directories` crate.
fn init_database_paths() -> Result<(), String> {
    if let Some(proj_dirs) = ProjectDirs::from("xyz", "Minigrim0", "cookbook") {
        let database_path = proj_dirs.data_dir();

        if !database_path.exists() {
            std::fs::create_dir_all(database_path).expect("Error creating database directory");
        }
        Ok(())
    } else {
        Err("Error getting project directories".to_string())
    }
}
