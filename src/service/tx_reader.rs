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

/// This conversion can fail if the structure of the Value does not match the
/// structure expected by `T`, for example if `T` is a struct type but the Value
/// contains something other than a JSON map. It can also fail if the structure
/// is correct but `T`'s implementation of `Deserialize` decides that something
/// is wrong with the data, for example required struct fields are missing from
/// the JSON map or some number is too big to fit in the expected primitive
/// type.
// pub fn from_value<T>(value: Value) -> Result<T, Error>
// where
//     T: DeserializeOwned,
// {
//     T::deserialize(value)
// }

fn read_file(filepath: &str) -> String {
    let file = File::open(filepath).expect("could not open file");
    let mut buffered_reader = BufReader::new(file);
    let mut contents = String::new();
    buffered_reader
        .read_to_string(&mut contents)
        .expect("could not read file into the string");
    contents
}

#[derive(Debug, Deserialize, Clone)]
struct Test {
    pub name: String,
}
fn read_file_2(filepath: &str) {
    let file = File::open("files/test.json").expect("could not open file");

    // println!("file name 1 {:?}", file);
    let x: Vec<Test> = serde_json::from_reader(&file)
        .into_iter()
        .map(|f: Test| {
            println!("hello f {:?}", f);
            f
        })
        .collect();

    let d = serde_json::Deserializer::from_reader(file)
        .into_iter::<Test>()
        .filter_map(|f| {
            println!("hello f {:?}", f.ok());
            Option(Test {
                name: "y".to_owned(),
            })
        });
    //.collect::<Vec<Test>>();
    // .into_iter()
    // .filter_map(|t: Option<Test>| t)
    // .collect();
    // .filter(|t| true)
    // .map(|x| x)
    // .collect();

    // .expect("read failed");
    // .into_iter::<Test>()
    // .map(|t| t.ok())
    // .rev()
    // .collect();
    // .collect();
    //.filter_map(|t| t.ok())
    //.collect();

    println!("ss {:?}", x);
    for t in x {
        println!("my tx {:?}", t);
    }
}
