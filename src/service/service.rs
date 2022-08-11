use super::transactions::repository::Repository;
use super::users::repository::Repository as UserRepository;
use super::{transactions::models::Transaction, users::models::User};
pub struct Service<'a> {
    repo: &'a Repository<'a>,
}

type ServiceResult = Result<usize, String>;

impl<'a> Service<'a> {
    pub fn new(repo: &'a Repository) -> Self {
        Self { repo }
    }

    pub fn add_transactions(&self, transactions: Vec<Transaction>) -> ServiceResult {
        let mut valid_tx: Vec<Transaction> = vec![];

        for tx in transactions {
            let result = self.repo.get(tx.txid.clone());

            let is_new_record = match result {
                Ok(r) => r.len() == 0,
                Err(e) => false,
            };

            if is_valid_transaction(tx.confirmations, tx.txid.as_str(), &valid_tx) && is_new_record
            {
                valid_tx.push(tx);
            }
        }

        self.repo.insert_all(valid_tx).map_err(|e| e.to_string())
    }
}

fn is_valid_transaction(
    confirmations: i32,
    transaction_id: &str,
    valid_tx: &Vec<Transaction>,
) -> bool {
    confirmations >= 6 && !valid_tx.iter().any(|tx| tx.txid == transaction_id)
}
