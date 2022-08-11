use btc_tx::service::transactions::models::Transaction;
use btc_tx::service::users::models::User;
use std::collections::HashMap;
struct TransactionData<'a> {
    address: &'a str,
    name: String,
    amount: f64,
    transactions_count: i32,
}

pub fn get_transactions_aggregate(mut transactions: Vec<Transaction>, users: Vec<User>) {
    let mut users_map: HashMap<String, TransactionData> = HashMap::new();

    transactions.sort_by(|a, b| a.amount.partial_cmp(&b.amount).unwrap());

    let smallest_amount = transactions.get(0).unwrap().amount;
    let largest_amount = transactions.get(transactions.len() - 1).unwrap().amount;
    let mut total_deposits_without_ref = 0.;
    let mut total_deposits_without_ref_count = 0;

    for t in transactions.iter() {
        // calculate total deposits for users without ref & count them
        let is_known_user = users.iter().any(|a| a.id == t.address);
        if !is_known_user {
            total_deposits_without_ref += t.amount;
            total_deposits_without_ref_count += 1;
        }

        // if the user was previously added to the hashmap, return his current sum amount and tx count
        let (amount_sum, transactions_count) = match users_map.get(&t.address.clone()) {
            Some(r) => (r.amount, r.transactions_count + 1),
            None => (0., 1),
        };

        if is_known_user {
            // get user name
            let user_name = users
                .iter()
                .find(|a| a.id == t.address)
                .unwrap()
                .name
                .clone();
            // add to map
            users_map.insert(
                t.address.clone(),
                TransactionData {
                    address: t.address.as_str(),
                    name: user_name,
                    amount: t.amount + amount_sum,
                    transactions_count,
                },
            );
        }
    }

    print_result(
        users_map,
        users,
        total_deposits_without_ref,
        total_deposits_without_ref_count,
        largest_amount,
        smallest_amount,
    );
}

// prints results
fn print_result(
    data: HashMap<String, TransactionData>,
    users: Vec<User>,
    total_deposits_without_ref: f64,
    total_deposits_without_ref_count: i32,
    largest_amount: f64,
    smallest_amount: f64,
) {
    let transactions_data: Vec<&TransactionData> = data.values().collect();

    for user in users.iter() {
        let _user = transactions_data
            .iter()
            .into_iter()
            .find(|a| a.address == user.id);

        if let Some(u) = _user {
            println!(
                "Deposited for {}: count={} sum={}",
                u.name, u.transactions_count, u.amount
            )
        }
    }

    println!(
        "Deposited without reference: count={} sum={}",
        total_deposits_without_ref_count, total_deposits_without_ref
    );
    println!("Smallest valid deposit: {}", smallest_amount);
    println!("Largest valid deposit: {}", largest_amount);
}
