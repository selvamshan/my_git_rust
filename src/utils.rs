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

pub fn next_chunck<'a>(bytes:&'a[u8], offest_nulls: usize) -> Option<&'a [u8]> {
    let mut result = bytes.split(
        |byte|  *byte == b'\0');
    
        result.nth(offest_nulls)   
}

pub fn get_num_chunks(bytes: &[u8]) -> usize {
    bytes.split(|byte|  *byte == b'\0').collect::<Vec<_>>().len()
}

// pub fn get_all_chunks(bytes: &[u8]) -> Vec<&[u8]> {
//     bytes.split(|byte|  *byte == b'\0').collect::<Vec<_>>()
// }

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
    fn test_first_next_chunk() {
        let s = "sarfse\0sfsfs\0";
        let expected_value = "sarfse".as_bytes();
        let result = next_chunck(&s.as_bytes(), 0);
        assert_eq!(result, Some(expected_value));
    }

    #[test]
    fn test_second_next_chunk() {
        let s = "sarfse\0sfsfs";
        let expected_value = "sfsfs".as_bytes();
        let result = next_chunck(&s.as_bytes(), 1);
        assert_eq!(result, Some(expected_value));
    }

    #[test]
    fn test_get_num_chunks() {
        let s = "sarfse\0sfsfs";
        let expected_value:usize = 2;
        let result = get_num_chunks(&s.as_bytes());
        assert_eq!(result, expected_value);

    }

    // #[test]
    // fn test_all_chunks() {
    //     let s = "sarfse\0sfsfs";
    //     let first_expected_value = "sfsfs".as_bytes();
    //     let result = get_all_chunks(&s.as_bytes());
    //     assert_eq!(result[0], first_expected_value)
    // }

}
