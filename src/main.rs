#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_codegen;
extern crate dotenv;
extern crate read_file;

#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate serde_json;

use dotenv::dotenv;
use std::fs::{self, File, ReadDir};
use std::io;

use diesel::{Connection, PgConnection};
use read_file::service::{models::Transaction, schema, tx_reader, tx_repository};
use std::env;

fn main() {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("set DATABASE_URL");
    let conn = PgConnection::establish(&database_url).unwrap();

    let transaction = Transaction {
        txid: String::from("id2"),
        account: String::from("test"),
        address: String::from("test"),
        category: String::from("test"),
        amount: 29299.,
        label: String::from("test"),
        confirmations: 2,
        blockhash: String::from("test"),
        blockindex: 2,
        blocktime: 3,
        vout: 2,
        time: 3,
        // user_id: "id".to_owned(),
        timereceived: 3,
    };

    let tx = tx_reader::read_transactions().unwrap();
    // for t in tx {
    //     println!("display:: {}", t.address);
    //     if Transaction::insert(t, &conn) {
    //         println!("successs")
    //     } else {
    //         println!("failed")
    //     }
    // }
    // if Transaction::insert(transaction, &conn) {
    //     print!("successs")
    // } else {
    //     print!("failed")
    // }
}
