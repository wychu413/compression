use std::cmp::Ordering;
use std::collections::HashMap;
use std::hash;

pub struct ByteFreq {
    pub byte: u8,
    pub freq: u32,
}

impl PartialEq for ByteFreq {
    fn eq(&self, other: &Self) -> bool {
        self.freq == other.freq
    }
}

impl Eq for ByteFreq {}

impl PartialOrd for ByteFreq {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.freq.partial_cmp(&other.freq)
    }
}

impl Ord for ByteFreq {
    fn cmp(&self, other: &Self) -> Ordering {
        self.freq.cmp(&other.freq)
    }
}

pub fn bytes_freq_count(bytes: &Vec<u8>) -> HashMap<u8, u32> {
    let mut hm = HashMap::new();
    for &byte in bytes.iter() {
       *hm.entry(byte).or_insert(0) += 1; 
    }
    hm
}
