use std::env;
// use std::fs;
// use std::io::Read;
// use flate2::read::ZlibDecoder;

use git_rust::init;
use git_rust::cat_file;
use git_rust::hash_object;
use git_rust::ls_tree;

fn main() {
    
    let args: Vec<String> = env::args().collect();
    let rest_of_args = &args[2..];
    match args[1].as_str() {
        "init" => init(),
        "cat-file" => cat_file(rest_of_args),
        "hash-object" => hash_object(rest_of_args),
        "ls-tree" => ls_tree(rest_of_args),

        _ => println!("unkown command: args[1]"),
    }
  
}
