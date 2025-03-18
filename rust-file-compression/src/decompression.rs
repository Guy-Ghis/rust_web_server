use std::io::prelude::*;
use flate2::read::GzDecoder;

pub fn decompress() {
    let mut d = GzDecoder::new("...".as_bytes());
    let mut s = String::new();
    d.read_to_string(&mut s).unwrap();
    println!("{}", s);
}