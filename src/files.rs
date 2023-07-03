use std::{fs::File, process::exit, io::{BufReader, Read}};

pub fn load_file(path: &str) -> File {
    let result = File::open(path);
    if result.is_err() {
        println!("Error opening file: {}\n{}", path, result.unwrap_err());
        exit(1);
    }
    result.unwrap()
}

pub fn is_gzip(file: &File) -> bool {
    let mut buffer = [0; 2];
    let _ = BufReader::new(file).read_exact(&mut buffer);
    buffer == [0x1f, 0x8b]
}