use std::collections::BinaryHeap;
use std::collections::HashSet;
use std::cmp::Reverse;
use crate::pkg::printer;
use crate::pkg::{printer::MsgLevel, util::{ByteFreq, bytes_freq_count}};

enum HuffmanTree {
    // The Leaf Node of the Huffman Tree
    Leaf(ByteFreq),
    // The SubTree of the Huffman Tree
    Tree {
        // Total Freqency of the Tree/Subtree
        total_freqency: u32,
        // Set of ByteFreq
        bytes: HashSet<ByteFreq>,
        // Subtree
        left: Box<HuffmanTree>,
        right: Box<HuffmanTree>,
    }
}

impl HuffmanTree {

    fn tree_builder(freq_heap: BinaryHeap<ByteFreq>) {
        match (freq_heap.pop(), freq_heap.pop()) {
            (Some(x), Some(y)) => {
                
            }
        }

    }

    fn encode(self) -> Vec<u8> {
        vec![0x0u8]
    }
}

pub fn compress(bytes: &Vec<u8>) -> Vec<u8> {
    // build frequency map from bytes vector
    let freq_map = bytes_freq_count(bytes);
    
    let mut freq_heap= BinaryHeap::new();
    // turn frequency map into Binary Heap
    for (&byte, &freq) in &freq_map {
        freq_heap.push(Reverse(ByteFreq {
            byte,
            freq,
        }))
    }
    HuffmanTree::from(freq_heap).encode()
}
