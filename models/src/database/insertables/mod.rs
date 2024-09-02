pub mod author;
pub mod category;
pub mod cuisine;
pub mod ingredient;
pub mod rating;
pub mod recipe;
pub mod recipe_ingredient;
pub mod step;
pub mod unit;

use crate::database::SharedDatabasePool;

pub trait DBWrapped {
    fn new(data: &serde_json::Value) -> Self;
    fn exists(&self, pool: &SharedDatabasePool) -> Option<i32>; // Returns the id of the existing row if any
    fn save(&self, pool: &SharedDatabasePool) -> Result<i32, diesel::result::Error>;
}
