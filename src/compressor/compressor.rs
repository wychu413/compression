use std::process::exit;
use crate::pkg::printer;
use crate::pkg::printer::MsgLevel;

use crate::compressor::huffman;

pub struct Compressor
{
    compress: fn(&Vec<u8>) -> Vec<u8>,

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

    pub fn run(self, bytes: &Vec<u8>) -> Vec<u8> {
        (self.compress)(bytes)
    }
}
