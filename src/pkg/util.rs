use std::{collections::HashMap};

pub fn character_freq_counter(plain_text: &str) -> HashMap<char, u32> {
    let mut hm = HashMap::new();
    for ch in plain_text.chars() {
       *hm.entry(ch).or_insert(0) += 1; 
    }
    hm
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use crate::pkg::util::character_freq_counter;

    #[test]
    fn test_character_freq_counter() {
        let mut expected = HashMap::new();
        expected.insert('a', 2); 
        expected.insert('b', 2); 
        expected.insert('c', 2); 
        expected.insert('d', 2); 
        expected.insert('e', 2); 
        expected.insert('g', 1); 
        assert_eq!(expected, character_freq_counter("aabbccddeeg"));
    }
}
