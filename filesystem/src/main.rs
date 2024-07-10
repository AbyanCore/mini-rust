use std::fs::File;
use std::io::{BufRead, BufReader, Write};

fn main() {
    let path = "src/bre.txt";

    match File::open(path) {
        Ok(i) => {
            let buffer = BufReader::new(i);

            for line in buffer.lines() {
                print!("{}",line.unwrap());
            }
        },
        Err(e) => {
            println!("{e}");
            write!(File::create(path).unwrap(),"");
        },
    }
}
