#[cfg(feature = "database")]
use diesel::prelude::*;
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

#[cfg(feature = "database")]
use sha2::{Digest, Sha256};

#[cfg(feature = "database")]
use crate::database::SharedDatabasePool;
#[cfg(feature = "database")]
use crate::database::insertables::NewJobLog;

#[cfg(feature = "database")]
pub trait Updateable {
    /// Update the row in the database
    /// 
    /// # Arguments
    /// 
    /// * `pool` - The database connection pool
    /// 
    /// # Returns
    /// 
    /// * `Result<i32, diesel::result::Error>` - The id of the updated row
    fn update(&self, pool: &SharedDatabasePool) -> Result<i32, diesel::result::Error>;
}

#[cfg_attr(feature = "database", derive(Queryable, Selectable))]
#[cfg_attr(feature = "database", diesel(table_name = crate::database::schema::author))]
#[cfg_attr(feature = "database", diesel(check_for_backend(diesel::sqlite::Sqlite)))]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Author {
    pub id: i32,
    pub type_: String,
    pub name: String,
    pub url: String,
}

#[cfg(feature = "database")]
impl Updateable for Author {
    fn update(&self, pool: &SharedDatabasePool) -> Result<i32, diesel::result::Error> {
        let conn = &mut pool.get().unwrap();

        diesel::update(crate::database::schema::author::table)
            .filter(crate::database::schema::author::id.eq(self.id))
            .set((
                crate::database::schema::author::type_.eq(&self.type_),
                crate::database::schema::author::name.eq(&self.name),
                crate::database::schema::author::url.eq(&self.url),
            ))
            .execute(conn)?;

        Ok(self.id)
    }
}

#[cfg_attr(feature = "database", derive(Queryable, Selectable))]
#[cfg_attr(feature = "database", diesel(table_name = crate::database::schema::category))]
#[cfg_attr(feature = "database", diesel(check_for_backend(diesel::sqlite::Sqlite)))]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Category {
    pub id: i32,
    pub name: String,
}

#[cfg(feature = "database")]
impl Updateable for Category {
    fn update(&self, pool: &SharedDatabasePool) -> Result<i32, diesel::result::Error> {
        let conn = &mut pool.get().unwrap();

        diesel::update(crate::database::schema::category::table)
            .filter(crate::database::schema::category::id.eq(self.id))
            .set(crate::database::schema::category::name.eq(&self.name))
            .execute(conn)?;

        Ok(self.id)
    }
}

#[cfg_attr(feature = "database", derive(Queryable, Selectable))]
#[cfg_attr(feature = "database", diesel(table_name = crate::database::schema::cuisine))]
#[cfg_attr(feature = "database", diesel(check_for_backend(diesel::sqlite::Sqlite)))]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Cuisine {
    pub id: i32,
    pub name: String,
}

#[cfg(feature = "database")]
impl Updateable for Cuisine {
    fn update(&self, pool: &SharedDatabasePool) -> Result<i32, diesel::result::Error> {
        let conn = &mut pool.get().unwrap();

        diesel::update(crate::database::schema::cuisine::table)
            .filter(crate::database::schema::cuisine::id.eq(self.id))
            .set(crate::database::schema::cuisine::name.eq(&self.name))
            .execute(conn)?;

        Ok(self.id)
    }
}

#[cfg_attr(feature = "database", derive(Queryable, Selectable))]
#[cfg_attr(feature = "database", diesel(table_name = crate::database::schema::ingredient))]
#[cfg_attr(feature = "database", diesel(check_for_backend(diesel::sqlite::Sqlite)))]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Ingredient {
    pub id: i32,
    pub name: String,
}

#[cfg(feature = "database")]
impl Updateable for Ingredient {
    fn update(&self, pool: &SharedDatabasePool) -> Result<i32, diesel::result::Error> {
        let conn = &mut pool.get().unwrap();

        diesel::update(crate::database::schema::ingredient::table)
            .filter(crate::database::schema::ingredient::id.eq(self.id))
            .set(crate::database::schema::ingredient::name.eq(&self.name))
            .execute(conn)?;

        Ok(self.id)
    }
}

#[cfg_attr(feature = "database", derive(Queryable, Selectable))]
#[cfg_attr(feature = "database", diesel(table_name = crate::database::schema::rating))]
#[cfg_attr(feature = "database", diesel(check_for_backend(diesel::sqlite::Sqlite)))]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Rating {
    pub id: i32,
    pub score: f32,
    pub amount: i32,
}

#[cfg(feature = "database")]
impl Updateable for Rating {
    fn update(&self, pool: &SharedDatabasePool) -> Result<i32, diesel::result::Error> {
        let conn = &mut pool.get().unwrap();

        diesel::update(crate::database::schema::rating::table)
            .filter(crate::database::schema::rating::id.eq(self.id))
            .set((
                crate::database::schema::rating::score.eq(self.score),
                crate::database::schema::rating::amount.eq(self.amount),
            ))
            .execute(conn)?;

        Ok(self.id)
    }
}

#[cfg_attr(feature = "database", derive(Queryable, Selectable))]
#[cfg_attr(feature = "database", diesel(table_name = crate::database::schema::recipe))]
#[cfg_attr(feature = "database", diesel(check_for_backend(diesel::sqlite::Sqlite)))]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Recipe {
    pub id: i32,
    pub name: String,
    pub cook_time: i32,
    pub prep_time: i32,
    pub yield_: i32,
    pub author_id: i32,
    pub rating_id: i32,
    pub category_id: i32,
    #[cfg(not(feature = "database"))]
    pub image: Option<String>,
}

#[cfg(feature = "database")]
impl Updateable for Recipe {
    fn update(&self, pool: &SharedDatabasePool) -> Result<i32, diesel::result::Error> {
        let conn = &mut pool.get().unwrap();

        diesel::update(crate::database::schema::recipe::table)
            .filter(crate::database::schema::recipe::id.eq(self.id))
            .set((
                crate::database::schema::recipe::name.eq(&self.name),
                crate::database::schema::recipe::cook_time.eq(self.cook_time),
                crate::database::schema::recipe::prep_time.eq(self.prep_time),
                crate::database::schema::recipe::yield_.eq(self.yield_),
                crate::database::schema::recipe::author_id.eq(self.author_id),
                crate::database::schema::recipe::rating_id.eq(self.rating_id),
                crate::database::schema::recipe::category_id.eq(self.category_id),
            ))
            .execute(conn)?;

        Ok(self.id)
    }
}

#[cfg_attr(feature = "database", derive(Queryable, Selectable))]
#[cfg_attr(feature = "database", diesel(table_name = crate::database::schema::recipe_ingredient))]
#[cfg_attr(feature = "database", diesel(check_for_backend(diesel::sqlite::Sqlite)))]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RecipeIngredient {
    pub id: i32,
    pub recipe_id: i32,
    pub ingredient_id: i32,
    pub unit_id: i32,
    pub amount: f32,
    pub details: Option<String>,
}

#[cfg(feature = "database")]
impl Updateable for RecipeIngredient {
    fn update(&self, pool: &SharedDatabasePool) -> Result<i32, diesel::result::Error> {
        let conn = &mut pool.get().unwrap();

        diesel::update(crate::database::schema::recipe_ingredient::table)
            .filter(crate::database::schema::recipe_ingredient::id.eq(self.id))
            .set((
                crate::database::schema::recipe_ingredient::recipe_id.eq(self.recipe_id),
                crate::database::schema::recipe_ingredient::ingredient_id.eq(self.ingredient_id),
                crate::database::schema::recipe_ingredient::unit_id.eq(self.unit_id),
                crate::database::schema::recipe_ingredient::amount.eq(self.amount),
                crate::database::schema::recipe_ingredient::details.eq(&self.details),
            ))
            .execute(conn)?;

        Ok(self.id)
    }
}

#[cfg_attr(feature = "database", derive(Queryable, Selectable))]
#[cfg_attr(feature = "database", diesel(table_name = crate::database::schema::image_blobs))]
#[cfg_attr(feature = "database", diesel(check_for_backend(diesel::sqlite::Sqlite)))]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Image {
    pub id: i32,
    pub image_data: Vec<u8>,
    pub hash: String,
}

#[cfg(feature = "database")]
impl Updateable for Image {
    fn update(&self, pool: &SharedDatabasePool) -> Result<i32, diesel::result::Error> {
        let conn = &mut pool.get().unwrap();
        let mut hasher = Sha256::new();
        hasher.update(&self.image_data);
        let new_hash = format!("{:x}", hasher.finalize());

        diesel::update(crate::database::schema::image_blobs::table)
            .filter(crate::database::schema::image_blobs::id.eq(self.id))
            .set((
                crate::database::schema::image_blobs::image_data.eq(&self.image_data),
                crate::database::schema::image_blobs::hash.eq(new_hash),
            ))
            .execute(conn)?;

        Ok(self.id)
    }
}

#[cfg_attr(feature = "database", derive(Queryable, Selectable))]
#[cfg_attr(feature = "database", diesel(table_name = crate::database::schema::step))]
#[cfg_attr(feature = "database", diesel(check_for_backend(diesel::sqlite::Sqlite)))]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Step {
    pub id: i32,
    pub recipe_id: i32,
    pub number: i32,
    pub description: String,
}

#[cfg(feature = "database")]
impl Updateable for Step {
    fn update(&self, pool: &SharedDatabasePool) -> Result<i32, diesel::result::Error> {
        let conn = &mut pool.get().unwrap();

        diesel::update(crate::database::schema::step::table)
            .filter(crate::database::schema::step::id.eq(self.id))
            .set((
                crate::database::schema::step::recipe_id.eq(self.recipe_id),
                crate::database::schema::step::number.eq(self.number),
                crate::database::schema::step::description.eq(&self.description),
            ))
            .execute(conn)?;

        Ok(self.id)
    }
}

#[cfg_attr(feature = "database", derive(Queryable, Selectable))]
#[cfg_attr(feature = "database", diesel(table_name = crate::database::schema::unit))]
#[cfg_attr(feature = "database", diesel(check_for_backend(diesel::sqlite::Sqlite)))]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Unit {
    pub id: i32,
    pub name: String,
}

#[cfg(feature = "database")]
impl Updateable for Unit {
    fn update(&self, pool: &SharedDatabasePool) -> Result<i32, diesel::result::Error> {
        let conn = &mut pool.get().unwrap();

        diesel::update(crate::database::schema::unit::table)
            .filter(crate::database::schema::unit::id.eq(self.id))
            .set(crate::database::schema::unit::name.eq(&self.name))
            .execute(conn)?;

        Ok(self.id)
    }
}

#[cfg_attr(feature = "database", derive(Queryable, Selectable))]
#[cfg_attr(feature = "database", diesel(table_name = crate::database::schema::job))]
#[cfg_attr(feature = "database", diesel(check_for_backend(diesel::sqlite::Sqlite)))]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Job {
    pub id: i32,
    pub status: String,
    pub progress: f32,
    pub details: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[cfg(feature = "database")]
impl Updateable for Job {
    fn update(&self, pool: &SharedDatabasePool) -> Result<i32, diesel::result::Error> {
        let conn = &mut pool.get().unwrap();

        diesel::update(crate::database::schema::job::table)
            .filter(crate::database::schema::job::id.eq(self.id))
            .set((
                crate::database::schema::job::status.eq(&self.status),
                crate::database::schema::job::progress.eq(self.progress),
                crate::database::schema::job::details.eq(&self.details),
                crate::database::schema::job::updated_at.eq(self.updated_at),
            ))
            .execute(conn)?;

        Ok(self.id)
    }
}

#[cfg_attr(feature = "database", derive(Queryable, Selectable))]
#[cfg_attr(feature = "database", diesel(table_name = crate::database::schema::job_log))]
#[cfg_attr(feature = "database", diesel(check_for_backend(diesel::sqlite::Sqlite)))]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct JobLog {
    pub id: i32,
    pub job_id: i32,
    pub log_entry: String,
    pub created_at: NaiveDateTime,
}

#[cfg(feature = "database")]
impl Updateable for JobLog {
    fn update(&self, pool: &SharedDatabasePool) -> Result<i32, diesel::result::Error> {
        let conn = &mut pool.get().unwrap();

        diesel::update(crate::database::schema::job_log::table)
            .filter(crate::database::schema::job_log::id.eq(self.id))
            .set((
                crate::database::schema::job_log::job_id.eq(self.job_id),
                crate::database::schema::job_log::log_entry.eq(&self.log_entry),
                crate::database::schema::job_log::created_at.eq(self.created_at),
            ))
            .execute(conn)?;

        Ok(self.id)
    }
}

#[cfg(feature = "database")]
impl Job {
    pub fn add_log(&self, pool: &SharedDatabasePool, log_entry: String) -> Result<i32, diesel::result::Error> {
        let conn = &mut pool.get().unwrap();

        let new_log = NewJobLog {
            job_id: self.id,
            log_entry: log_entry,
            created_at: chrono::Local::now().naive_local(),
        };

        diesel::insert_into(crate::database::schema::job_log::table)
            .values(new_log)
            .execute(conn)?;

        Ok(self.id)
    }
}
