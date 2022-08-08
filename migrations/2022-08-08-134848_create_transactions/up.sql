-- Your SQL goes here

CREATE TABLE transactions (
    txid VARCHAR PRIMARY KEY,
    account VARCHAR NOT NULL,
    address VARCHAR NOT NULL,
    category VARCHAR NOT NULL,
    amount DOUBLE PRECISION NOT NULL,
    label VARCHAR NOT NULL,
    confirmations INT NOT NULL,
    blockhash VARCHAR NOT NULL,
    blockindex INT NOT NULL,
    blocktime BIGINT NOT NULL,
    vout INT NOT NULL,
    time BIGINT NOT NULL,
    timereceived BIGINT NOT NULL
);

CREATE TABLE users (
    id VARCHAR PRIMARY KEY,
    first_name VARCHAR NOT NULL,
    last_name VARCHAR NOT NULL
);