use diesel::prelude::*;

use crate::database::SharedDatabasePool;
use crate::insertables::DBWrapped;

#[derive(Insertable)]
#[diesel(table_name = crate::database::schema::category)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct NewCategory {
    pub name: String,
}

impl DBWrapped for NewCategory {
    fn new(data: &serde_json::Value) -> Self {
        NewCategory {
            name: data["recipeCategory"]
                .as_str()
                .unwrap_or("unknown")
                .to_string(),
        }
    }

    fn exists(&self, pool: &SharedDatabasePool) -> Option<i32> {
        use crate::database::schema::category::dsl::*;
        let conn = &mut pool.get().unwrap();

        category
            .filter(name.eq(self.name.clone()))
            .select(id)
            .first::<i32>(conn)
            .ok()
    }

    fn save(&self, pool: &SharedDatabasePool) -> Result<i32, diesel::result::Error> {
        let conn = &mut pool.get().unwrap();

        diesel::insert_into(crate::database::schema::category::table)
            .values(self)
            .execute(conn)
            .expect("Error saving new category");

        let last_id: i32 = diesel::select(diesel::dsl::sql::<diesel::sql_types::Integer>(
            "last_insert_rowid()",
        ))
        .get_result(conn)
        .expect("Error getting last insert rowid");
        Ok(last_id)
    }
}
