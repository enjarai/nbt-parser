mod parser;

use std::env::args;
use std::fs::read;

fn main() {
    let file = args().nth(1);
    match read(file) {
        Ok(data) => {

        },
        Err(e) => println!("{e}")
    }
}


