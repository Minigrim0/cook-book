use diesel::prelude::*;

pub trait Saveable {
    fn save(&self, connection: &mut SqliteConnection) -> Result<i32, diesel::result::Error>;
}

#[derive(Insertable)]
#[diesel(table_name = crate::schema::author)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct NewAuthor {
    pub id: i32,
    pub type_: String,
    pub name: String,
    pub url: String,
}

impl Saveable for NewAuthor {
    fn save(&self, connection: &mut SqliteConnection) -> Result<i32, diesel::result::Error> {
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
    pub id: i32,
    pub name: String,
}


#[derive(Insertable)]
#[diesel(table_name = crate::schema::cuisine)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct NewCuisine {
    pub id: i32,
    pub name: String,
}


#[derive(Insertable)]
#[diesel(table_name = crate::schema::ingredient)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct NewIngredient {
    pub id: i32,
    pub name: String,
}


#[derive(Insertable)]
#[diesel(table_name = crate::schema::rating)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct NewRating {
    pub id: i32,
    pub score: i32,
    pub amount: i32,
}


#[derive(Insertable)]
#[diesel(table_name = crate::schema::recipe)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct NewRecipe {
    pub id: i32,
    pub name: String,
    pub cook_time: i32,
    pub prep_time: i32,
    pub yield_: i32,
    pub author_id: i32,
    pub rating_id: i32,
    pub category_id: i32,
}


#[derive(Insertable)]
#[diesel(table_name = crate::schema::recipe_ingredient)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct NewRecipeIngredient {
    pub id: i32,
    pub recipe_id: i32,
    pub ingredient_id: i32,
    pub unit_id: i32,
    pub amount: String,
}


#[derive(Insertable)]
#[diesel(table_name = crate::schema::step)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct NewStep {
    pub id: i32,
    pub recipe_id: i32,
    pub number: i32,
    pub description: String,
}


#[derive(Insertable)]
#[diesel(table_name = crate::schema::unit)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct NewUnit {
    pub id: i32,
    pub name: String,
}
