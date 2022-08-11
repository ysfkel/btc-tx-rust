use super::schema::users;
use crate::service::transactions::schema::transactions;
use diesel;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use diesel::{Associations, Insertable, Queryable};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Queryable, Insertable, Serialize, Deserialize)]
#[table_name = "users"]
pub struct User {
    pub id: String,
    pub name: String,
}
