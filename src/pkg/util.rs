use std::collections::HashMap;

pub fn bytes_freq_count(bytes: &Vec<u8>) -> HashMap<u8, u32> {
    let mut hm = HashMap::new();
    for &byte in bytes.iter() {
       *hm.entry(byte).or_insert(0) += 1; 
    }
    hm
}

#[test]
fn test_bytes_freq_count() {
    let bytes = &Vec::from("aabbccddeefg".as_bytes());

    let mut expected = HashMap::new();
    expected.insert(b'a', 2);
    expected.insert(b'b', 2);
    expected.insert(b'c', 2);
    expected.insert(b'd', 2);
    expected.insert(b'e', 2);
    expected.insert(b'f', 1);
    expected.insert(b'g', 1);

    let test_result = bytes_freq_count(bytes);

    assert_eq!(test_result, expected)
}
