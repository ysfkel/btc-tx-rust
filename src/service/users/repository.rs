use crate::service::users::models::User;
use crate::service::users::schema::users;
use crate::service::users::schema::users::dsl::users as users_orm;
use diesel::pg::PgConnection;
use diesel::prelude::*;

type DbResult = Result<Vec<User>, diesel::result::Error>;

pub struct Repository<'a> {
    pub conn: &'a PgConnection,
}

impl<'a> Repository<'a> {
    pub fn new(conn: &'a PgConnection) -> Self {
        Self { conn }
    }
    pub fn get(&self, id: String) -> DbResult {
        users_orm.find(id).load::<User>(self.conn)
    }

    pub fn get_all(&self) -> DbResult {
        users_orm.order(users::id.desc()).load::<User>(self.conn)
    }

    pub fn insert_all(&self, tx: Vec<User>) -> Result<usize, diesel::result::Error> {
        diesel::insert_into(users::table)
            .values(&tx)
            .execute(self.conn)
    }
}
