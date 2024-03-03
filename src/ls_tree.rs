use std::{fs::File, io::Read, path::Path};



use crate::utils::{decompress, get_num_chunks, get_object_directory_name, get_object_file_name, next_chunck};



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
    let num_chunks = get_num_chunks(&bytes);
    //println!("{}",num_chunks);
    // let header_chunck = next_chunck(&bytes, 0).unwrap();
    // let header_object = HeadObject::new(header_chunck);   
    // //dbg!(header_object);

    let node_chunk = next_chunck(&bytes, 1).unwrap();
    let tree_object = TreeObject::new(node_chunk);
    println!("{}",tree_object.filename);
  
    for i in 2..num_chunks-1 {
        let node_chunk = next_chunck(&bytes, i).unwrap();
        let chunck = node_chunk.iter().skip(20).map(|x| *x).collect::<Vec<u8>>();
        let tree_object = TreeObject::new(&chunck);
        println!("{}",tree_object.filename);
    }
  

    
}


// #[allow(dead_code)]
// #[derive(Debug)]
// struct HeadObject {    
//     object_type: String,
//     size: u32,
// }

// impl HeadObject {
//     pub fn new(bytes: &[u8]) -> Self {       
//         let mut header_bytes = bytes;
//         let mut header = String::new();
//         header_bytes.read_to_string(&mut header).unwrap();
//         let mut header = header.trim().split_whitespace();
//         let object_type = header.next().unwrap().to_owned();
//         let size = header.next().unwrap().parse().unwrap();

//         Self { object_type, size}
//     }
    
// }



#[allow(dead_code)]
#[derive(Debug)]
struct TreeObject { 
    mode: String,
    filename: String,   
}



impl TreeObject {
    pub fn new(bytes: &[u8]) -> Self {       
        let mut mode_and_filename_bytes = bytes;
        let mut mode_and_filename = String::new();
        mode_and_filename_bytes.read_to_string(&mut mode_and_filename).unwrap();
        let mut mode_and_filename = mode_and_filename.trim().split_whitespace();
        let mode = mode_and_filename.next().unwrap().to_owned();
        let filename = mode_and_filename.next().unwrap().to_owned();       
        
        Self { mode, filename}
    }

 
}