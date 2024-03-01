use std::env;
use std::fs;


fn main() {
    
    let args: Vec<String> = env::args().collect();
    if args[1] == "init" {
        fs::create_dir(".git").unwrap();
        fs::create_dir(".git/objects").unwrap();
        fs::create_dir(".git/refs").unwrap();
        fs::write(".git/HEAD", "ref: refs/heads/master\n").unwrap();
        println!("Initialized git directory");  
    } else if args[1] == "cat-file" {
        if args[2] == "-p" {
            let hash = args[3].as_str();
            let folder_name = &hash[0..2];
            let file_name = &hash[2..];
            dbg!(folder_name, file_name);            
        }
         
    } else {
        println!("unkown command: args[1]");
    }
}
