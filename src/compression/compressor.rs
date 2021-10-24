use std::process::exit;

use super::super::printer;
use super::super::MsgLevel;

mod huffman;

pub struct Compressor<F>
where
    F: Fn(&str) -> String,
{
    compress: F,
}

impl<F> Compressor<F> 
where 
    F: Fn(&str) -> String,
{
    pub fn new(alg: &str) -> Compressor<impl Fn(&str) -> String> {
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


