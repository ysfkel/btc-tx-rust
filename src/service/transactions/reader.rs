use serde::Deserialize;

use crate::service::transactions::models::{Transaction, Transactions};
use std::env;
use std::fmt::format;
use std::fs::{self, File};
use std::io::{self, BufRead};
use std::io::{BufReader, Read};

pub fn read() -> Result<Vec<Transaction>, io::Error> {
    let dir = "files/tx";
    let mut tx: Vec<Transaction> = vec![];
    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();
        let file_name_os_str = path.file_name().expect("empty filename");
        let file_name = file_name_os_str
            .to_str()
            .expect("failed to concert os_str to str");
        let f = format!("{}/{}", dir, file_name);
        let data = btc_tx(&f);
        let mut t: Transactions = serde_json::from_str(data.as_str()).unwrap();
        tx.append(&mut t.transactions);
    }
    Ok(tx)
}

fn btc_tx(filepath: &str) -> String {
    let file = File::open(filepath).expect("could not open file");
    let mut buffered_reader = BufReader::new(file);
    let mut contents = String::new();
    buffered_reader
        .read_to_string(&mut contents)
        .expect("could not read file into the string");
    contents
}
