use std::{fs::File, io::Read, path::Path};



use crate::utils::{decompress, get_object_directory_name, get_object_file_name, index_of_next_null};



pub fn ls_tree(args: &[String]) {
    let _option = &args[0];
    let hash = &args[1];
    let directory_name = get_object_directory_name(hash);
    let file_name = get_object_file_name(hash);
    let path = Path::new(".git")
        .join("objects")
        .join(directory_name)
        .join(file_name);
    let mut file = File::open(path).unwrap();
    let mut content_buffer = vec![];    
    file.read_to_end(&mut content_buffer).unwrap();
    let bytes = decompress(&content_buffer);
    let header_object = HeadObject::new(&bytes);
    let header_index = header_object.get_header_index();
    let tree_object = TreeObject::new(&bytes, header_index);
    dbg!(header_object);
    dbg!(tree_object);

    // let mode_index = tree_object.get_mode_index();
    // let tree_object = TreeObject::new(&bytes, mode_index);
    // dbg!(&tree_object);


    
}

#[derive(Debug)]
struct HeadObject {    
    object_type: String,
    size: u32,
    index: usize,
}

impl HeadObject {
    pub fn new(bytes: &[u8]) -> Self {
        let index = index_of_next_null(&bytes, 0).expect("doesn't have header");
        let mut header_bytes = &bytes[0..index];
        let mut header = String::new();
        header_bytes.read_to_string(&mut header).unwrap();
        let mut header = header.trim().split_whitespace();
        let object_type = header.next().unwrap().to_owned();
        let size = header.next().unwrap().parse().unwrap();

        Self { object_type, size, index}
    }

    pub fn get_header_index(&self) -> usize {
        self.index
    }
}



#[derive(Debug)]
struct TreeObject { 
    mode: String,
    filename: String,
    index:usize
}



impl TreeObject {
    pub fn new(bytes: &[u8], header_null_index:usize) -> Self {
        let mode_null_index = index_of_next_null(&bytes, header_null_index + 1).expect("doesn't have mode");
        let mut mode_and_filename_bytes = &bytes[header_null_index +1..mode_null_index];
        let mut mode_and_filename = String::new();
        mode_and_filename_bytes.read_to_string(&mut mode_and_filename).unwrap();
        let mut mode_and_filename = mode_and_filename.trim().split_whitespace();
        let mode = mode_and_filename.next().unwrap().to_owned();
        let filename = mode_and_filename.next().unwrap().to_owned();       
        
        Self { mode, filename, index:mode_null_index}
    }

    pub fn get_mode_index(&self) -> usize {
        self.index
    }
}