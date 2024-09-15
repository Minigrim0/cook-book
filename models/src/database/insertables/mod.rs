mod author;
mod category;
mod cuisine;
mod ingredient;
mod rating;
mod recipe;
mod recipe_ingredient;
mod step;
mod unit;
mod job;
mod job_log;

pub use author::*;
pub use category::*;
pub use cuisine::*;
pub use ingredient::*;
pub use rating::*;
pub use recipe::*;
pub use recipe_ingredient::*;
pub use step::*;
pub use unit::*;
pub use job::*;
pub use job_log::*;

use crate::database::SharedDatabasePool;

pub trait DBWrapped {
    fn new(data: &serde_json::Value) -> Self;
    fn exists(&self, pool: &SharedDatabasePool) -> Option<i32>; // Returns the id of the existing row if any
    fn save(&self, pool: &SharedDatabasePool) -> Result<i32, diesel::result::Error>;
}
