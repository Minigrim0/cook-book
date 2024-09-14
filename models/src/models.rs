#[cfg(feature = "database")]
use diesel::prelude::*;
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

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

// Author
cfg_if::cfg_if! {
    if #[cfg(feature = "database")] {
        #[derive(Queryable, Selectable)]
        #[diesel(table_name = crate::database::schema::author)]
        #[diesel(check_for_backend(diesel::sqlite::Sqlite))]
        #[derive(Serialize, Deserialize, Debug, Clone)]
        pub struct Author {
            pub id: i32,
            pub type_: String,
            pub name: String,
            pub url: String,
        }

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
    } else {
        #[derive(Serialize, Deserialize, Debug, Clone)]
        pub struct Author {
            pub id: i32,
            pub type_: String,
            pub name: String,
            pub url: String,
        }
    }
}

// Category
cfg_if::cfg_if! {
    if #[cfg(feature = "database")] {
        #[derive(Queryable, Selectable)]
        #[diesel(table_name = crate::database::schema::category)]
        #[diesel(check_for_backend(diesel::sqlite::Sqlite))]
        #[derive(Serialize, Deserialize, Debug, Clone)]
        pub struct Category {
            pub id: i32,
            pub name: String,
        }

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
    } else {
        #[derive(Serialize, Deserialize, Debug, Clone)]
        pub struct Category {
            pub id: i32,
            pub name: String,
        }
    }
}

// Cuisine
cfg_if::cfg_if! {
    if #[cfg(feature = "database")] {
        #[derive(Queryable, Selectable)]
        #[diesel(table_name = crate::database::schema::cuisine)]
        #[diesel(check_for_backend(diesel::sqlite::Sqlite))]
        #[derive(Serialize, Deserialize, Debug, Clone)]
        pub struct Cuisine {
            pub id: i32,
            pub name: String,
        }

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
    } else {
        #[derive(Serialize, Deserialize, Debug, Clone)]
        pub struct Cuisine {
            pub id: i32,
            pub name: String,
        }
    }
}


// Ingredient
cfg_if::cfg_if! {
    if #[cfg(feature = "database")] {
        #[derive(Queryable, Selectable)]
        #[diesel(table_name = crate::database::schema::ingredient)]
        #[diesel(check_for_backend(diesel::sqlite::Sqlite))]
        #[derive(Serialize, Deserialize, Debug, Clone)]
        pub struct Ingredient {
            pub id: i32,
            pub name: String,
        }

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
    } else {
        #[derive(Serialize, Deserialize, Debug, Clone)]
        pub struct Ingredient {
            pub id: i32,
            pub name: String,
        }
    }
}

// Rating
cfg_if::cfg_if! {
    if #[cfg(feature = "database")] {
        #[derive(Queryable, Selectable)]
        #[diesel(table_name = crate::database::schema::rating)]
        #[diesel(check_for_backend(diesel::sqlite::Sqlite))]
        #[derive(Serialize, Deserialize, Debug, Clone)]
        pub struct Rating {
            pub id: i32,
            pub score: f32,
            pub amount: i32,
        }

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
    } else {
        #[derive(Serialize, Deserialize, Debug, Clone)]
        pub struct Rating {
            pub id: i32,
            pub score: f32,
            pub amount: i32,
        }
    }
}

// Recipe
cfg_if::cfg_if! {
    if #[cfg(feature = "database")] {
        #[derive(Queryable, Selectable)]
        #[diesel(table_name = crate::database::schema::recipe)]
        #[diesel(check_for_backend(diesel::sqlite::Sqlite))]
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
            pub image: Option<String>,
        }

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
                        crate::database::schema::recipe::image.eq(&self.image),
                    ))
                    .execute(conn)?;

                Ok(self.id)
            }
        }
    } else {
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
            pub image: Option<String>,
        }
    }
}

// RecipeIngredient
cfg_if::cfg_if! {
    if #[cfg(feature = "database")] {
        #[derive(Queryable, Selectable)]
        #[diesel(table_name = crate::database::schema::recipe_ingredient)]
        #[diesel(check_for_backend(diesel::sqlite::Sqlite))]
        #[derive(Serialize, Deserialize, Debug, Clone)]
        pub struct RecipeIngredient {
            pub id: i32,
            pub recipe_id: i32,
            pub ingredient_id: i32,
            pub unit_id: i32,
            pub amount: f32,
            pub details: Option<String>,
        }

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
    } else {
        #[derive(Serialize, Deserialize, Debug, Clone)]
        pub struct RecipeIngredient {
            pub id: i32,
            pub recipe_id: i32,
            pub ingredient_id: i32,
            pub unit_id: i32,
            pub amount: f32,
            pub details: Option<String>,
        }
    }
}

// Step
cfg_if::cfg_if! {
    if #[cfg(feature = "database")] {
        #[derive(Queryable, Selectable)]
        #[diesel(table_name = crate::database::schema::step)]
        #[diesel(check_for_backend(diesel::sqlite::Sqlite))]
        #[derive(Serialize, Deserialize, Debug, Clone)]
        pub struct Step {
            pub id: i32,
            pub recipe_id: i32,
            pub number: i32,
            pub description: String,
        }

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
    } else {
        #[derive(Serialize, Deserialize, Debug, Clone)]
        pub struct Step {
            pub id: i32,
            pub recipe_id: i32,
            pub number: i32,
            pub description: String,
        }
    }
}

// Unit
cfg_if::cfg_if! {
    if #[cfg(feature = "database")] {
        #[derive(Queryable, Selectable)]
        #[diesel(table_name = crate::database::schema::unit)]
        #[diesel(check_for_backend(diesel::sqlite::Sqlite))]
        #[derive(Serialize, Deserialize, Debug, Clone)]
        pub struct Unit {
            pub id: i32,
            pub name: String,
        }

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
    } else {
        #[derive(Serialize, Deserialize, Debug, Clone)]
        pub struct Unit {
            pub id: i32,
            pub name: String,
        }
    }
}

// Job
cfg_if::cfg_if! {
    if #[cfg(feature = "database")] {
        #[derive(Queryable, Selectable)]
        #[diesel(table_name = crate::database::schema::job)]
        #[diesel(check_for_backend(diesel::sqlite::Sqlite))]
        #[derive(Serialize, Deserialize, Debug, Clone)]
        pub struct Job {
            pub id: i32,
            pub status: String,
            pub progress: f32,
            pub details: Option<String>,
            pub created_at: NaiveDateTime,
            pub updated_at: NaiveDateTime,
        }

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
    } else {
        #[derive(Serialize, Deserialize, Debug, Clone)]
        pub struct Job {
            pub id: i32,
            pub status: String,
            pub progress: f32,
            pub details: Option<String>,
            pub created_at: NaiveDateTime,
            pub updated_at: NaiveDateTime,
        }
    }
}

// JobLog
cfg_if::cfg_if! {
    if #[cfg(feature = "database")] {
        #[derive(Queryable, Selectable)]
        #[diesel(table_name = crate::database::schema::job_log)]
        #[diesel(check_for_backend(diesel::sqlite::Sqlite))]
        #[derive(Serialize, Deserialize, Debug, Clone)]
        pub struct JobLog {
            pub id: i32,
            pub job_id: i32,
            pub log_entry: String,
            pub created_at: NaiveDateTime,
        }

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
    } else {
        #[derive(Serialize, Deserialize, Debug, Clone)]
        pub struct JobLog {
            pub id: i32,
            pub job_id: i32,
            pub log_entry: String,
            pub created_at: NaiveDateTime,
        }
    }
}
