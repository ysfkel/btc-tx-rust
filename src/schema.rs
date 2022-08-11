table! {
    transactions (txid) {
        txid -> Varchar,
        account -> Varchar,
        address -> Varchar,
        category -> Varchar,
        amount -> Float8,
        label -> Varchar,
        confirmations -> Int4,
        blockhash -> Varchar,
        blockindex -> Int4,
        blocktime -> Int8,
        vout -> Int4,
        time -> Int8,
        timereceived -> Int8,
    }
}

table! {
    users (id) {
        id -> Varchar,
        name -> Varchar,
    }
}

allow_tables_to_appear_in_same_query!(transactions, users,);
