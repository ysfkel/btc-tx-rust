extern crate btc_tx;
extern crate dotenv;

use btc_tx::db;
use btc_tx::service::service::Service;
use btc_tx::service::transactions::reader as tx_reader;
use btc_tx::service::transactions::repository::Repository as TransactionRepository;
use btc_tx::service::users;
use btc_tx::service::users::repository::Repository as UserRepository;
use diesel::{Connection, PgConnection};
use dotenv::dotenv;
mod process_transactions;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let database_url = std::env::var("POSTGRES_URL").expect("set POSTGRES_URL");
    // run database migrations
    db::migration::run_migrations(&database_url)
        .await
        .expect("postgress migrations error");

    let conn = PgConnection::establish(&database_url)
        .expect("AppError: failed to establish database connecion");

    // Read JSON files
    let transactions = tx_reader::read().expect("failed to read transactions");
    let users_list = users::reader::read().expect("failed to read users");

    // save users to database
    let user_repo = UserRepository::new(&conn);
    user_repo
        .insert_all(users_list.clone())
        .expect("failed to insert users");

    // initialize service
    let tx_repo = TransactionRepository::new(&conn);
    let tx_service = Service::new(&tx_repo);
    //save transactions
    tx_service
        .add_transactions(transactions.clone())
        .expect("failed to insert transactions");
    // print transactions
    process_transactions::get_transactions_aggregate(transactions, users_list);
}
