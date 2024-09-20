use diesel::prelude::*;

use crate::database::SharedDatabasePool;
use crate::insertables::DBWrapped;

#[derive(Insertable)]
#[diesel(table_name = crate::database::schema::image_blobs)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct NewImage {

}