#[cfg(feature = "database")]
use diesel::prelude::*;

#[cfg(feature = "database")]
#[derive(Queryable, Selectable)]
#[cfg(feature = "database")]
#[diesel(table_name = crate::database::schema::author)]
#[cfg(feature = "database")]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Author {
    pub id: i32,
    pub type_: String,
    pub name: String,
    pub url: String,
}

#[cfg(feature = "database")]
#[derive(Queryable, Selectable)]
#[cfg(feature = "database")]
#[diesel(table_name = crate::database::schema::category)]
#[cfg(feature = "database")]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Category {
    pub id: i32,
    pub name: String,
}

#[cfg(feature = "database")]
#[derive(Queryable, Selectable)]
#[cfg(feature = "database")]
#[diesel(table_name = crate::database::schema::cuisine)]
#[cfg(feature = "database")]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Cuisine {
    pub id: i32,
    pub name: String,
}

#[cfg(feature = "database")]
#[derive(Queryable, Selectable)]
#[cfg(feature = "database")]
#[diesel(table_name = crate::database::schema::ingredient)]
#[cfg(feature = "database")]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Ingredient {
    pub id: i32,
    pub name: String,
}

#[cfg(feature = "database")]
#[derive(Queryable, Selectable)]
#[cfg(feature = "database")]
#[diesel(table_name = crate::database::schema::rating)]
#[cfg(feature = "database")]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Rating {
    pub id: i32,
    pub score: f32,
    pub amount: i32,
}

#[cfg(feature = "database")]
#[derive(Queryable, Selectable)]
#[cfg(feature = "database")]
#[diesel(table_name = crate::database::schema::recipe)]
#[cfg(feature = "database")]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
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

#[cfg(feature = "database")]
#[derive(Queryable, Selectable)]
#[cfg(feature = "database")]
#[diesel(table_name = crate::database::schema::recipe_ingredient)]
#[cfg(feature = "database")]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct RecipeIngredient {
    pub id: i32,
    pub recipe_id: i32,
    pub ingredient_id: i32,
    pub unit_id: i32,
    pub amount: f32,
    pub details: Option<String>,
}

#[cfg(feature = "database")]
#[derive(Queryable, Selectable)]
#[cfg(feature = "database")]
#[diesel(table_name = crate::database::schema::step)]
#[cfg(feature = "database")]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Step {
    pub id: i32,
    pub recipe_id: i32,
    pub number: i32,
    pub description: String,
}

#[cfg(feature = "database")]
#[derive(Queryable, Selectable)]
#[cfg(feature = "database")]
#[diesel(table_name = crate::database::schema::unit)]
#[cfg(feature = "database")]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Unit {
    pub id: i32,
    pub name: String,
}
