use directories::ProjectDirs;
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
use crate::database::connection::establish_connection;

pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!();

fn run_migrations(connection: &mut diesel::sqlite::SqliteConnection) {
    match connection.run_pending_migrations(MIGRATIONS) {
        Ok(_m) => (),
        Err(e) => panic!("Error running migrations: {:?}", e),
    }
}

pub fn init_database() {
    if let Some(proj_dirs) = ProjectDirs::from("xyz", "Minigrim0",  "cookbook") {
        let database_path = proj_dirs.data_dir();

        if !database_path.exists() {
            std::fs::create_dir_all(database_path).expect("Error creating database directory");
        }
        run_migrations(&mut establish_connection());
    } else {
        panic!("Error getting project directories");
    }
}
