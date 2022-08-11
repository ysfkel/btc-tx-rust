use crate::service::transactions::models::Transaction;
use crate::service::transactions::schema::transactions;
use crate::service::transactions::schema::transactions::dsl::transactions as transactions_orm;
use diesel::dsl;
use diesel::pg::PgConnection;
use diesel::prelude::*;

pub type DbResult = Result<Vec<Transaction>, diesel::result::Error>;

pub struct Repository<'a> {
    pub conn: &'a PgConnection,
}

impl<'a> Repository<'a> {
    pub fn new(conn: &'a PgConnection) -> Self {
        Self { conn }
    }
    pub fn get(&self, id: String) -> DbResult {
        transactions_orm.find(id).load::<Transaction>(self.conn)
    }

    pub fn get_all(&self) -> DbResult {
        transactions_orm
            .order(transactions::txid.desc())
            .load::<Transaction>(self.conn)
    }

    pub fn insert(&self, tx: Transaction) -> Result<usize, diesel::result::Error> {
        diesel::insert_into(transactions::table)
            .values(&tx)
            .execute(self.conn)
    }

    pub fn insert_all(&self, tx: Vec<Transaction>) -> Result<usize, diesel::result::Error> {
        diesel::insert_into(transactions::table)
            .values(&tx)
            .execute(self.conn)
    }
}
