use crc::{Crc, CRC_32_ISCSI};
// use crc::Algorithm;

//for read file
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
//read line by line exclude linefeed
// use std::io::BufReader;
// use std::fs;

fn main() {

    let var_inst = Crc::<u32>::new(&CRC_32_ISCSI);

    //one shot checksum
    assert_eq!(var_inst.checksum(b"123456789"), 0xe3069283);
    
    //iteration for final checksum
    let mut digest = var_inst.digest();
    digest.update(b"123456789");
    assert_eq!(digest.finalize(),0xe3069283);

    //caculate a full file checksum 
    let path = Path::new("Cargo.toml");
    let mut file = File::open(&path).unwrap();
    let mut s = String::new();
    file.read_to_string(&mut s).unwrap();
    let mut file_digest = var_inst.digest();
    file_digest.update(s.as_bytes());
    let checksum = file_digest.finalize();
    println!("checksum :0x{:x}",checksum);

    //caculate a very large binary file checksum 💣
    let mut file2_digest = var_inst.digest();
    let path2 = Path::new("case3-calcrc");
    let mut file2 = File::open(&path2).unwrap();
    let mut file_size:usize = 0;
    loop {
        let mut buffer = [0;100];
        let n = file2.read(&mut buffer[..]).unwrap();
        file2_digest.update(&buffer);
        file_size += n;
        if n < 100 {
            let file2_checksum = file2_digest.finalize();
            println!("file2_checksum(0x{:x}) :0x{:x}",file_size, file2_checksum);
            break;
        }
    }
    // let input2 = fs::read(&path2).unwrap();
    // for ele in input2.iter() {
    //     file2_digest.update(ele);
    // }

    // let file_eater = BufReader::new(file2);
    // let mut index = 0;
    // for line in file_eater.lines() {
    //     // file2_digest.update(line.unwrap().as_bytes());
    //     index += 1;
    //     println!("why {}: {:?}",index, line.unwrap().as_bytes());
    // }
    // let file2_checksum = file2_digest.finalize();
    // println!("file2_checksum :0x{:x}", file2_checksum);

}