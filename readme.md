
- name: btc-tx-rust
- diesel setup 
- diesel migration generate create_books
- add sql to your migration files 
- then run the up sql migration 
- $ diesel migration run 
- run down sql migration 
- $ diesel migration redo

- generate schema from db 
- $diesel print-schema > scr/schema.rs

- we can also infer the schema from the database, by placeing this 
   in the schema.rs file or we just can just use the table! macro!
   infer_schema!("dotenv:DATABASE_URL")

   -- Your SQL goes here

CREATE TABLE transactions (
    txid VARCHAR PRIMARY KEY,
    account VARCHAR NOT NULL,
    address VARCHAR NOT NULL,
    category VARCHAR NOT NULL,
    amount DOUBLE PRECISION NOT NULL -- label VARCHAR NOT NULL,
    -- confirmations INT NOT NULL,
    -- blockhash VARCHAR NOT NULL,
    -- blockindex INT NOT NULL,
    -- blocktime BIGINT NOT NULL,
    -- vout INT NOT NULL,
    -- time BIGINT NOT NULL,
    -- timereceived BIGINT NOT NULL
)