mod nbt;
mod files;

use std::env::args;
use std::io::{BufReader, BufWriter};
use flate2::Compression;
use flate2::read::{GzDecoder, GzEncoder};
use crate::files::*;
use crate::nbt::tag::Tag;

fn main() {
    let path = args().nth(1).unwrap();
    let file = load_file(&path);
    let gzip = is_gzip(&file);

    let tag = if gzip {
        let mut reader = BufReader::new(GzDecoder::new(&file));
        Tag::read(&mut reader).unwrap()
    } else {
        let mut reader = BufReader::new(&file);
        Tag::read(&mut reader).unwrap()
    };

    let mut count = 0;
    tag.count_elements(&mut count);
    println!("Parsed {} elements", count);
    println!();
    // println!("{}", tag);

    let save = || {
        if gzip {
            let mut writer = BufWriter::new(
                GzEncoder::new(&file, Compression::default()));
            tag.write(&mut writer).unwrap();
        } else {
            let mut writer = BufWriter::new(&file);
            tag.write(&mut writer).unwrap();
        }
    };
}


