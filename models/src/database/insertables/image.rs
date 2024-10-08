use diesel::prelude::*;

use crate::database::SharedDatabasePool;
use crate::insertables::DBWrapped;

#[derive(Insertable)]
#[diesel(table_name = crate::database::schema::image_blobs)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct NewImage {
    pub image_data: Vec<u8>,
    pub hash: String,
}

impl DBWrapped for NewImage {
    fn new(_data: &serde_json::Value) -> Self {
        NewImage {
            image_data: Vec::new(),
            hash: String::new(),
        }
    }

    fn exists(&self, pool: &SharedDatabasePool) -> Option<i32> {
        use crate::database::schema::image_blobs::dsl::*;
        let conn = &mut pool.get().unwrap();

        image_blobs
            .filter(hash.eq(self.hash.clone()))
            .select(id)
            .first::<i32>(conn)
            .ok()
    }

    fn save(&self, pool: &SharedDatabasePool) -> Result<i32, diesel::result::Error> {
        let conn = &mut pool.get().unwrap();

        diesel::insert_into(crate::database::schema::image_blobs::table)
            .values(self)
            .execute(conn)?;

        let last_id: i32 = diesel::select(diesel::dsl::sql::<diesel::sql_types::Integer>(
            "last_insert_rowid()",
        ))
        .get_result(conn)?;

        Ok(last_id)
    }
}
