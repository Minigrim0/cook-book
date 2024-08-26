use diesel::prelude::*;
use diesel::r2d2::CustomizeConnection;
use diesel::r2d2::{ConnectionManager, Error as R2D2Error, Pool};

use super::{init_database_paths, run_migrations};

#[derive(Copy, Clone, Debug)]
pub struct SetupUserTableCustomizer;

/// This customization of connection ensures all migrations are run
impl CustomizeConnection<SqliteConnection, R2D2Error> for SetupUserTableCustomizer {
    fn on_acquire(&self, conn: &mut SqliteConnection) -> Result<(), R2D2Error> {
        run_migrations(conn).map_err(|e| {
            R2D2Error::ConnectionError(ConnectionError::BadConnection(e.to_string()))
        })?;
        Ok(())
    }
}

/// Creates a connection pool using r2d2. This will run migrations upon acquiring connection
pub fn get_connection_pool() -> Result<Pool<ConnectionManager<SqliteConnection>>, String> {
    init_database_paths()?;

    let url = super::database_url()?;
    let manager = ConnectionManager::<SqliteConnection>::new(url);

    Ok(Pool::builder()
        .max_size(1)
        .test_on_check_out(true)
        .connection_customizer(Box::new(SetupUserTableCustomizer))
        .build(manager)
        .expect("Could not build connection pool"))
}
