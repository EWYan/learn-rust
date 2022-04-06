use std::char;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

fn main() {
    //read file content from sample.txt
    // Create a path to the desired file
    let path = Path::new("sample.txt");

    // Open the path in read-only mode, returns `io::Result<File>`
    let mut file = File::open(&path).unwrap();

    // Read the file contents into a string, returns `io::Result<usize>`
    let mut s = String::new();
    file.read_to_string(&mut s).unwrap();

    let mut a: Vec<char> = Vec::new();
    //write to a new file and turn character "人" to "😻"
    let _dum: Vec<char> = s.chars().map(|c| {
                                if c == '人' {
                                    a.push('😻');
                                    c
                                } else {
                                    a.push(c);
                                    c
                                }
                            }).collect();
                            
    let a_string: String = a.iter().collect::<String>();
    let path = Path::new("out.txt");
    let mut file_w = File::create(&path).unwrap();
    file_w.write_all(a_string.as_bytes()).unwrap();

}


