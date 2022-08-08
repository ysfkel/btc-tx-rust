use serde::Deserialize;

use crate::service::models::{Transaction, Transactions};
use std::fmt::format;
use std::fs::{self, File};
use std::io::{self, BufRead};
use std::io::{BufReader, Read};

pub fn read_transactions() -> Result<Vec<Transaction>, io::Error> {
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
        let data = read_file(&f);
        read_file_2(&f);
        let t: Transactions = serde_json::from_str(data.as_str()).unwrap();
        tx = t.transactions;
        // std::fs::remove_file(f)?;
        println!("tx read {}", tx.len());
    }
    println!("read_transactions completed");

    Ok(tx)
}

fn read_file(filepath: &str) -> String {
    let file = File::open(filepath).expect("could not open file");
    let mut buffered_reader = BufReader::new(file);
    let mut contents = String::new();
    buffered_reader
        .read_to_string(&mut contents)
        .expect("could not read file into the string");
    contents
}
