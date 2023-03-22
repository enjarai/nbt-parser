mod parser;
mod getters;

use std::env::args;
use std::fs::File;
use std::io::BufReader;
use flate2::read::GzDecoder;

fn main() {
    let file = args().nth(1).unwrap();
    let gzip = match args().nth(2) {
        Some(s) => s == "gzip",
        None => false
    };
    match File::open(file) {
        Ok(data) => {
            let time = std::time::Instant::now();

            let tag = if gzip {
                let mut reader = BufReader::new(GzDecoder::new(data));
                parser::Tag::parse(&mut reader).unwrap()
            } else {
                let mut reader = BufReader::new(data);
                parser::Tag::parse(&mut reader).unwrap()
            };

            println!("Took {}ms", time.elapsed().as_millis());
            println!("Parsed {} elements", tag.1);
            println!();
            // println!("{}", tag.0);
        },
        Err(e) => println!("{e}")
    }
}


