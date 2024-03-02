use std::io::Read;
use flate2::bufread::ZlibDecoder;



pub fn get_object_directory_name(hash:&str) -> String {
    hash[0..2].to_owned()
}

pub fn get_object_file_name(hash:&str) -> String {
    hash[2..].to_owned()
}

pub fn decompress(buffer:&[u8]) -> Vec<u8> {   
    let mut extracted_content =vec![]; 
    let mut decoder = ZlibDecoder::new(buffer);            
    decoder.read_to_end(&mut extracted_content).unwrap();
    extracted_content
}

pub fn index_of_next_null(bytes:&[u8], offest_index: usize) -> Option<usize> {
    //bytes.iter().position(|byte| *byte == b'\0')
    if let Some(index) = bytes
        .iter()
        .skip(offest_index)
        .position(|byte| *byte == b'\0') {
            Some(index + offest_index)
        } else {
            None
        }
}

#[cfg(test)] 
mod tests {
        use std::io::Write;
    use flate2::{write::ZlibEncoder, Compression};

    use super::*;

    #[test]
    fn test_get_object_directory_name() {
        let hash = "75c821c8c9c52ddbd1a14ee3fd0d10bca372";
        let expected = "75";
        let result = get_object_directory_name(hash);
        assert_eq!(expected, result)
    }

    #[test]
    fn test_get_object_file_name() {
        let hash = "75c821c8c9c52ddbd1a14ee3fd0d10bca372";
        let expected = "c821c8c9c52ddbd1a14ee3fd0d10bca372";
        let result = get_object_file_name(hash);
        assert_eq!(expected, result)
    }

    #[test]
    fn test_decompress() {
        let decompress_string = "75c821c8c9c52ddbd1a14ee3fd0d10bca372";       
        let mut encoder = ZlibEncoder::new(vec![], Compression::default());
        encoder.write_all(decompress_string.as_bytes()).unwrap();  
        let compressed = encoder.finish().unwrap();      
        let result = decompress(&compressed);
        assert_eq!(result, decompress_string.as_bytes());
    }

    #[test]
    fn test_index_of_next_null() {
        let s = "sarfse\0sfsfs\0";
        let expected_index:usize = 6;
        let result = index_of_next_null(&s.as_bytes(), 0);
        assert_eq!(result, Some(expected_index));
    }

}
