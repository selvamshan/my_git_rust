use std::env;
// use std::fs;
// use std::io::Read;
// use flate2::read::ZlibDecoder;

use git_rust::init;
use git_rust::cat_file;
use git_rust::hash_object;

fn main() {
    
    let args: Vec<String> = env::args().collect();
    match args[1].as_str() {
        "init" => init(),
        "cat-file" => cat_file(&args[2..]),
        "hash-object" => hash_object(&args[2..]),

        _ => println!("unkown command: args[1]"),
    }
  
}
