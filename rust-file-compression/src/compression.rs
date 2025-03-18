use flate2::Compression;
use flate2::write::ZlibEncoder;
use std::io::prelude::*;

pub fn compress() {
    let mut e = ZlibEncoder::new(Vec::new(), Compression::default());
    e.write_all(b"foo");
    e.write_all(b"bar");
    let compressed_bytes = e.finish();
}
