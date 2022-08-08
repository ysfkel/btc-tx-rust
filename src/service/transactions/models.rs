use super::schema::{transactions, users};
use diesel;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use diesel::{Associations, Insertable, Queryable};
use serde::{Deserialize, Serialize};

#[derive(Debug, Queryable, Insertable, Serialize, Deserialize)]
#[table_name = "users"]
pub struct User {
    id: String,
    first_name: String,
    last_name: String,
}
