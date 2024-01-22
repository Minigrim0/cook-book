use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;

use crate::database::connection::establish_connection;

pub trait DBWrapped {
    fn new(data: &serde_json::Value) -> Self;
    fn exists(&self) -> Option<i32>;  // Returns the id of the existing row if any
    fn save(&self) -> Result<i32, diesel::result::Error>;
}

#[derive(Insertable)]
#[diesel(table_name = crate::schema::author)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct NewAuthor {
    pub type_: String,
    pub name: String,
    pub url: String,
}

impl DBWrapped for NewAuthor {
    fn new(data: &serde_json::Value) -> Self {
        NewAuthor {
            type_: data["@type"].as_str().unwrap_or("unknown").to_string(),
            name: data["name"].as_str().unwrap_or("unknown").to_string(),
            url: data["url"].as_str().unwrap_or("unknown").to_string()
        }
    }

    fn exists(&self) -> Option<i32> {
        use crate::schema::author::dsl::*;

        let connection: &mut SqliteConnection = &mut establish_connection();
        author
            .filter(url.eq(self.url.clone()))
            .select(id)
            .first::<i32>(connection)
            .ok()
    }

    fn save(&self) -> Result<i32, diesel::result::Error> {
        let connection: &mut SqliteConnection = &mut establish_connection();

        diesel::insert_into(crate::schema::author::table)
            .values(self)
            .execute(connection)
            .expect("Error saving new author");

            let last_id: i32 = diesel::select(
                diesel::dsl::sql::<diesel::sql_types::Integer>(
                    "last_insert_rowid()"
                )
            )
                .get_result(connection)
                .expect("Error getting last insert rowid");
        Ok(last_id)
    }
}


#[derive(Insertable)]
#[diesel(table_name = crate::schema::category)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct NewCategory {
    pub name: String,
}

impl DBWrapped for NewCategory {
    fn new(data: &serde_json::Value) -> Self {
        NewCategory {
            name: data["name"].as_str().unwrap_or("unknown").to_string(),
        }
    }

    fn exists(&self) -> Option<i32> {
        use crate::schema::category::dsl::*;
        let connection: &mut SqliteConnection = &mut establish_connection();

        category
            .filter(name.eq(self.name.clone()))
            .select(id)
            .first::<i32>(connection)
            .ok()
    }

    fn save(&self) -> Result<i32, diesel::result::Error> {
        let connection: &mut SqliteConnection = &mut establish_connection();

        diesel::insert_into(crate::schema::category::table)
            .values(self)
            .execute(connection)
            .expect("Error saving new category");

            let last_id: i32 = diesel::select(
                diesel::dsl::sql::<diesel::sql_types::Integer>(
                    "last_insert_rowid()"
                )
            )
                .get_result(connection)
                .expect("Error getting last insert rowid");
        Ok(last_id)
    }
}


#[derive(Insertable)]
#[diesel(table_name = crate::schema::cuisine)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct NewCuisine {
    pub name: String,
}

impl DBWrapped for NewCuisine {
    fn new(data: &serde_json::Value) -> Self {
        NewCuisine {
            name: data["name"].as_str().unwrap_or("unknown").to_string(),
        }
    }

    fn exists(&self) -> Option<i32> {
        use crate::schema::cuisine::dsl::*;
        let connection: &mut SqliteConnection = &mut establish_connection();

        cuisine
            .filter(name.eq(self.name.clone()))
            .select(id)
            .first::<i32>(connection)
            .ok()
    }

    fn save(&self) -> Result<i32, diesel::result::Error> {
        let connection: &mut SqliteConnection = &mut establish_connection();

        diesel::insert_into(crate::schema::cuisine::table)
            .values(self)
            .execute(connection)
            .expect("Error saving new cuisine");

            let last_id: i32 = diesel::select(
                diesel::dsl::sql::<diesel::sql_types::Integer>(
                    "last_insert_rowid()"
                )
            )
                .get_result(connection)
                .expect("Error getting last insert rowid");
        Ok(last_id)
    }
}


#[derive(Insertable)]
#[diesel(table_name = crate::schema::ingredient)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct NewIngredient {
    pub name: String,
}

impl DBWrapped for NewIngredient {
    fn new(data: &serde_json::Value) -> Self {
        NewIngredient {
            name: data["name"].as_str().unwrap_or("unknown").to_string(),
        }
    }

    fn exists(&self) -> Option<i32> {
        use crate::schema::ingredient::dsl::*;
        let connection: &mut SqliteConnection = &mut establish_connection();

        ingredient
            .filter(name.eq(self.name.clone()))
            .select(id)
            .first::<i32>(connection)
            .ok()
    }

    fn save(&self) -> Result<i32, diesel::result::Error> {
        let connection: &mut SqliteConnection = &mut establish_connection();

        diesel::insert_into(crate::schema::ingredient::table)
            .values(self)
            .execute(connection)
            .expect("Error saving new ingredient");

            let last_id: i32 = diesel::select(
                diesel::dsl::sql::<diesel::sql_types::Integer>(
                    "last_insert_rowid()"
                )
            )
                .get_result(connection)
                .expect("Error getting last insert rowid");
        Ok(last_id)
    }
}


#[derive(Insertable, Debug)]
#[diesel(table_name = crate::schema::rating)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct NewRating {
    pub score: f32,
    pub amount: i32,
}

impl DBWrapped for NewRating {
    fn new(data: &serde_json::Value) -> Self {
        NewRating {
            score: data["ratingValue"]
                .as_str()
                .unwrap_or("-1")
                .parse::<f32>()
                .ok()
                .unwrap_or(-1.0),
            amount: data["ratingCount"]
                .as_str()
                .unwrap_or("-1")
                .parse::<i32>()
                .ok()
                .unwrap_or(-1),
        }
    }

    fn exists(&self) -> Option<i32> {
        use crate::schema::rating::dsl::*;
        let connection: &mut SqliteConnection = &mut establish_connection();

        rating
            .filter(score.eq(self.score.clone()))
            .filter(amount.eq(self.amount.clone()))
            .select(id)
            .first::<i32>(connection)
            .ok()
    }

    fn save(&self) -> Result<i32, diesel::result::Error> {
        let connection: &mut SqliteConnection = &mut establish_connection();

        diesel::insert_into(crate::schema::rating::table)
            .values(self)
            .execute(connection)
            .expect("Error saving new rating");

            let last_id: i32 = diesel::select(
                diesel::dsl::sql::<diesel::sql_types::Integer>(
                    "last_insert_rowid()"
                )
            )
                .get_result(connection)
                .expect("Error getting last insert rowid");
        Ok(last_id)
    }
}


#[derive(Insertable)]
#[diesel(table_name = crate::schema::recipe)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct NewRecipe {
    pub name: String,
    pub cook_time: i32,
    pub prep_time: i32,
    pub yield_: i32,
    pub author_id: i32,
    pub rating_id: i32,
    pub category_id: i32,
}

impl DBWrapped for NewRecipe {
    fn new(data: &serde_json::Value) -> Self {
        NewRecipe {
            name: data["name"].as_str().unwrap_or("unknown").to_string(),
            cook_time: data["cook_time"].as_i64().unwrap_or(-1) as i32,
            prep_time: data["prep_time"].as_i64().unwrap_or(-1) as i32,
            yield_: data["yield"].as_i64().unwrap_or(-1) as i32,
            author_id: -1,
            rating_id: -1,
            category_id: -1,
        }
    }

    fn exists(&self) -> Option<i32> {
        use crate::schema::recipe::dsl::*;
        let connection: &mut SqliteConnection = &mut establish_connection();

        recipe
            .filter(name.eq(self.name.clone()))
            .filter(cook_time.eq(self.cook_time.clone()))
            .filter(prep_time.eq(self.prep_time.clone()))
            .filter(yield_.eq(self.yield_.clone()))
            .select(id)
            .first::<i32>(connection)
            .ok()
    }

    fn save(&self) -> Result<i32, diesel::result::Error> {
        let connection: &mut SqliteConnection = &mut establish_connection();

        diesel::insert_into(crate::schema::recipe::table)
            .values(self)
            .execute(connection)
            .expect("Error saving new recipe");

            let last_id: i32 = diesel::select(
                diesel::dsl::sql::<diesel::sql_types::Integer>(
                    "last_insert_rowid()"
                )
            )
                .get_result(connection)
                .expect("Error getting last insert rowid");
        Ok(last_id)
    }
}


#[derive(Insertable)]
#[diesel(table_name = crate::schema::recipe_ingredient)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct NewRecipeIngredient {
    pub recipe_id: i32,
    pub ingredient_id: i32,
    pub unit_id: i32,
    pub amount: String,
}

impl DBWrapped for NewRecipeIngredient {
    fn new(data: &serde_json::Value) -> Self {
        NewRecipeIngredient {
            recipe_id: -1,
            ingredient_id: -1,
            unit_id: -1,
            amount: data["amount"].as_str().unwrap_or("unknown").to_string(),
        }
    }

    fn exists(&self) -> Option<i32> {
        use crate::schema::recipe_ingredient::dsl::*;
        let connection: &mut SqliteConnection = &mut establish_connection();

        recipe_ingredient
            .filter(recipe_id.eq(self.recipe_id.clone()))
            .filter(ingredient_id.eq(self.ingredient_id.clone()))
            .select(id)
            .first::<i32>(connection)
            .ok()
    }

    fn save(&self) -> Result<i32, diesel::result::Error> {
        let connection: &mut SqliteConnection = &mut establish_connection();

        diesel::insert_into(crate::schema::recipe_ingredient::table)
            .values(self)
            .execute(connection)
            .expect("Error saving new recipe <-> ingredient");

            let last_id: i32 = diesel::select(
                diesel::dsl::sql::<diesel::sql_types::Integer>(
                    "last_insert_rowid()"
                )
            )
                .get_result(connection)
                .expect("Error getting last insert rowid");
        Ok(last_id)
    }
}


#[derive(Insertable)]
#[diesel(table_name = crate::schema::step)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct NewStep {
    pub recipe_id: i32,
    pub number: i32,
    pub description: String,
}

impl DBWrapped for NewStep {
    fn new(data: &serde_json::Value) -> Self {
        NewStep {
            recipe_id: -1,
            number: data["number"].as_i64().unwrap_or(-1) as i32,
            description: data["description"].as_str().unwrap_or("unknown").to_string(),
        }
    }

    fn exists(&self) -> Option<i32> {
        use crate::schema::step::dsl::*;
        let connection: &mut SqliteConnection = &mut establish_connection();

        step
            .filter(recipe_id.eq(self.recipe_id))
            .filter(number.eq(self.number))
            .select(id)
            .first::<i32>(connection)
            .ok()
    }

    fn save(&self) -> Result<i32, diesel::result::Error> {
        let connection: &mut SqliteConnection = &mut establish_connection();

        diesel::insert_into(crate::schema::step::table)
            .values(self)
            .execute(connection)
            .expect("Error saving new step");

            let last_id: i32 = diesel::select(
                diesel::dsl::sql::<diesel::sql_types::Integer>(
                    "last_insert_rowid()"
                )
            )
                .get_result(connection)
                .expect("Error getting last insert rowid");
        Ok(last_id)
    }
}


#[derive(Insertable)]
#[diesel(table_name = crate::schema::unit)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct NewUnit {
    pub name: String,
}

impl DBWrapped for NewUnit {
    fn new(data: &serde_json::Value) -> Self {
        NewUnit {
            name: data["name"].as_str().unwrap_or("unknown").to_string(),
        }
    }

    fn exists(&self) -> Option<i32> {
        use crate::schema::unit::dsl::*;
        let connection: &mut SqliteConnection = &mut establish_connection();

        unit
            .filter(name.eq(self.name.clone()))
            .select(id)
            .first::<i32>(connection)
            .ok()
    }

    fn save(&self) -> Result<i32, diesel::result::Error> {
        let connection: &mut SqliteConnection = &mut establish_connection();

        diesel::insert_into(crate::schema::unit::table)
            .values(self)
            .execute(connection)
            .expect("Error saving new unit");

            let last_id: i32 = diesel::select(
                diesel::dsl::sql::<diesel::sql_types::Integer>(
                    "last_insert_rowid()"
                )
            )
                .get_result(connection)
                .expect("Error getting last insert rowid");
        Ok(last_id)
    }
}
