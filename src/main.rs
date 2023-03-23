mod read;
mod tag;
mod write;

use std::env::args;
use std::fs::File;
use std::io::{BufReader, BufWriter};
use flate2::Compression;
use flate2::read::{GzDecoder, GzEncoder};
use crate::tag::Tag;

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
                Tag::read(&mut reader).unwrap()
            } else {
                let mut reader = BufReader::new(data);
                Tag::read(&mut reader).unwrap()
            };

            println!("Reading: Took {}ms", time.elapsed().as_millis());

            let mut count = 0;
            tag.count_elements(&mut count);
            println!("Parsed {} elements", count);
            println!();
            // println!("{}", tag);

            let time = std::time::Instant::now();

            let file = File::create("test.nbt").unwrap();
            if gzip {
                let mut writer = BufWriter::new(
                    GzEncoder::new(file, Compression::default()));
                tag.write(&mut writer).unwrap();
            } else {
                let mut writer = BufWriter::new(file);
                tag.write(&mut writer).unwrap();
            }

            println!("Writing: Took {}ms", time.elapsed().as_millis());
        },
        Err(e) => println!("{e}")
    }
}


