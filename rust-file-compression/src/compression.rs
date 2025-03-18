use flate2::Compression;
use flate2::write::ZlibEncoder;
use std::io::prelude::*;

pub fn compress() {
    let mut e = ZlibEncoder::new(Vec::new(), Compression::default());
    let _ = e.write_all(b"foo");
    let _ = e.write_all(b"bar");
    let _compressed_bytes = e.finish();
}
