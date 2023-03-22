mod parser;

use std::env::args;
use std::fs::File;
use std::io::{BufRead, BufReader, Read};

fn main() {
    let file = args().nth(1).unwrap();
    match File::open(file) {
        Ok(data) => {
            let mut reader = BufReader::new(data);
            let tag = parser::Tag::parse(&mut reader).unwrap();
            println!("{:?}", tag);
        },
        Err(e) => println!("{e}")
    }
}


