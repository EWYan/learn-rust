extern crate sha2;

use sha2::{Sha256, Digest};
use std::{fs, io};
use std::path::Path;
use base64ct::{Base64, Encoding};

fn main() {
    let path = Path::new("./Cargo.toml");
    let mut file = fs::File::open(&path).unwrap();
    let mut hasher = Sha256::new();
    let n = io::copy(&mut file, &mut hasher).unwrap();
    let hash = hasher.finalize();
    println!("binary hash({:x?}) :{:x?}", n,hash);
    let base64_hash = Base64::encode_string(&hash);
    println!("Base64-encoded hash: {:?}", base64_hash);

}
/* 
output:
    binary hash(f2) :[60, bc, 31, 98, 71, 44, 87, d1, 13, f5, 0, 53, 67, d4, 7c, 19, 56, d2, 7f, 83, 69, 3b, 22, 82, 59, fe, 2d, 42, e4, 9b, ee, 86]
    Base64-encoded hash: "YLwxmHFEh9ET9QBTZ9R8GVbSf4NpOyKCWf4tQuSb7oY="
 */