use diesel;
use diesel::r2d2::{ConnectionManager, PooledConnection};
use log::error;

use crate::get_connection_pool;

mod author;
mod category;
mod cuisine;
mod ingredient;
mod rating;
mod recipe;
mod unit;

pub use author::*;
pub use category::*;
pub use cuisine::*;
pub use ingredient::*;
pub use rating::*;
pub use recipe::*;
pub use unit::*;

/// Returns a connection from the database connectoin pool
fn get_connection() -> Result<PooledConnection<ConnectionManager<diesel::SqliteConnection>>, String>
{
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
