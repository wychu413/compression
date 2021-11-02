use std::collections::{BinaryHeap, HashMap};
use std::cmp::{Reverse, Ordering};
use bit_vec::BitVec;
use crate::pkg::util::bytes_freq_count;

/// Cannonical Huffman code is being used
/// to compress and decompress information

impl PartialEq for HuffmanTree {
    fn eq(&self, other: &Self) -> bool {

        let freq1 = match self {
            &Self::Leaf {freq, symbol: _} => freq,
            &Self::Tree {freq, left: _, right: _} => freq,
        };

        let freq2 = match other {
            &Self::Leaf {freq, symbol: _} => freq,
            &Self::Tree {freq, left: _, right: _} => freq, 
        };

        freq1 == freq2
    }
}

impl Eq for HuffmanTree {}

impl PartialOrd for HuffmanTree {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {

        let freq1 = match self {
            &Self::Leaf {freq, symbol: _} => freq,
            &Self::Tree {freq, left: _, right: _} => freq,
        };

        let freq2 = match other {
            &Self::Leaf {freq, symbol: _} => freq,
            &Self::Tree {freq, left: _, right: _} => freq, 
        };

        freq1.partial_cmp(&freq2)
    }
}

impl Ord for HuffmanTree {
    fn cmp(&self, other: &Self) -> Ordering {

        let freq1 = match self {
            &Self::Leaf {freq, symbol: _} => freq,
            &Self::Tree {freq, left: _, right: _} => freq,
        };

        let freq2 = match other {
            &Self::Leaf {freq, symbol: _} => freq,
            &Self::Tree {freq, left: _, right: _} => freq, 
        };

        freq1.cmp(&freq2)
    }
}

#[derive(Debug)]
enum HuffmanTree {
    // The Leaf Node of the Huffman Tree
    Leaf {
        freq: u32,
        symbol: u8,
    },

    // The SubTree of the Huffman Tree
    Tree {
        freq: u32,
        // Left SubTree
        left: Box<HuffmanTree>,
        // Right SubTree
        right: Box<HuffmanTree>,
    }
}

impl HuffmanTree {
    fn get_node_frequency(&self) -> u32 {
        match self {
            &Self::Leaf { freq, symbol: _ } => freq,
            &Self::Tree { freq, left: _, right: _ } => freq
        }
    }

    fn make_canonical_huffman_code_table(&self) 
        -> HashMap<u8, Vec<bool>>
    {
        // First construct the huffman code table with
        // the symbol as the key and the length of huffman code
        // as value
        let map = HashMap::<u8, u32>::new();
        // map

        HashMap::<u8, Vec<bool>>::new()
    }
}

fn make_tree(mut heap: BinaryHeap<Reverse<HuffmanTree>>) -> HuffmanTree {
    
    // since tail call optimization is not supported by Rust
    // Implementation of huffman tree construction has changed
    // to Iteration with while-loop

    // if heap.len() <= 1 {
    //     let Reverse(tree) = heap.pop().unwrap();
    //     return tree
    // }

    // let Reverse(item1) = heap.pop().unwrap();
    // let Reverse(item2) = heap.pop().unwrap();

    // heap.push(Reverse(HuffmanTree::Tree {
    //     freq: item1.get_node_frequency() + item2.get_node_frequency(),
    //     left: Box::new(item1),
    //     right: Box::new(item2),
    // }));
    // make_tree(heap)

    while heap.len() > 1 {
        let Reverse(item1) = heap.pop().unwrap();
        let Reverse(item2) = heap.pop().unwrap();

        heap.push(Reverse(HuffmanTree::Tree {
            freq: item1.get_node_frequency() + item2.get_node_frequency(),
            left: Box::new(item1),
            right: Box::new(item2),
        }));
    }

    let Reverse(tree) = heap.pop().unwrap();
    tree
}

pub fn compress(bytes: &Vec<u8>) -> Vec<u8> {
    // build frequency map from bytes vector
    let map = bytes_freq_count(bytes);
    
    let mut heap = BinaryHeap::new();

    // insert the Huffman Tree into binary heap
    for (&byte, &freq) in map.iter() {
        heap.push(Reverse(HuffmanTree::Leaf {
            symbol: byte,
            freq
        }));
    }
    let tree = make_tree(heap);
    
    println!("{:?}", tree);
    vec![0]
}

#[test]
fn test_make_tree() {
    let s = "aaaabbccdeeeeeeeeee";
    let map = bytes_freq_count(&Vec::from(s.as_bytes()));

    // since the heap is not consistent in every tests, It is going to test
    // the ultimate Huffman tree total frequency count instead of checking
    // the entire tree structure.
    // The problem should be occured when two keys (frequency) are having 
    // the same value.

    let mut heap = BinaryHeap::new();
    for (&byte, &freq) in map.iter() {
        heap.push(Reverse(HuffmanTree::Leaf {
            symbol: byte,
            freq
        }));
    }
    let test_result = make_tree(heap);

    assert_eq!(test_result.get_node_frequency(), 19)
}
