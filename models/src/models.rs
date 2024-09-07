#[cfg(feature = "database")]
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

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
    } else {
            #[derive(Serialize, Deserialize, Debug, Clone)]
            pub struct Unit {
                pub id: i32,
                pub name: String,
            }
    }
}

/// The type of the shared data for the ingredients
/// This is a tuple of the ingredients (paginated), the total number of ingredients and the number of pages
/// corresponding to the query
pub type PaginatedIngredients = (Vec<Ingredient>, usize, usize);
