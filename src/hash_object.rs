use std::fs::{DirBuilder, File};
use std::io::{BufReader, BufWriter, Write};
use std::io::Read;
use std::path::Path;
use flate2::bufread::ZlibEncoder;
use flate2::Compression;
use hex::ToHex;
use sha1::{Digest, Sha1};

pub fn hash_object(args: &[String]) {
    match args[0].as_str() {
        "-w" => {
            let file_name = &args[1];
            let  file = std::fs::read(file_name).unwrap();
            let header = get_header(&file);
            let mut content = header.into_bytes();
            content.extend(&file);
            
            let sha = get_sha(&content);

            let folder_path = create_folder(&sha);
            let buf = compress(&file);
            let file_sha = get_file_sha(&sha);
            //let file_path = format!("{}/{}", &folder_path, file_sha);
            save_file(&buf, &folder_path, &file_sha);
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

fn save_file(buf :&[u8], folder_path: &str, file_sha:&str ) {   
    let path = format!("{}/{}", folder_path, file_sha);  
    let path = Path::new(&path);
    if path.exists() {
        return;
    }
    let f = File::create(path).expect("unable to create file");
    let mut f = BufWriter::new(f);
    f.write_all(buf).expect("unable to write data")   
}


fn print_sha(sha: &str) {
    print!("{sha}");
    let _ = std::io::stdout().flush();
}

fn get_file_sha<'a>(sha:&'a str) -> &'a str {
    &sha[2..]
}

fn get_header(content:&[u8]) -> String {
    let object_type = "blob";
    let size  = content.len();
    
    format!("{object_type} {size}\0")
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_file_sha() {
        let sha = "c4cf32e80d9a2cbb211f23f0f697b6e8b8e62131";
        let expected = "cf32e80d9a2cbb211f23f0f697b6e8b8e62131";
        let result = get_file_sha(&sha);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_create_blob_header() {
        let content = "what is up, doc?";
        let expected = "blob 16\0";
        let result = get_header(content.as_bytes());
        assert_eq!(result, expected);

    }
}