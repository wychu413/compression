use std::process::exit;
use crate::pkg::printer;
use crate::pkg::printer::MsgLevel;

use crate::compressor::huffman;

pub struct Compressor
{
    compress: fn(&str) -> String,
}

impl Compressor {
    pub fn new(alg: &str) -> Compressor {
        Compressor {
           compress: match alg {
                "huffman" => huffman::compress,
                _ => {
                    printer::print(MsgLevel::Error, "unsupported compression algorithm");
                    exit(1);
                }
            }
        }
    }

    pub fn run(self, text: &str) -> String {
        (self.compress)(text)
    }
}

#[test]
fn test_huffman() {
    let data = "aabbccddee";
    let c = Compressor::new("huffman");
    let result = c.run(data);
    assert_eq!(result, String::from(""));
}