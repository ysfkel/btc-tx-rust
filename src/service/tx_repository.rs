use crate::service::models::Transaction;
use crate::service::schema::transactions;
use crate::service::schema::transactions::dsl::transactions as transactions_orm;
use diesel::pg::PgConnection;
use diesel::prelude::*;

impl Transaction {
    pub fn get(id: String, conn: &PgConnection) -> Vec<Transaction> {
        transactions_orm
            .find(id)
            .load::<Transaction>(conn) // cast to book struct
            .expect("Error loading book")
    }

    pub fn get_all(conn: &PgConnection) -> Vec<Transaction> {
        transactions_orm
            .order(transactions::txid.desc())
            .load::<Transaction>(conn)
            .expect("error loading books")
    }

    // pub fn update_by_id(id: i32, conn: &PgConnection, book: NewBook) -> bool {
    //     use super::schema::transactions::dsl::{author as a, published as p, title as t};
    //     let NewBook {
    //         title,
    //         author,
    //         published,
    //     } = book;

    //     diesel::update(all_books.find(id))
    //         .set((a.eq(author), p.eq(published), t.eq(title)))
    //         .get_result::<Book>(conn)
    //         .is_ok()
    // }

    pub fn insert(tx: Transaction, conn: &PgConnection) -> bool {
        diesel::insert_into(transactions::table)
            .values(&tx)
            .execute(conn)
            .is_ok()
    }

    // pub fn delete_by_id(id: i32, conn: &PgConnection) -> bool {
    //     if Book::get(id, conn).is_empty() {
    //         return false;
    //     };
    //     diesel::delete(all_books.find(id)).execute(conn).is_ok()
    // }

    // pub fn get_by_author(author: &str, conn: &PgConnection) -> Vec<Book> {
    //     all_books
    //         .filter(books::author.eq(author))
    //         .load::<Book>(conn)
    //         .expect("Error loading books by author")
    // }
}
