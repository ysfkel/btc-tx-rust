use serde::Deserialize;

use crate::service::users::models::User;
use std::fmt::format;
use std::fs::{self, File};
use std::io::{self, BufRead};
use std::io::{BufReader, Read};

pub fn read() -> Result<Vec<User>, io::Error> {
    let file_path = "files/users/users.json";
    let users: Vec<User> = vec![];

    let data = btc_tx(&file_path);
    let users: Vec<User> = serde_json::from_str(data.as_str()).unwrap();
    Ok(users)
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
