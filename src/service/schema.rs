table! {
    users(id) {
        id -> VarChar,
        first_name -> Varchar,
        last_name -> Varchar,
    }
}

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
        // userid -> Int4,
    }
}
