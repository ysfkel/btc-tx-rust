use crate::service::models::User;
use crate::service::schema::users;
use crate::service::schema::users::dsl::users as users_orm;
use diesel::pg::PgConnection;
use diesel::prelude::*;

impl User {
    pub fn get(id: String, conn: &PgConnection) -> Vec<User> {
        users_orm
            .find(id)
            .load::<User>(conn) // cast to book struct
            .expect("Error loading book")
    }

    pub fn get_all(conn: &PgConnection) -> Vec<User> {
        users_orm
            .order(users::id.desc())
            .load::<User>(conn)
            .expect("error loading books")
    }
    pub fn insert(tx: User, conn: &PgConnection) -> bool {
        diesel::insert_into(users::table)
            .values(&tx)
            .execute(conn)
            .is_ok()
    }
}
