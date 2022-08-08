#[derive(Debug, Queryable, Associations, Insertable, Serialize, Deserialize)]
// #[belongs_to(User, foreign_key = "address")]
#[table_name = "transactions"]
pub struct Transaction {
    // involvesWatchonly: bool,
    // #[serde(alias = "id")]
    pub txid: String,
    pub account: String,
    pub address: String,
    pub category: String,
    pub amount: f64,
    pub label: String,
    pub confirmations: i32,
    pub blockhash: String,
    pub blockindex: i32,
    pub blocktime: i64,
    pub vout: i32,
    pub time: i64,
    pub timereceived: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Transactions {
    pub transactions: Vec<Transaction>,
}