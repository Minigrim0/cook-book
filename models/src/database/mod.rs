use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
use directories::ProjectDirs;

use diesel::r2d2::{ConnectionManager, Pool, PooledConnection};

use diesel::SqliteConnection;
use log::error;

use std::sync::Arc;

mod pool;
pub mod schema;

pub mod job;
pub mod insertables;
pub mod loadables;

pub type DatabasePool = Pool<ConnectionManager<SqliteConnection>>;

pub type SharedDatabasePool = Arc<DatabasePool>;

pub use pool::get_connection_pool;

pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!();

/// Returns the path to the database
/// 
/// # Errors
/// 
/// Returns an error if the project directories are not found
pub fn database_url() -> Result<String, String> {
    if let Some(proj_dirs) = ProjectDirs::from("xyz", "Minigrim0", "Cookbook") {
        let database_path = proj_dirs.data_dir();
        Ok(format!("{}/cookbook.db", database_path.to_str().unwrap()))
    } else {
        Err("Unable to find the project data directory.".to_string())
    }
}

/// Runs the migrations on the database
/// 
/// # Errors
/// 
/// Returns an error if the migrations fail
pub fn run_migrations(connection: &mut diesel::sqlite::SqliteConnection) -> Result<(), String> {
    match connection.run_pending_migrations(MIGRATIONS) {
        Ok(_m) => Ok(()),
        Err(e) => Err(format!("Error running migrations: {:?}", e)),
    }
}

/// Initializes paths to the database by creating the required folders in the
/// host filesystem. The path is built using the `directories` crate.
/// 
/// # Errors
/// 
/// Returns an error if the project directories are not found
pub fn init_database_paths() -> Result<(), String> {
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


/// Returns a connection from the database connectoin pool
/// 
/// # Errors
/// 
/// Returns an error if the connection pool is not available
pub fn get_connection() -> Result<PooledConnection<ConnectionManager<diesel::SqliteConnection>>, String> {
    match get_connection_pool() {
        Ok(pool) => match pool.get() {
            Ok(conn) => Ok(conn),
            Err(e) => {
                error!("unable to get connection pool: {}", e.to_string());
                Err(e.to_string())
            }
        },
        Err(e) => {
            error!("unable to get connection pool: {}", e.to_string());
            Err(e.to_string())
        }
    }
}
