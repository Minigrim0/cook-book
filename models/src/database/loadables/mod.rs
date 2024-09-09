use log::error;
use diesel;
use diesel::r2d2::{PooledConnection, ConnectionManager};

use crate::get_connection_pool;

mod author;
mod cuisine;
mod ingredient;
mod recipe;

pub use author::*;
pub use cuisine::*;
pub use ingredient::*;
pub use recipe::*;

/// Returns a connection from the database connectoin pool
fn get_connection() -> Result<PooledConnection<ConnectionManager<diesel::SqliteConnection>>, String>{
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
