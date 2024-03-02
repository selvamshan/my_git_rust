use std::fs::{DirBuilder, File};
use std::io::{BufReader, BufWriter, Write};
use std::io::Read;
use flate2::bufread::ZlibEncoder;
use flate2::Compression;
use hex::ToHex;
use sha1::{Digest, Sha1};

pub fn hash_object(args: &[String]) {
    match args[0].as_str() {
        "-w" => {
            let file_name = &args[1];
            let file = std::fs::read(file_name).unwrap();
            let sha = get_sha(&file);

            let folder_path = create_folder(&sha);
            let buf = compress(&file);
            let file_path = format!("{}/{}", &folder_path, &sha[2..]);
            save_file(&buf, &file_path);
            print_sha(&sha)

        },
        _ => println!("unknow option"),
    }
}


fn get_sha(file: &[u8]) -> String {
    let mut hasher = Sha1::new();
    hasher.update(file);
    hasher.finalize().encode_hex::<String>()
}

fn create_folder(sha: &str) -> String {
    let path = format!(".git/objects/{}", &sha[0..2]);
    DirBuilder::new().recursive(true).create(&path).unwrap();    
    path
}

fn compress(file: &[u8]) -> Vec<u8> {
    let b = BufReader::new(file);
    let mut buffer = Vec::new();
    let mut endcode = ZlibEncoder::new(b, Compression::default());    
    endcode.read_to_end(&mut buffer).unwrap();
    buffer   
}

fn save_file(buf :&[u8], path: &str ) {    
    let f = File::create(path).expect("unable to create file");
    let mut f = BufWriter::new(f);
    f.write_all(buf).expect("unable to write data")   
}


fn print_sha(sha: &str) {
    print!("{sha}");
    let _ = std::io::stdout().flush();
}